# Contributing to Multi-Parser

Thank you for your interest in contributing to Multi-Parser! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a new branch for your feature or bugfix

## Development Setup

### Prerequisites
- Rust (latest stable version)
- Cargo package manager
- Git

### Setup Steps
```bash
git clone https://github.com/your-username/multi-parser.git
cd multi-parser
cargo build
cargo test
```

## Contributing Process

1. **Issue Tracking**
   - Check existing issues before creating a new one
   - Clearly describe the problem or feature
   - For new languages, explain the use case and provide example code

2. **Code Contributions**
   - Follow Rust's standard formatting (`rustfmt`)
   - Write clear, concise commit messages
   - Add tests for new features or bug fixes
   - Update documentation

3. **Adding a New Language Parser**
   - Locate the Tree-sitter grammar repository
   - Update `Cargo.toml`
   - Modify `build.rs`
   - Update `src/parser_reg.rs`
   - Add tests for the new language

### Commit Message Guidelines
- Use present tense
- Capitalize the first letter
- No period at the end
- Be descriptive but concise

Example: 
- `Add support for Lua language parser`
- `Fix memory leak in parser compilation`

## Pull Request Process
1. Ensure all tests pass
2. Update README/documentation if needed
3. Add a clear description of changes
4. Link to any related issues

## Code of Conduct
- Be respectful
- Be inclusive
- Provide constructive feedback

## Questions?
Open an issue or reach out to the maintainers.

## Legal
By contributing, you agree to license your changes under the project's MIT License.
