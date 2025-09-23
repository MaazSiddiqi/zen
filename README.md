<div align="center">

![Zen Banner](./banner.jpg)

</div>

```bash
# The problem: Context switching between projects
~/react-dashboard $ npm run dev         # React project
~/django-api $ python manage.py runserver  # Django project
~/rust-cli $ cargo run                  # Rust project
~/docker-app $ docker-compose up        # Docker project

# Same developer, different mental overhead every time

# The zen solution: One command everywhere
~/react-dashboard $ zen add dev "npm run dev"
~/django-api $ zen add dev "python manage.py runserver"
~/rust-cli $ zen add dev "cargo run"
~/docker-app $ zen add dev "docker-compose up"

# Now just use muscle memory everywhere:
~/react-dashboard $ zz dev    # → npm run dev
~/django-api $ zz dev         # → python manage.py runserver
~/rust-cli $ zz dev           # → cargo run
~/docker-app $ zz dev         # → docker-compose up

# Same workflow, different projects
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

You can also run your command in interactive mode for commands that require your custom shell configuration like global aliases.

```zsh
export ZEN_ENV_USE_INTERACTIVE=true

# or run directly with your command
ZEN_ENV_USE_INTERACTIVE=true zz run

```

## Command Definitions & Parameter Substitution

### Basic Commands
Commands are stored as simple string templates:
```bash
zen add dev "npm run dev"
zen add build "make clean && make build"
```

### Parameter Substitution
Use `{}` placeholders to insert arguments anywhere in your commands:

```bash
# Single parameter
zen add test "cargo test {} --verbose"
zz test my_module         # Runs: cargo test my_module --verbose

# Multiple parameters
zen add copy "cp {} {}"
zz copy file1.txt file2.txt  # Runs: cp file1.txt file2.txt

# Mixed placeholders and fixed text
zen add deploy "rsync -av {} user@server:{}"
zz deploy ./dist /var/www  # Runs: rsync -av ./dist user@server:/var/www

# Extra arguments are appended
zen add build "make {}"
zz build release --jobs 4  # Runs: make release --jobs 4
```

### How Parameter Substitution Works
1. **Sequential replacement**: `{}` placeholders are replaced left-to-right with provided arguments
2. **Extra arguments**: Any remaining arguments are appended to the end of the command
3. **Flexible**: Works with any number of placeholders and arguments
4. **Backward compatible**: Commands without `{}` work exactly as before

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

### vs Command Runners (like Just)

zen takes a developer-focused approach compared to project-focused tools like [just](https://github.com/casey/just). While just excels at structured automation within projects using justfiles and recipes, zen prioritizes cross-project workflow consistency with zero setup - no files to edit, no syntax to learn, just `zz dev --register "npm run dev"` when you need it. Both tools solve command management from different angles and can be used together: zen streamlines workflows *across* repos, just streamlines workflows *within* repos.

## Roadmap

The essence of  zen  is to be simple. I would love to build more robust features that still keep this simple user experience at its core.

If you have any suggestions or features you would like to see, create an issue or submit a PR!

I've thought of a few things I would like to add myself:

- A global zen command registry, storing global aliases. Local aliases override globals
- `zz` to list currently available commands
- running servers with zen keeps track of the server process, allowing to kill any outstanding servers you don't want from an fzf view. Tackles the `kill on port` problem

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

- Report bugs via [Issues](https://github.com/MaazSiddiqi/zen/issues)
- Suggest features via [Issues](https://github.com/MaazSiddiqi/zen/issues)
- Submit [Pull Requests](https://github.com/MaazSiddiqi/zen/pulls)
