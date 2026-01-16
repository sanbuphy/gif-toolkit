# Contributing to GIF Toolkit

First off, thank you for considering contributing to GIF Toolkit! It's people like you that make GIF Toolkit such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by the basic principle of being respectful. Please be respectful to all contributors and users.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

**Bug Report Template**

```markdown
**Description**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Run command '...'
3. See error

**Expected Behavior**
A clear and concise description of what you expected to happen.

**Environment**
- OS: [e.g. Windows 11, macOS 14, Ubuntu 22.04]
- GIF Toolkit Version: [e.g. v1.0.0]
- GIF File: [attach if possible]

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Additional Context**
Add any other context about the problem here.
```

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Use a clear and descriptive title**
- **Provide a detailed description of the suggested enhancement**
- **Explain why this enhancement would be useful**
- **List some examples of how this feature would be used**
- **Include mockups or screenshots if applicable**

### Pull Requests

1. **Fork the repository** and create your branch from `main` or `develop`.
2. **If you're adding a new feature**, create an issue first to discuss it.
3. **Write clean, documented code** following the project's coding style.
4. **Add tests** for your changes if applicable.
5. **Ensure all tests pass** before submitting.
6. **Update documentation** if you've changed functionality.

**Pull Request Template**

```markdown
## Description
Brief description of changes made

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
Describe the tests you ran and how to reproduce them

## Checklist
- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published
```

## Development Setup

### Prerequisites

- Git
- Rust toolchain (if using Rust)
- Python 3.8+ (if using Python)
- Appropriate build tools for your platform

### Setting Up Your Development Environment

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/gif-toolkit.git
   cd gif-toolkit
   ```

2. **Install dependencies**
   ```bash
   # Rust
   cargo build

   # Python (if applicable)
   pip install -r requirements-dev.txt
   ```

3. **Run tests**
   ```bash
   # Rust
   cargo test

   # Python (if applicable)
   pytest
   ```

4. **Build the project**
   ```bash
   # Rust
   cargo build --release

   # Python (if applicable)
   python -m build
   ```

## Coding Standards

### General Guidelines
- Write clear, self-documenting code
- Add comments for complex logic
- Follow the existing code style
- Keep functions small and focused
- Use meaningful variable and function names

### Language-Specific Guidelines

**Rust:**
- Follow rustfmt formatting
- Use clippy for linting
- Prefer idiomatic Rust patterns
- Document public APIs with rustdoc

**Python:**
- Follow PEP 8 style guide
- Use type hints for function signatures
- Document with docstrings
- Use Black for formatting

## Testing

### Unit Tests
- Write tests for new functionality
- Aim for high test coverage
- Test edge cases and error conditions

### Integration Tests
- Test end-to-end workflows
- Test with real GIF files
- Verify cross-platform behavior

### Test Data
- Add test GIF files to the `tests/fixtures` directory
- Include various GIF types (animated, static, large, small)
- Test with edge cases (corrupted files, unusual dimensions)

## Documentation

### Code Documentation
- Document public APIs
- Explain complex algorithms
- Add examples where helpful

### README Updates
- Update for new features
- Keep examples current
- Update installation instructions if needed

### Architecture Documentation
- Update ARCHITECTURE.md for structural changes
- Document design decisions

## Commit Messages

Follow these guidelines for commit messages:

- **Use the imperative mood**: "Add feature" not "Added feature"
- **Limit the first line to 72 characters or less**
- **Reference issues and pull requests liberally**
- **Start with one of these types**:
  - `feat:` A new feature
  - `fix:` A bug fix
  - `docs:` Documentation only changes
  - `style:` Changes that don't affect code meaning
  - `refactor:` Code change that neither fixes a bug nor adds a feature
  - `perf:` Performance improvement
  - `test:` Adding or updating tests
  - `chore:` Changes to the build process or auxiliary tools

**Examples:**
```
feat: add support for lossy compression
fix: handle corrupted GIF files gracefully
docs: update installation instructions for Windows
```

## Release Process

Releases are managed by maintainers:
1. Update version number
2. Update CHANGELOG.md
3. Create git tag
4. GitHub Actions builds and publishes release
5. Announce release

## Getting Help

If you need help:
- Check existing documentation
- Search existing issues
- Ask in GitHub Discussions
- Start a conversation with an issue

## Recognition

Contributors will be recognized in:
- CONTRIBUTORS.md file
- Release notes for significant contributions
- Project documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to GIF Toolkit! 🎉
