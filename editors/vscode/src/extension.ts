import * as vscode from "vscode";
import type { Range } from "vscode-languageserver-types";

import { bootstrapRustowl } from "./bootstrap";
import {
  LanguageClient,
  type Executable,
  TransportKind,
  type LanguageClientOptions,
} from "vscode-languageclient/node";

type DisplayMode = "selected" | "hover" | "manual" | "disabled";

type LspDecorationType =
  | "lifetime"
  | "imm_borrow"
  | "mut_borrow"
  | "move"
  | "call"
  | "shared_mut"
  | "outlive";

interface LspDecoration {
  readonly type: LspDecorationType;
  readonly range: Range;
  readonly hover_text?: string | null;
  readonly overlapped: boolean;
}

interface LspCursorResponse {
  readonly is_analyzed: boolean;
  readonly status: "analyzing" | "finished" | "error";
  readonly decorations: readonly LspDecoration[];
}

export let client: LanguageClient | undefined = undefined;

let decoTimer: NodeJS.Timeout | null = null;

const getConfig = (): vscode.WorkspaceConfiguration =>
  vscode.workspace.getConfiguration("rustowl");

const getDisplayMode = (): DisplayMode =>
  getConfig().get<DisplayMode>("displayMode", "selected");

const getDisplayDelay = (): number =>
  getConfig().get<number>("displayDelay", 500);

interface DecorationConfig {
  readonly underlineThickness: string;
  readonly lifetimeColor: string;
  readonly moveCallColor: string;
  readonly immutableBorrowColor: string;
  readonly mutableBorrowColor: string;
  readonly outliveColor: string;
  readonly highlightBackground: boolean;
}

const getDecorationConfig = (): DecorationConfig => {
  const config = getConfig();
  return {
    underlineThickness: config.get<string>("underlineThickness", "2"),
    lifetimeColor: config.get<string>("lifetimeColor", "hsla(125, 80%, 60%, 0.6)"),
    moveCallColor: config.get<string>("moveCallColor", "hsla(35, 80%, 60%, 0.6)"),
    immutableBorrowColor: config.get<string>("immutableBorrowColor", "hsla(230, 80%, 60%, 0.6)"),
    mutableBorrowColor: config.get<string>("mutableBorrowColor", "hsla(300, 80%, 60%, 0.6)"),
    outliveColor: config.get<string>("outliveColor", "hsla(0, 80%, 60%, 0.6)"),
    highlightBackground: config.get<boolean>("highlightBackground", false),
  };
};

const lspRangeToVscodeRange = (range: Range): vscode.Range =>
  new vscode.Range(
    new vscode.Position(range.start.line, range.start.character),
    new vscode.Position(range.end.line, range.end.character),
  );

const createDecorationType = (
  color: string,
  thickness: string,
  useBackground: boolean,
): vscode.TextEditorDecorationType =>
  useBackground
    ? vscode.window.createTextEditorDecorationType({ backgroundColor: color })
    : vscode.window.createTextEditorDecorationType({
        textDecoration: `underline solid ${thickness}px ${color}`,
      });

const categorizeDecoration = (
  deco: LspDecoration,
): "lifetime" | "immut" | "mut" | "moveCall" | "outlive" => {
  switch (deco.type) {
    case "lifetime":
      return "lifetime";
    case "imm_borrow":
      return "immut";
    case "mut_borrow":
      return "mut";
    case "call":
    case "move":
      return "moveCall";
    default:
      return "outlive";
  }
};

class DecorationManager {
  private lifetimeDecorationType = vscode.window.createTextEditorDecorationType({});
  private moveDecorationType = vscode.window.createTextEditorDecorationType({});
  private imBorrowDecorationType = vscode.window.createTextEditorDecorationType({});
  private mBorrowDecorationType = vscode.window.createTextEditorDecorationType({});
  private outLiveDecorationType = vscode.window.createTextEditorDecorationType({});
  private emptyDecorationType = vscode.window.createTextEditorDecorationType({});

  public dispose(): void {
    this.lifetimeDecorationType.dispose();
    this.moveDecorationType.dispose();
    this.imBorrowDecorationType.dispose();
    this.mBorrowDecorationType.dispose();
    this.outLiveDecorationType.dispose();
    this.emptyDecorationType.dispose();
  }

