<div align="center">

![Zen Banner](./banner.jpg)

</div>

```bash
# Set up aliases for a React project
zen add dev "npm run dev"
zen add build "npm run build"
zen add test "npm test -- --watchAll=false"

# Quick execution
zz dev                    # Runs: npm run dev
zz test --verbose        # Runs: npm test -- --watchAll=false --verbose

# In-flow registration (when you forget to set up an alias)
zz deploy                # Error: No command registered
# ↑ + --register
zz deploy --register "npm run build && aws s3 sync dist/ s3://my-bucket"
```

# zen

A simple command launcher and alias manager

## What is this?

`zen` lets you create project-specific command aliases. Instead of remembering `npm run dev` for React, `cargo run` for Rust, or `python manage.py runserver` for Django, just use `zz run` everywhere.

Reduces context-switching fatigue and standardizes workflows across projects.

## Command Reference

| Command | Description | Example |
|---------|-------------|---------|
| `zen add <alias> <command>` | Register a new command alias | `zen add dev "npm run dev"` |
| `zen run <alias> [args]` | Execute a registered alias | `zen run dev --port 3000` |
| `zen list` | Show all registered aliases | `zen list` |
| `zen remove <alias>` | Delete an alias | `zen remove dev` |
| `zen browse` | Interactive alias selection (requires fzf) | `zen browse` |

### Quick Usage with `zz`

For faster workflow, you can also use:

| Command | Equivalent | Description |
|---------|------------|-------------|
| `zz <alias> [args]` | `zen run <alias> [args]` | Quick execution |
| `zz <alias> --register <command>` | `zen add <alias> <command>` | Register in-flow |
| `zz` | `zen browse` | Interactive selection |

## Installation

### Homebrew (Recommended)

```bash
brew tap MaazSiddiqi/tap
brew install zen --formula
```

After installation, set up the `zz` alias for quick access:
```bash
echo 'alias zz="zen run"' >> ~/.zshrc
source ~/.zshrc
```

> **Note:** For bash users, replace `.zshrc` with `.bashrc` in the commands above.

### Manual Installation

#### Prerequisites
- [Rust](https://rustup.rs/) (cargo required for building)
- Git

#### Steps

1. **Clone the repository**
   ```bash
   git clone https://github.com/MaazSiddiqi/zen.git
   cd zen
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Install the binary**
   ```bash
   # Copy to a directory in your PATH
   cp target/release/zen ~/.local/bin/zen
   # Or for system-wide installation (requires sudo)
   sudo cp target/release/zen /usr/local/bin/zen
   ```

4. **Add to PATH** (if using ~/.local/bin)
   ```bash
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
   ```

5. **Set up the zz alias**
   ```bash
   echo 'alias zz="zen run"' >> ~/.zshrc
   ```

6. **Reload your shell**
   ```bash
   source ~/.zshrc
   ```

### Optional: fzf Integration

For the best experience with interactive browsing, install [fzf](https://github.com/junegunn/fzf):

```bash
# macOS
brew install fzf

# Ubuntu/Debian
sudo apt install fzf
```

## Why?

I work on many projects simultaneously, spread widely between many different languages and frameworks. I find it very annoying to memorize the commands for each project, especially if I'm revisiting a project after a while.

I found myself writing simple `run.sh` scripts in these projects. This was helpful as I stopped having to memorize the command, and typing `./run.sh` was much faster than `npm run dev`

I also started noticing that my projects had other commands I would find useful, like `build.sh`, `init.sh`, `test.sh` etc.

These little scripts were honestly such a time saver and took off so much mental load when context switching between all my different projects.

The only issue was I very lazy to have to write these scripts, even if they were only 1 line long. That's why this project was born.

## Roadmap

The essence of  zen  is to be simple. I would love to build more robust features that still keep this simple user experience at its core.

If you have any suggestions or features you would like to see, create an issue or submit a PR!

I've thought of a few things I would like to add myself:

- A global zen command registry, storing global aliases. Local aliases override globals
- `zz` to list currently available commands
- running servers with zen keeps track of the server process, allowing to kill any outstanding servers you don't want from an fzf view. Tackles the `kill on port` problem
