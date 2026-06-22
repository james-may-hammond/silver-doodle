# gog-wine

Declaratively install DRM-free Windows games on macOS using Wine-compatible runners.

`gog-wine` is a lightweight command-line tool that automates the installation of DRM-free Windows games from GOG. Each installation is described by a simple YAML manifest, making the process predictable, reproducible, and easy to share.

## Features

* Declarative installation using a YAML manifest
* Support for multiple Wine-compatible runners
* Isolated Wine prefix for every game
* Installs games into `~/Applications/Games`
* Simple, Unix-style command-line interface

## Installation

> Coming soon.

## Usage

### Create a new installation

```bash
gog-wine init <GAME_NAME> [DIRECTORY]
```

If `DIRECTORY` is omitted, a directory with the same name as the game will be created.

Example:

```bash
gog-wine init "Hollow Knight"
```

Creates:

```text
Hollow Knight/
├── gog-wine.yaml
└── installer/
```

Copy the GOG installer into the `installer/` directory and edit `gog-wine.yaml`.

### Install a game

From inside the game directory:

```bash
gog-wine install
```
`gog-wine` reads `gog-wine.yaml`, creates the Wine prefix, installs any required dependencies, runs the installer, and installs the game into:

```text
~/Applications/Games/<Game Name>/
```

## Example Manifest

```yaml
name: Hollow Knight

runner: whisky

dependencies:
  - vcrun2022

launch:
  executable: Hollow Knight.exe
```

## Directory Structure

```yaml
gog-wine/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── cli.rs
    ├── commands/
    │   ├── mod.rs
    │   ├── init.rs
    │   └── install.rs
    ├── manifest.rs
    └── template.rs
```
## License

MIT
