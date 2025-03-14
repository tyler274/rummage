# CI/CD Pipeline

This document describes the continuous integration and continuous deployment pipeline for the Rummage project.

## Overview

Rummage employs a comprehensive CI/CD pipeline to ensure code quality, prevent regressions, and automate the build and deployment process. The pipeline is designed to catch issues early and provide rapid feedback to developers.

## Pipeline Structure

Our CI/CD pipeline consists of the following stages:

1. **Code Validation**
   - Linting and formatting checks
   - Static analysis
   - Dependency vulnerability scanning

2. **Unit Testing**
   - Fast unit tests run on every PR
   - Component and system validation
   - Rule implementation verification

3. **Integration Testing**
   - System interaction tests
   - Game flow validation
   - ECS pattern verification

4. **End-to-End Testing**
   - Complete game scenario tests
   - Cross-system integration tests
   - Performance benchmarks

5. **Build and Packaging**
   - Multi-platform builds
   - Asset bundling
   - Documentation generation

6. **Deployment**
   - Development environment deployment
   - Release candidate publishing
   - Production deployment

## GitHub Actions Workflow

The pipeline is implemented using GitHub Actions with the following key workflows:

### Pull Request Checks

Triggered on every PR to the main branch:

```yaml
name: PR Validation

on:
  pull_request:
    branches: [ main ]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      
      - name: Check formatting
        run: cargo fmt -- --check
      
      - name: Run Clippy
        run: cargo clippy -- -D warnings
      
      - name: Run unit tests
        run: cargo test --lib --bins
```

### Main Branch Integration

Triggered on merges to the main branch:

```yaml
name: Main Integration

on:
  push:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run all tests
        run: cargo test
      
      - name: Run benchmarks
        run: cargo bench
  
  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build release
        run: cargo build --release
      
      - name: Upload build artifacts
        uses: actions/upload-artifact@v3
        with:
          name: rummage-build
          path: target/release/rummage
```

## Local Development Integration

The CI/CD pipeline is also integrated with local development through pre-commit hooks that run a subset of the pipeline checks locally:

```bash
#!/bin/sh
# Pre-commit hook for Rummage development
# Install with: cp .github/hooks/pre-commit .git/hooks/ && chmod +x .git/hooks/pre-commit

echo "Running pre-commit checks..."

# Format code
cargo fmt -- --check
if [ $? -ne 0 ]; then
  echo "Error: Code formatting issues detected"
  exit 1
fi

# Run clippy
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
  echo "Error: Clippy warnings detected"
  exit 1
fi

# Run fast tests
cargo test --lib
if [ $? -ne 0 ]; then
  echo "Error: Unit tests failed"
  exit 1
fi

echo "All pre-commit checks passed!"
exit 0
```

## Test Coverage Reporting

The pipeline includes test coverage reporting to track which parts of the codebase are well-tested:

1. **Coverage Generation**: Using tools like tarpaulin to generate coverage reports
2. **Coverage Visualization**: Uploading coverage reports to services like Codecov
3. **Minimum Coverage Requirements**: Enforcing minimum coverage thresholds

## Performance Regression Testing

To catch performance regressions early:

1. **Benchmark Tracking**: Storing benchmark results across commits
2. **Performance Alerts**: Notifying developers of significant performance changes
3. **Resource Profiling**: Monitoring memory usage and CPU utilization

## Related Documentation

- [Testing Overview](index.md)
- [Unit Testing](unit_testing.md)
- [Integration Testing](integration_testing.md)
- [End-to-End Testing](end_to_end_testing.md)
- [Development Integration](development_integration.md) 