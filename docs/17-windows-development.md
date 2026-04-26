# Windows Development Guide

This guide covers setting up a Windows development environment for ts2wasm without using WSL or bash dependencies.

## Prerequisites

- Windows 10/11
- Administrator access (for some tool installations)

## Required Tools

### 1. Python 3

Install Python 3.10 or later from [python.org](https://www.python.org/downloads/).

```powershell
# Verify installation
python --version
```

### 2. Rust Toolchain

Install Rust using rustup:

```powershell
# Download and run rustup-init.exe from https://rustup.rs/
# Or use winget:
winget install Rustlang.Rustup
```

Install cargo-nextest:

```powershell
cargo install cargo-nextest
```

### 3. Node.js

Install Node.js LTS from [nodejs.org](https://nodejs.org/).

```powershell
# Verify installation
node --version
npm --version
```

### 4. Git

Install Git from [git-scm.com](https://git-scm.com/).

```powershell
# Verify installation
git --version
```

### 5. Additional Tools

Install the following tools using their respective installers:

- **jq**: Download from [stedolan.github.io/jq](https://stedolan.github.io/jq/download/)
- **ripgrep**: Download from [github.com/BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/releases)
- **wasm-tools**: `cargo install wasm-tools`
- **ast-grep**: `cargo install ast-grep`
- **WAMR (iwasm)**: Download Windows binary from [github.com/bytecodealliance/wasm-micro-runtime](https://github.com/bytecodealliance/wasm-micro-runtime/releases)

Add all tools to your PATH.

## Repository Setup

Clone the repository:

```powershell
git clone <repository-url>
cd ts2wasm
```

## Using the Python Script Manager

The project provides `scripts/manager.py` as a cross-platform alternative to the bash-based `scripts/manager`.

### Basic Commands

```powershell
# Format code
python scripts/manager.py fmt

# Run clippy
python scripts/manager.py clippy

# Run tests
python scripts/manager.py nextest

# Run fast gate (fmt + issue health + coverage matrix + tests)
python scripts/manager.py check-fast-gate

# Skip tests in fast gate
python scripts/manager.py check-fast-gate --skip-nextest

# Show all commands
python scripts/manager.py help
```

### Available Commands

- `fmt` - cargo fmt --all --check
- `clippy` - cargo clippy --all-targets
- `nextest` - cargo nextest run
- `check-fast-gate` - fmt + issue health + coverage matrix + nextest
- `check-issue-health` - Validate issues/ directory
- `update-issue-index` - Regenerate issues/index.md
- And more (see `python scripts/manager.py help`)

## Known Limitations

Some scripts still depend on bash and are not yet available on Windows:

- `check-scripts` - Bash syntax check (not needed on Windows)
- `check-manifest-imports` - Requires bash
- `check-toolchain` - Requires bash
- `reference-coverage` - Requires bash
- `test262` - Requires bash
- Many other check/gate/run scripts

For full functionality, consider using WSL2. See the main README for WSL setup instructions.

## Alternative: Using Mise (Experimental)

Mise has Windows support. Install from [mise.jdx.dev](https://mise.jdx.dev/).

```powershell
# Install mise
winget install jdx.mise

# Use mise tasks
mise tasks
mise run fmt
mise run nextest
```

Note: Some mise tasks still call bash scripts and may not work on Windows.

## Development Workflow

### Typical Development Cycle

```powershell
# 1. Format code
python scripts/manager.py fmt

# 2. Run linter
python scripts/manager.py clippy

# 3. Run tests
python scripts/manager.py nextest

# 4. Run full gate (before committing)
python scripts/manager.py check-fast-gate
```

### Issue Management

```powershell
# Validate issues
python scripts/manager.py check-issue-health

# Update issue index
python scripts/manager.py update-issue-index

# Generate issues from coverage
python scripts/manager.py gen-issues-from-coverage --suite test262
```

## Troubleshooting

### "python not found"

Ensure Python is installed and added to PATH. Restart your terminal after installation.

### "cargo not found"

Ensure Rust is installed via rustup and added to PATH. Restart your terminal.

### "iwasm not found"

Download WAMR Windows binary from GitHub releases and add to PATH.

### Script fails with "bash not found"

The script you're trying to run depends on bash. Use the Python manager (`python scripts/manager.py`) instead, or use WSL2 for full functionality.

## Contributing

When contributing Windows-specific fixes:

1. Prefer Python over bash scripts
2. Use `pathlib.Path` for cross-platform path handling
3. Avoid Unix-specific commands (mktemp, grep, etc.)
4. Test on both Windows and Linux if possible

## Future Improvements

The following bash scripts are planned to be migrated to Python for full Windows support:

- [ ] check-toolchain
- [ ] check-manifest-imports
- [ ] reference-coverage
- [ ] test262
- [ ] All remaining check/gate/run scripts

See the issue tracker for progress.
