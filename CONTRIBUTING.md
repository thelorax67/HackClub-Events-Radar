# Contributing to HackClub Events Radar

Thank you for your interest in contributing! We welcome contributions of all kinds.

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/HackClub-Events-Radar.git
   cd HackClub-Events-Radar
   ```
3. **Create a branch** for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

```bash
# Install Rust if you haven't
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Setup .env
cp .env.example .env
# Edit .env with your NVIDIA API key

# Build
cargo build

# Run
cargo run --release -- -v
```

## Code Standards

### Formatting

We use the standard Rust formatting conventions:

```bash
cargo fmt
```

### Linting

Check for common mistakes:

```bash
cargo clippy -- -D warnings
```

### Testing

Run the test suite:

```bash
cargo test
```

## What to Contribute

### Good First Issues

- Documentation improvements
- README clarifications
- Bug fixes with tests
- Refactoring with no behavior change

### Feature Ideas

- Support for additional hackathon sources
- Caching mechanisms for repeated probes
- Custom output formats (CSV, YAML)
- Configuration file support
- Webhook notifications
- Web UI for results

## Pull Request Process

1. **Update documentation** if changing functionality
2. **Add tests** for new features
3. **Ensure code compiles**:
   ```bash
   cargo check
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```
4. **Write clear commit messages**:
   ```
   Fix: Brief description
   - Detailed explanation
   - Any relevant context
   ```
5. **Keep PRs focused** - one feature per PR
6. **Reference issues** if applicable: "Closes #123"

## Commit Guidelines

- Use present tense: "Add feature" not "Added feature"
- Use imperative mood: "Move cursor to..." not "Moves cursor to..."
- Limit subject to 50 characters
- Provide detailed explanations in commit body
- Reference issues and PRs liberally

## Code Style

- Follow Rust naming conventions (snake_case for variables/functions, PascalCase for types)
- Add doc comments to public items
- Keep functions focused and readable
- Use meaningful variable names
- Comments should explain *why*, not *what*

## Questions?

- Open an issue for bug reports
- Start a discussion for feature ideas
- Ask questions in pull reviews

Thank you for contributing! 🚀
