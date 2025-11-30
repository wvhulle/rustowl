## Architecture / OS / package repositories

### [Cargo Binstall](https://github.com/cargo-bins/cargo-binstall)

One of the easiest way to install RustOwl is using cargo-binstall.

```bash
cargo binstall rustowl
```

Toolchain is automatically Downloaded and unpacked.

### Windows

We have a winget package, install with:

```sh
winget install rustowl
```

### Archlinux

We have an AUR package. It downloads prebuilt binaries from release page. Run:

```sh
yay -S rustowl-bin
```

If you would like to build from that version instead:

```sh
yay -S rustowl
```

Replace `yay` with your AUR helper of choice.

We also have a git version, that builds from source:

```sh
yay -S rustowl-git
```

### Docker

You can run `rustowl` using the pre-built Docker image from GitHub Container Registry (GHCR).

1. Pull the latest stable image

```sh
docker pull ghcr.io/wvhulle/rustowl:latest
```

Or pull a specific version:

```sh
docker pull ghcr.io/wvhulle/rustowl:v0.3.4
```

2. Run the image

```sh
docker run --rm -v /path/to/project:/app ghcr.io/wvhulle/rustowl:latest
```

You can also pass command-line arguments as needed:

```sh
docker run --rm /path/to/project:/app ghcr.io/wvhulle/rustowl:latest --help
```

3. (Optional) Use as a CLI

To use `rustowl` as if it were installed on your system, you can create a shell alias:

```sh
alias rustowl='docker run --rm -v $(pwd):/app ghcr.io/wvhulle/rustowl:latest'
```

Now you can run `rustowl` from your terminal like a regular command.

## Build from source

See [source/README.md](source/README.md) for building RustOwl from source.
