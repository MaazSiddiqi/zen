# Contributing to zen

Thank you for your interest in contributing to zen!

## Quick Start

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Add tests if applicable
5. Run tests: `cargo test`
6. Commit your changes: `git commit -m "feat: add amazing feature"`
7. Push to your branch: `git push origin feature/amazing-feature`
8. Open a Pull Request

## Development Setup

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- Git

### Local Development
```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/zen.git
cd zen

# Build and test
cargo build
cargo test

# Install locally for testing
cargo install --path .
```

## What We're Looking For

### High Priority
- Bug fixes and stability improvements
- Performance optimizations
- Better error messages and UX
- Cross-platform compatibility improvements

### Feature Ideas
- Global alias registry
- Server process management
- Shell completion
- Config file improvements
- fzf integration enhancements

### Documentation
- README improvements
- Code comments
- Usage examples
- Tutorial content

## Guidelines

### Code Style
- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes
- Keep functions small and focused

### Commit Messages
We use [Conventional Commits](https://www.conventionalcommits.org/):
- `feat:` new features
- `fix:` bug fixes
- `docs:` documentation changes
- `style:` formatting changes
- `refactor:` code refactoring
- `test:` adding tests
- `chore:` maintenance tasks

### Pull Requests
- Keep PRs focused and small
- Include tests for new features
- Update documentation as needed
- Reference related issues

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Questions?

- Open a [Discussion](https://github.com/MaazSiddiqi/zen/discussions)
- Report bugs via [Issues](https://github.com/MaazSiddiqi/zen/issues)
- Suggest features via [Issues](https://github.com/MaazSiddiqi/zen/issues)

We appreciate all contributions, big and small!