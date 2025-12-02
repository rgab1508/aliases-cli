# Rust Alias

A simple and fast command-line alias manager written in Rust. Create shortcuts for frequently used commands and navigate directories quickly.

## Features

- 🚀 **Fast execution** - Built with Rust for optimal performance
- 📝 **Simple alias management** - Add, update, and list aliases easily
- 🔄 **Directory navigation** - Change directories with shortcuts that work in your current shell
- 💾 **JSON storage** - Aliases stored in a simple JSON file
- 🛡️ **Safe updates** - Prompts before overwriting existing aliases
- 🔍 **List aliases** - View all your shortcuts at once

## Installation

### Prerequisites

- Rust and Cargo installed ([rustup.rs](https://rustup.rs/))

### Build from source

```bash
git clone <repository-url>
cd ga
cargo build --release
```

The binary will be at `target/release/ga`. You can add it to your PATH:

```bash
# Add to your shell config (.zshrc, .bashrc, etc.)
export PATH="$PATH:/path/to/ga/target/release"
```

Or install it system-wide:

```bash
cargo install --path .
```

## Usage

### Add an alias

Create a new alias with a name and command:

```bash
ga --name myproject --command "cd /path/to/myproject"
# or short form
ga -n myproject -c "cd /path/to/myproject"
```

### Execute an alias

Run an alias by its name:

```bash
ga myproject
```

If the alias doesn't exist, it will be executed as a regular command.

### List all aliases

View all your saved aliases:

```bash
ga --show
# or short form
ga -s
```

### Update an existing alias

If you try to add an alias that already exists, you'll be prompted to update it:

```bash
ga -n myproject -c "cd /new/path"
# Will prompt: "short already exists! (Do you want to update it (y/n))"
```

## Examples

### Directory shortcuts

```bash
# Add shortcuts for common directories
ga -n home -c "cd ~"
ga -n projects -c "cd ~/projects"
ga -n work -c "cd /var/app"

# Use them
ga home      # Changes to home directory
ga projects  # Changes to projects directory
ga work      # Changes to /var/app
```

### Command shortcuts

```bash
# Create shortcuts for complex commands
ga -n status -c "git status"
ga -n logs -c "docker logs -f"
ga -n build -c "cargo build --release"

# Execute them
ga status
ga logs
ga build
```

### View all shortcuts

```bash
$ ga --show
app ~> cd /var/app
home ~> cd ~
projects ~> cd ~/projects
```

## How it works

- **Alias storage**: Aliases are stored in `~/.config/ga/aliases.json` (centralized location, created automatically)
- **Command execution**: Commands are executed in an interactive shell, ensuring your shell configuration (like nvm, etc.) is loaded
- **Directory changes**: When executing `cd` commands, the program changes to the directory and spawns a new shell session in that location
- **Shell compatibility**: Works with zsh, bash, and other Unix shells

## File Structure

```
ga/
├── Cargo.toml          # Rust project configuration
└── src/
    ├── main.rs        # Main application logic
    ├── cli.rs         # CLI argument parsing
    └── alias.rs       # Alias data structures

# Config file location (created automatically)
~/.config/ga/aliases.json
```

## Requirements

- Rust 1.70+ (or latest stable)
- Unix-like system (macOS, Linux) - Windows support may require modifications

## Dependencies

- `clap` - Command-line argument parsing
- `serde` / `serde_json` - JSON serialization/deserialization

## Troubleshooting

### Command not found errors

If you get errors like "command not found: nvm_find_nvmrc" when using `cd` commands, make sure your shell configuration (`.zshrc`, `.bashrc`) is properly loaded. The program uses login shells (`-l` flag) to ensure all configurations are loaded.

### Aliases not persisting

Make sure the `~/.config/ga/` directory is writable. The config file is stored at `~/.config/ga/aliases.json` and is created automatically on first use.