  public update(editor: vscode.TextEditor, data: LspCursorResponse): void {
    this.dispose();

    const config = getDecorationConfig();
    this.lifetimeDecorationType = createDecorationType(config.lifetimeColor, config.underlineThickness, config.highlightBackground);
    this.moveDecorationType = createDecorationType(config.moveCallColor, config.underlineThickness, config.highlightBackground);
    this.imBorrowDecorationType = createDecorationType(config.immutableBorrowColor, config.underlineThickness, config.highlightBackground);
    this.mBorrowDecorationType = createDecorationType(config.mutableBorrowColor, config.underlineThickness, config.highlightBackground);
    this.outLiveDecorationType = createDecorationType(config.outliveColor, config.underlineThickness, config.highlightBackground);
    this.emptyDecorationType = vscode.window.createTextEditorDecorationType({});

    const grouped = {
      lifetime: [] as vscode.DecorationOptions[],
      immut: [] as vscode.DecorationOptions[],
      mut: [] as vscode.DecorationOptions[],
      moveCall: [] as vscode.DecorationOptions[],
      outlive: [] as vscode.DecorationOptions[],
      messages: [] as vscode.DecorationOptions[],
    };

    for (const deco of data.decorations) {
      const range = lspRangeToVscodeRange(deco.range);

      if (!deco.overlapped) {
        const category = categorizeDecoration(deco);
        grouped[category].push({ range });
      }

      if (deco.hover_text) {
        grouped.messages.push({ range, hoverMessage: deco.hover_text });
      }
    }

    editor.setDecorations(this.lifetimeDecorationType, grouped.lifetime);
    editor.setDecorations(this.imBorrowDecorationType, grouped.immut);
    editor.setDecorations(this.mBorrowDecorationType, grouped.mut);
    editor.setDecorations(this.moveDecorationType, grouped.moveCall);
    editor.setDecorations(this.outLiveDecorationType, grouped.outlive);
    editor.setDecorations(this.emptyDecorationType, grouped.messages);
  }
}

class StatusBarManager {
  private readonly statusBar: vscode.StatusBarItem;

  constructor() {
    this.statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 0);
    this.statusBar.text = "RustOwl";
    this.statusBar.command = {
      command: "rustowl.cycleDisplayMode",
      title: "Cycle display mode",
      tooltip: "Cycle RustOwl display mode",
    };
    this.statusBar.show();
  }

  public updateFromLspStatus(status: string): void {
    switch (status) {
      case "finished":
        this.statusBar.text = "$(check) RustOwl";
        this.statusBar.tooltip = "Analysis finished";
        break;
      case "analyzing":
        this.statusBar.text = "$(loading~spin) RustOwl";
        this.statusBar.tooltip = "Analyzing...";
        break;
      default:
        this.statusBar.text = "$(error) RustOwl";
        this.statusBar.tooltip = "Analysis failed";
    }
    this.statusBar.show();
  }

  public updateFromDisplayMode(mode: DisplayMode): void {
    const modeConfig: Record<DisplayMode, { icon: string; tooltip: string }> = {
      selected: { icon: "check", tooltip: "Display mode: selected" },
      hover: { icon: "eye", tooltip: "Display mode: hover" },
      manual: { icon: "tools", tooltip: "Display mode: manual" },
      disabled: { icon: "debug-pause", tooltip: "Display mode: disabled" },
    };
    const config = modeConfig[mode];
    this.statusBar.text = `$(${config.icon}) RustOwl`;
    this.statusBar.tooltip = config.tooltip;
    this.statusBar.show();
  }

  public dispose(): void {
    this.statusBar.dispose();
  }
}

const initializeClient = async (
  context: vscode.ExtensionContext,
  clientOptions: LanguageClientOptions,
): Promise<void> => {
  const command = await bootstrapRustowl(context.globalStorageUri.fsPath);
  const serverOptions: Executable = {
    command,
    transport: TransportKind.stdio,
  };

  client = new LanguageClient("rustowl", "RustOwl", serverOptions, clientOptions);
  await client.start();
};

const isLspCursorResponse = (value: unknown): value is LspCursorResponse =>
  typeof value === "object" &&
  value !== null &&
  "is_analyzed" in value &&
  "status" in value &&
  "decorations" in value &&
  Array.isArray((value as LspCursorResponse).decorations);

const sendCursorRequest = async (
  position: vscode.Position,
  uri: vscode.Uri,
): Promise<LspCursorResponse | null> => {
  const resp = await client?.sendRequest("rustowl/cursor", {
    position: { line: position.line, character: position.character },
    document: { uri: uri.toString() },
  });
  return isLspCursorResponse(resp) ? resp : null;
};

