# Development Guide

This section provides comprehensive information for developers who want to contribute to or work with the Rummage MTG Commander game engine.

## Table of Contents

1. [Introduction](#introduction)
2. [Key Development Areas](#key-development-areas)
3. [Development Environment](#development-environment)
4. [Working with Bevy](#working-with-bevy)
5. [Integration with Testing](#integration-with-testing)

## Introduction

The Rummage development guide is designed to help developers understand the architecture, code style, and development practices used in the project. Whether you're a new contributor or an experienced developer, this guide will help you navigate the codebase and make effective contributions.

Rummage follows a test-driven development approach, which means testing is an integral part of the development process. Understanding how development and testing interact will help you create robust, maintainable code that correctly implements the complex MTG rule system.

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
   - [Entity Component System](bevy_guide/ecs.md) - Understanding and working with ECS
   - [Plugin Architecture](bevy_guide/plugins.md) - Creating and using plugins
   - [Rendering](bevy_guide/rendering.md) - Card rendering and UI components

5. [Core Systems](../core_systems/snapshot/index.md)
   - [Snapshot System](../core_systems/snapshot/index.md) - Game state serialization and replay functionality
   - [Testing Integration](../testing/index.md#snapshot-testing) - How core systems integrate with testing

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

## Working with Bevy

Rummage is built on the Bevy game engine, which provides a data-driven, entity-component-system (ECS) architecture. The [Working with Bevy](bevy_guide/index.md) section provides detailed guidance on:

- **Understanding ECS**: How Rummage organizes game elements into entities, components, and systems
- **Plugin Development**: Creating and working with Bevy plugins
- **Rendering Systems**: Implementing visual elements and UI components
- **Bevy 0.15.x Specifics**: Working with the latest Bevy APIs

Bevy 0.15.x introduces some important changes, including deprecated UI components like `Text2dBundle`, `SpriteBundle`, and `NodeBundle` which are replaced by `Text2d`, `Sprite`, and `Node` respectively. Our documentation provides guidance on using these newer APIs correctly.

## Integration with Testing

Testing is a foundational aspect of Rummage development, ensuring that our implementation correctly follows MTG rules and maintains compatibility across system changes. Our [Testing Overview](../testing/index.md) provides comprehensive information on our testing approach.

### Test-Driven Development Workflow

When developing new features for Rummage, we follow this test-driven workflow:

1. **Document the Feature**: Define requirements and behavior in the documentation
2. **Write Tests First**: Create tests that verify the expected behavior
3. **Implement the Feature**: Write code that passes the tests
4. **Refactor**: Improve the implementation while maintaining test coverage
5. **Integration Testing**: Ensure the feature works correctly with other systems

### Testing Infrastructure in Development

Our development process is tightly integrated with testing:

- **ECS System Testing**: Use `ParamSet` and other techniques described in the [ECS Guide](bevy_guide/ecs.md) to avoid runtime panics
- **Snapshot Testing**: Leverage the [Snapshot System](../core_systems/snapshot/testing.md) for deterministic state verification
- **Visual Testing**: For UI components, use our visual differential testing tools

### MTG Rule Verification

When implementing MTG rules, refer to both:

- The [MTG Rules Reference](../mtg_rules/index.md) for authoritative rules text
- The [Testing Guidelines](../testing/unit_testing/rule_testing.md) for advice on verifying rule implementation

By integrating testing throughout the development process, we ensure that Rummage maintains a high level of quality and accurately implements the complex MTG rule system.

## Contributing

We welcome contributions to the Rummage project! Please see our [contribution guidelines](../CONTRIBUTING.md) for information on how to submit changes, report issues, and suggest improvements. For specific guidance on our git workflow and commit message format, refer to our [Git Workflow Guidelines](../contributing/git_workflow.md).

## Next Steps

To start developing with Rummage, we recommend:

1. Read the [Getting Started](getting_started.md) guide
2. Review the [Architecture Overview](architecture.md)
3. Familiarize yourself with [Bevy ECS concepts](bevy_guide/ecs.md)
4. Review the [Testing Overview](../testing/index.md) to understand our testing approach
5. Check out the [API Reference](../api/index.md) for detailed information on specific components and systems

---

For questions or assistance, please reach out to the development team through the project's GitHub repository. 