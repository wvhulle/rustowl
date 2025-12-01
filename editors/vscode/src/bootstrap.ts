import fs from "node:fs/promises";
import path from "node:path";
import os from "node:os";
import { spawn, spawnSync } from "node:child_process";
import * as vscode from "vscode";
import packageJson from "../package.json";

const version: string = packageJson.version;

const REPO_URL = "https://github.com/wvhulle/rustowl.git";
const CACHE_DIR = path.join(os.homedir(), ".cache", "rustowl");
const CARGO_BIN = path.join(os.homedir(), ".cargo", "bin");
const EXE_EXT = process.platform === "win32" ? ".exe" : "";

interface RustowlConfig {
  readonly serverPath: string;
}

const getConfig = (): RustowlConfig => ({
  serverPath: vscode.workspace.getConfiguration("rustowl").get<string>("serverPath", ""),
});

const getVersionOutput = (command: string, args: string[]): string => {
  const result = spawnSync(command, args);
  // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition, @typescript-eslint/strict-boolean-expressions
  return result.stdout ? String(result.stdout).trim() : "";
};

const commandExists = (command: string): boolean =>
  spawnSync(command, ["--version"]).status === 0;

const exists = async (filePath: string): Promise<boolean> =>
  fs.access(filePath).then(() => true).catch(() => false);

const isGitRepo = async (dir: string): Promise<boolean> =>
  exists(path.join(dir, ".git"));

const waitForProcess = (proc: ReturnType<typeof spawn>, name: string): Promise<void> =>
  new Promise((resolve, reject) => {
    proc.on("close", (code) =>
      code === 0 ? resolve() : reject(new Error(`${name} failed with code ${code ?? "unknown"}`))
    );
    proc.on("error", reject);
  });

export const needsUpdate = async (currentVersion: string): Promise<boolean> => {
  if (!currentVersion) {return true;}
  
  console.warn(`Current RustOwl version: ${currentVersion}`);
  console.warn(`Extension version: v${version}`);
  
  try {
    const semverParser = await import("semver-parser");
    const current = semverParser.parseSemVer(currentVersion, false);
    const target = semverParser.parseSemVer(version, false);
    return !(
      current.major === target.major &&
      current.minor === target.minor &&
      current.patch === target.patch &&
      JSON.stringify(current.pre) === JSON.stringify(target.pre)
    );
  } catch {
    return true;
  }
};

const tryBinstall = async (): Promise<boolean> =>
  vscode.window.withProgress(
    { location: vscode.ProgressLocation.Notification, title: "RustOwl: Trying cargo-binstall..." },
    async (progress) => {
      if (!commandExists("cargo-binstall")) {
        progress.report({ message: "cargo-binstall not found, skipping" });
        return false;
      }
      
      try {
        progress.report({ message: "Installing via cargo-binstall..." });
        const proc = spawn("cargo-binstall", ["--no-confirm", "rustowl"], { stdio: "pipe" });
        await waitForProcess(proc, "cargo-binstall");
        return true;
      } catch {
        progress.report({ message: "cargo-binstall failed" });
        return false;
      }
    }
  );

const cloneOrPullRepo = async (
  progress: vscode.Progress<{ message?: string }>
): Promise<void> => {
  await fs.mkdir(CACHE_DIR, { recursive: true });
  
  if (await isGitRepo(CACHE_DIR)) {
    progress.report({ message: "Pulling latest changes..." });
    const pull = spawn("git", ["pull", "--ff-only"], { cwd: CACHE_DIR });
    try {
      await waitForProcess(pull, "git pull");
    } catch {
      progress.report({ message: "Pull failed, re-cloning..." });
      await fs.rm(CACHE_DIR, { recursive: true, force: true });
      await fs.mkdir(CACHE_DIR, { recursive: true });
      const clone = spawn("git", ["clone", "--depth", "1", REPO_URL, CACHE_DIR]);
      await waitForProcess(clone, "git clone");
    }
  } else {
    progress.report({ message: "Cloning repository..." });
    await fs.rm(CACHE_DIR, { recursive: true, force: true });
    await fs.mkdir(CACHE_DIR, { recursive: true });
    const clone = spawn("git", ["clone", "--depth", "1", REPO_URL, CACHE_DIR]);
    await waitForProcess(clone, "git clone");
  }
};

