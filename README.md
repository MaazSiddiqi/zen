<div align="center">

![Zen Banner](./banner.jpg)

</div>

# zen

A simple command launcher and alias manager

## What is this?

`zen` provides a quick and convenient way to register and alias project specific commands.

This is provides a simple way to store project aliases for commands like `run` or `build` or `test` etc. You can abstract away the specifics of the command within a project.

This can be used to build a unified system of commands across multiple projects to reduces the context-switching fatigue and streamline your developer workflow.

This also allows new users onboarding on to your project to get off the ground faster by standardizing common aliases like `run` in your projects.

This is very powerful with a utilities like [zoxide](https://github.com/ajeetdsouza/zoxide) and [tmux](https://github.com/tmux/tmux)

## How to use this?

`zen` works by registering and looking up commands from your projects `zen-config.toml`.

### Register a command

```
$ zz <alias> --register <command> [args]
```

The command has now been registered and can be found in the local `zen-config.toml`. If one doesn't exist, it will be created.

Example: alias for sample nextjs project:

```
$ zz run --register npm run dev
```

### Invoke a `zen` command

```
$ zz <alias>
```

`zen` will lookup the command in its registry, and invoke it.


If you want to invoke and pass in more args to the aliased command, simple pass in more args as needed. All args to the alias will be forwarded to the source command

```
$ zz <alias> --another-arg
```


Example: Run your nextjs project from earlier 

```
$ zz run
```

`zen` also supports executing aliases while respecting your custom shell aliases too! Now your zen alias commands will have access to your shell aliases.

```
$ ZEN_USE_INTERACTIVE=true zz <alias> --another-arg
```

## Command Reference

| Command | Description | Example |
|---------|-------------|---------|
| `zen add <alias> <command>` | Register a new command alias | `zen add dev "npm run dev"` |
| `zen run <alias> [args]` | Execute a registered alias | `zen run dev --port 3000` |
| `zen list` | Show all registered aliases | `zen list` |
| `zen remove <alias>` | Delete an alias | `zen remove dev` |

### Quick Usage with `zz`

For faster workflow, you can also use:

| Command | Equivalent | Description |
|---------|------------|-------------|
| `zz <alias> [args]` | `zen run <alias> [args]` | Quick execution |
| `zz <alias> --register <command>` | `zen add <alias> <command>` | Register in-flow |

### Examples

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
