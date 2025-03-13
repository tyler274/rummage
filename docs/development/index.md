# Development Guide

This section provides comprehensive information for developers who want to contribute to or work with the Rummage MTG Commander game engine.

## Table of Contents

1. [Introduction](#introduction)
2. [Key Development Areas](#key-development-areas)
3. [Development Environment](#development-environment)

## Introduction

The Rummage development guide is designed to help developers understand the architecture, code style, and development practices used in the project. Whether you're a new contributor or an experienced developer, this guide will help you navigate the codebase and make effective contributions.

## Key Development Areas

The development documentation is organized into these key areas:

1. [Getting Started](getting_started.md)
   - Setting up your development environment
   - Building and running the project
   - First steps for new contributors

2. [Architecture Overview](architecture.md)
   - High-level system architecture
   - Component relationships
   - Design patterns used

3. [Code Style](code_style.md)
   - Coding conventions
   - Documentation standards
   - Best practices

4. [Working with Bevy](bevy_guide/index.md)
   - Entity Component System patterns
   - Plugin development
   - Rendering system usage

## Development Environment

To work with Rummage, we recommend the following tools and configurations:

### Required Tools

- Rust (latest stable version)
- Cargo (comes with Rust)
- Git
- A compatible IDE (Visual Studio Code with rust-analyzer recommended)

### Recommended Extensions

For Visual Studio Code:
- rust-analyzer: For Rust language support
- CodeLLDB: For debugging Rust applications
- Better TOML: For editing TOML configuration files

### Building the Project

Basic build commands:

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run the application
cargo run

# Run tests
cargo test
```

## Contributing

We welcome contributions to the Rummage project! Please see our [contribution guidelines](../CONTRIBUTING.md) for information on how to submit changes, report issues, and suggest improvements.

## Next Steps

To start developing with Rummage, we recommend:

1. Read the [Getting Started](getting_started.md) guide
2. Review the [Architecture Overview](architecture.md)
3. Familiarize yourself with [Bevy ECS concepts](bevy_guide/ecs.md)
4. Check out the [API Reference](../api/index.md) for detailed information on specific components and systems

---

For questions or assistance, please reach out to the development team through the project's GitHub repository. 