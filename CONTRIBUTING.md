# Contributing to HexaGo

First off, thank you for considering contributing to **HexaGo**! It's people like you who make this hexagonal architecture kit better for everyone.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct.

## How Can I Contribute?

### Reporting Bugs

- Use the Bug Report issue template
- Provide a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Include Go version and OS

### Suggesting Enhancements

- Check the Issues to see if the enhancement has already been suggested
- Use the Feature Request issue template
- Describe the use case and why it would benefit the project

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes (`go test ./...`)
5. Make sure your code lints (`golangci-lint run`)
6. Format your code (`go fmt ./...`)

#### Branch Naming

```
feat/<feature-name>     # New features
fix/<bug-description>  # Bug fixes
docs/<topic>           # Documentation
refactor/<area>        # Code refactoring
```

#### Commit Messages

Follow conventional commits:

```
feat(domain): add aggregate root pattern
fix(ports): correct interface definition
docs(readme): update quick start example
```

## Development Setup

```bash
# Clone the repository
git clone https://github.com/phenotype-dev/go-hex.git
cd go-hex

# Run tests
go test ./...

# Run linter
golangci-lint run

# Format code
go fmt ./...

# Run all checks
make ci
```

## Architecture Guidelines

HexaGo follows hexagonal architecture principles:

- **Domain Layer**: Pure business logic, zero external dependencies
- **Ports Layer**: Interface definitions
- **Application Layer**: Use cases and DTOs
- **Adapters Layer**: External integrations (REST, gRPC, CLI)

### Key Principles

1. **Dependency Rule**: Dependencies point toward domain
2. **Pure Domain**: Domain has no external dependencies
3. **Interface Definitions**: Ports defined in ports layer, implemented in adapters
4. **Go Idioms**: Follow Go conventions and best practices

## Code Style

- Use Go's standard formatting (`go fmt`)
- Follow Effective Go guidelines
- Write godoc comments for exported functions
- Use meaningful variable and function names

## License

By contributing, you agree that your contributions will be licensed under the MIT license.

---

Thank you for your contributions!