const buildFromSource = async (): Promise<boolean> =>
  vscode.window.withProgress(
    {
      location: vscode.ProgressLocation.Notification,
      title: "RustOwl: Building from source",
      cancellable: false,
    },
    async (progress) => {
      try {
        await cloneOrPullRepo(progress);
        
        progress.report({ message: "Running cargo install (this may take a few minutes)..." });
        
        const cargoInstall = spawn("cargo", ["install", "--path", ".", "--locked"], {
          cwd: CACHE_DIR,
        });

        cargoInstall.stderr.on("data", (data: Buffer) => {
          const line = String(data).trim();
          if (line.includes("Compiling")) {
            progress.report({ message: line });
          }
        });

        await waitForProcess(cargoInstall, "cargo install");
        
        progress.report({ message: "Installation complete" });
        return true;
      } catch (e) {
        console.error("Build from source failed:", e);
        return false;
      }
    }
  );

const createSymlink = async (binaryPath: string): Promise<void> => {
  const symlinkPath = path.join(CARGO_BIN, `rustowl${EXE_EXT}`);
  
  if (await exists(symlinkPath)) {
    const stat = await fs.lstat(symlinkPath);
    if (stat.isSymbolicLink() || stat.isFile()) {
      await fs.unlink(symlinkPath);
    }
  }
  
  try {
    await fs.symlink(binaryPath, symlinkPath);
    console.warn(`Created symlink: ${symlinkPath} -> ${binaryPath}`);
  } catch (e) {
    console.warn(`Could not create symlink: ${String(e)}`);
  }
};

const findRustowlBinary = async (): Promise<string | null> => {
  const locations = [
    path.join(CARGO_BIN, `rustowl${EXE_EXT}`),
    path.join(CACHE_DIR, "target", "release", `rustowl${EXE_EXT}`),
  ];
  
  for (const loc of locations) {
    if (await exists(loc)) {
      const ver = getVersionOutput(loc, ["--version", "--quiet"]);
      if (ver) {return loc;}
    }
  }
  
  const globalVer = getVersionOutput("rustowl", ["--version", "--quiet"]);
  if (globalVer) {return "rustowl";}
  
  return null;
};

const installRustowl = async (): Promise<string> => {
  if (!commandExists("cargo") || !commandExists("git")) {
    throw new Error(
      "RustOwl requires cargo and git. Please install Rust via rustup.rs and ensure git is available."
    );
  }

  if (await tryBinstall()) {
    const binary = await findRustowlBinary();
    if (binary) {return binary;}
  }

  if (await buildFromSource()) {
    const targetBinary = path.join(CACHE_DIR, "target", "release", `rustowl${EXE_EXT}`);
    if (await exists(targetBinary)) {
      await createSymlink(targetBinary);
    }
    
    const binary = await findRustowlBinary();
    if (binary) {return binary;}
  }

  void vscode.window.showErrorMessage(
    "RustOwl installation failed. Please install manually:\n" +
    "git clone https://github.com/wvhulle/rustowl.git ~/.cache/rustowl\n" +
    "cd ~/.cache/rustowl && cargo install --path . --locked"
  );
  
  throw new Error("Failed to install RustOwl");
};

export const bootstrapRustowl = async (_dirPath: string): Promise<string> => {
  const config = getConfig();

  if (config.serverPath) {
    const ver = getVersionOutput(config.serverPath, ["--version", "--quiet"]);
    if (ver) {
      console.warn(`Using configured serverPath: ${config.serverPath}`);
      return config.serverPath;
    }
    throw new Error(`Configured serverPath "${config.serverPath}" is not a valid rustowl executable`);
  }

  const existingBinary = await findRustowlBinary();
  
  if (existingBinary) {
    const currentVersion = getVersionOutput(existingBinary, ["--version", "--quiet"]);
    if (!(await needsUpdate(currentVersion))) {
      return existingBinary;
    }
    console.warn("RustOwl update available, installing...");
  }

  return installRustowl();
};
