# Contributing to RTGC-0.7

Thank you for your interest in contributing to RTGC-0.7! This document provides guidelines and instructions for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Coding Guidelines](#coding-guidelines)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Collaborate openly and transparently

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/rtgc.git`
3. Create a feature branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Push to your fork: `git push origin feature/my-feature`
6. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.75.0 or later (see `rust-toolchain.toml`)
- CMake 3.20+
- Vulkan SDK (for Vulkan backend)
- Visual Studio Build Tools (Windows, for DX12)

### Installation

```bash
# Install Rust toolchain
rustup install 1.75.0
rustup default 1.75.0

# Install required components
rustup component add rustfmt clippy

# Clone the repository
git clone https://github.com/your-username/rtgc.git
cd rtgc

# Build the project
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Coding Guidelines

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Write doc comments for public APIs

### File Organization

```
src/
├── main.rs              # Entry point
├── engine.rs            # Main engine loop
├── graphics/            # Rendering subsystem
│   ├── rhi/            # Render Hardware Interface
│   │   ├── gl.rs       # OpenGL backend
│   │   ├── vulkan/     # Vulkan backend
│   │   └── dx12/       # DirectX 12 backend
│   ├── renderer.rs     # High-level renderer
│   ├── lighting.rs     # PBR lighting system
│   └── ...
├── physics/            # Physics subsystem
├── world/              # World generation & streaming
├── ecs/                # Entity Component System
├── audio/              # Audio subsystem
├── input/              # Input handling
├── ui/                 # User interface
├── assets/             # Asset loading
└── utils/              # Utility functions
```

### Naming Conventions

- Structs/Enums: `PascalCase`
- Functions/Methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Traits: `PascalCase` (often with descriptive suffix like `Trait`)

### Error Handling

- Use `Result<T, E>` for recoverable errors
- Use `Option<T>` for optional values
- Avoid `.unwrap()` in production code
- Provide meaningful error messages

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        // Act
        // Assert
    }
}
```

Run tests before submitting:
```bash
cargo test --all-targets
cargo test --all-targets --release
```

## Pull Request Process

### Before Submitting

1. **Update documentation**: Ensure all new features are documented
2. **Add tests**: Include unit tests for new functionality
3. **Run linters**: 
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features
   ```
4. **Run tests**: All tests must pass
5. **Update CHANGELOG.md**: Document your changes

### PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested these changes

## Checklist
- [ ] Code follows style guidelines
- [ ] Tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
```

### Review Process

1. Maintainer reviews code
2. Automated CI checks run
3. Address review feedback
4. PR is merged when approved

## Issue Reporting

### Bug Reports

Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- System information (OS, Rust version, GPU)
- Logs (use `RUST_LOG=debug`)

### Feature Requests

Include:
- Problem description
- Proposed solution
- Use cases
- Alternative solutions considered

### Labels

- `bug`: Something isn't working
- `enhancement`: New feature request
- `documentation`: Documentation improvements
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention needed
- `priority: high`: Important issue

## Architecture Overview

### Core Systems

1. **Engine Loop** (`engine.rs`): Main game loop, state management
2. **RHI** (`graphics/rhi/`): Hardware abstraction for rendering
3. **Physics** (`physics/`): Rigid body dynamics, collisions
4. **ECS** (`ecs/`): Entity Component System for game objects
5. **World** (`world/`): Terrain, chunk streaming, LOD

### Data Flow

```
Input → ECS Systems → Physics → Rendering → Output
           ↓
        Audio
```

## Questions?

- Check existing issues and discussions
- Read the [README.md](README.md)
- Review the [PLEN.md](PLEN.md) for project goals

Thank you for contributing to RTGC-0.7! 🚁