const registerCommands = (
  context: vscode.ExtensionContext,
  activeEditorRef: { current: vscode.TextEditor | undefined },
  decorationManager: DecorationManager,
  statusBarManager: StatusBarManager,
): void => {
  const handleCursorRequest = async (
    editor: vscode.TextEditor,
    position: vscode.Position,
  ): Promise<void> => {
    const data = await sendCursorRequest(position, editor.document.uri);
    if (data) {
      statusBarManager.updateFromLspStatus(data.status);
      decorationManager.update(editor, data);
    }
  };

  const registerCommand = (
    id: string,
    handler: (...args: never[]) => unknown,
  ): void => {
    context.subscriptions.push(vscode.commands.registerCommand(id, handler));
  };

  registerCommand("rustowl.hover", async () => {
    const editor = activeEditorRef.current;
    if (editor) {
      await handleCursorRequest(editor, editor.selection.active);
    }
  });

  registerCommand("rustowl.cycleDisplayMode", async () => {
    const modes: DisplayMode[] = ["selected", "hover", "manual", "disabled"];
    const current = getDisplayMode();
    const nextMode = modes[(modes.indexOf(current) + 1) % modes.length];
    await getConfig().update("displayMode", nextMode, vscode.ConfigurationTarget.Global);
    void vscode.window.showInformationMessage(`RustOwl display mode: ${nextMode}`);
  });

  registerCommand("rustowl.toggle", async () => {
    const current = getDisplayMode();
    const newMode = current === "disabled" ? "selected" : "disabled";
    await getConfig().update("displayMode", newMode, vscode.ConfigurationTarget.Global);
    void vscode.window.showInformationMessage(
      `RustOwl ${newMode === "disabled" ? "disabled" : "enabled"}`,
    );
  });

  registerCommand(
    "rustowl.toggleOwnership",
    async (uri?: string, line?: number, character?: number) => {
      const args =
        uri && typeof line === "number" && typeof character === "number"
          ? [uri, line, character]
          : activeEditorRef.current
            ? [
                activeEditorRef.current.document.uri.toString(),
                activeEditorRef.current.selection.active.line,
                activeEditorRef.current.selection.active.character,
              ]
            : null;

      if (args) {
        await client?.sendRequest("workspace/executeCommand", {
          command: "rustowl.toggleOwnership",
          arguments: args,
        });
      }
    },
  );

  registerCommand("rustowl.analyze", async () => {
    await client?.sendRequest("rustowl/analyze", {});
    void vscode.window.showInformationMessage("RustOwl: Re-analyzing workspace...");
  });
};

const registerEventHandlers = (
  context: vscode.ExtensionContext,
  activeEditorRef: { current: vscode.TextEditor | undefined },
  decorationManager: DecorationManager,
  statusBarManager: StatusBarManager,
): void => {
  const triggerDecoration = (editor: vscode.TextEditor, position: vscode.Position): void => {
    decorationManager.dispose();

    if (decoTimer) {
      clearTimeout(decoTimer);
      decoTimer = null;
    }

    decoTimer = setTimeout(() => {
      void (async (): Promise<void> => {
        const data = await sendCursorRequest(position, editor.document.uri);
        if (data) {
          statusBarManager.updateFromLspStatus(data.status);
          decorationManager.update(editor, data);
        }
      })();
    }, getDisplayDelay());
  };

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      activeEditorRef.current = editor;
    }),

    vscode.workspace.onDidSaveTextDocument((doc) => {
      if (doc.languageId === "rust") {
        void client?.sendRequest("rustowl/analyze", {});
      }
    }),

    vscode.window.onDidChangeTextEditorSelection((ev) => {
      const mode = getDisplayMode();
      if (
        mode === "selected" &&
        ev.textEditor === activeEditorRef.current &&
        ev.textEditor.document.languageId === "rust"
      ) {
        triggerDecoration(ev.textEditor, ev.textEditor.selection.active);
      }
    }),

    vscode.languages.registerHoverProvider("rust", {
      provideHover(document, position) {
        if (getDisplayMode() === "hover") {
          const editor = vscode.window.activeTextEditor;
          if (editor?.document === document) {
            triggerDecoration(editor, position);
          }
        }
        return null;
      },
    }),

    vscode.workspace.onDidChangeConfiguration((ev) => {
      if (ev.affectsConfiguration("rustowl.displayMode")) {
        const mode = getDisplayMode();
        statusBarManager.updateFromDisplayMode(mode);
        if (mode === "disabled") {
          decorationManager.dispose();
        }
      }
    }),
  );
};

export function activate(context: vscode.ExtensionContext): void {
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "rust" }],
  };

  const activeEditorRef = { current: vscode.window.activeTextEditor };
  const decorationManager = new DecorationManager();
  const statusBarManager = new StatusBarManager();

  context.subscriptions.push({ dispose: () => decorationManager.dispose() });
  context.subscriptions.push({ dispose: () => statusBarManager.dispose() });

  void initializeClient(context, clientOptions).catch((e: unknown) => {
    void vscode.window.showErrorMessage(`Failed to start RustOwl\n${String(e)}`);
  });

  registerCommands(context, activeEditorRef, decorationManager, statusBarManager);
  registerEventHandlers(context, activeEditorRef, decorationManager, statusBarManager);
}

export async function deactivate(): Promise<void> {
  if (client) {
    await client.stop();
    client = undefined;
  }
}
