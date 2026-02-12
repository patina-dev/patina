# Contributing to Patina

Thank you for your interest in contributing to Patina. Whether you're fixing a bug, proposing a new rule, improving documentation, or reporting false positives — your input makes this tool better for everyone.

## Getting Started

1. Fork the repository and clone it locally
2. Make sure you have Rust installed (edition 2024)
3. Build and run the tests:

```bash
cargo build
cargo test
```

4. Try it out on some JavaScript/TypeScript code:

```bash
cargo run -- scan path/to/your/code
cargo run -- scan path/to/your/code --format json
```

## Ways to Contribute

### Report False Positives (or False Negatives)

This is one of the most valuable contributions you can make. If Patina flags something it shouldn't, or misses something it should catch, open an issue with:

- The code snippet that triggered (or didn't trigger) the finding
- Which rule was involved (e.g., `slop-001`)
- Why you think the result is incorrect

Tuning heuristics requires real-world examples. Yours matter.

### Submit a Bug Fix or Improvement

1. Check the [open issues](https://github.com/patina-dev/patina/issues) for something that interests you
2. Comment on the issue to let others know you're working on it
3. Create a branch from `main` (e.g., `fix/issue-number-short-description`)
4. Make your changes
5. Make sure `cargo test` passes and `cargo clippy` is clean
6. Open a pull request

### Propose a New Rule

Patina's value comes from its rules. If you have an idea for a new detection heuristic:

1. Open an issue describing:
   - What pattern the rule detects
   - Examples of code that should and should not be flagged
   - Why this pattern is problematic
   - Estimated false positive risk (low, medium, high)
2. If the idea gains traction, write a spec in `docs/<rule-id>/SPEC.md` following the format of `docs/slop-001/SPEC.md`
3. Implement the rule with test fixtures

### Add Language Support

Patina uses tree-sitter for parsing. Adding a new language requires:

1. A tree-sitter grammar crate (e.g., `tree-sitter-python`)
2. A parser implementation in `src/parsers/`
3. Verification that existing rules work correctly with the new grammar's node types

### Improve Documentation

Clear documentation is part of the project's mission. If something is confusing, incomplete, or could be explained better, a PR is welcome.

## Code Guidelines

- **Keep it simple.** Patina values clarity over cleverness. If a three-line solution works, don't write a ten-line abstraction.
- **No premature abstraction.** Add interfaces and generics when you have two concrete use cases, not before.
- **Test with fixtures.** Every rule should have corresponding test fixtures in `tests/fixtures/` with `// expect: <rule-id>` annotations.
- **No network calls, no models, no telemetry.** Patina is deterministic and local. Always.

## Pull Request Process

1. Keep PRs focused — one logical change per PR
2. Include tests for new functionality
3. Update relevant documentation if behavior changes
4. All CI checks must pass (`cargo build`, `cargo test`, `cargo clippy`)
5. PRs will be reviewed by a maintainer before merging

## Code of Conduct

Be respectful. This project exists to improve code quality, not to judge people. We welcome contributors regardless of experience level, background, or how they choose to use AI in their workflow.

## License

By contributing to Patina, you agree that your contributions will be licensed under the GNU General Public License v3.0 (GPL-3.0), the same license as the project.

## Questions?

Open a thread in [Discussions](https://github.com/patina-dev/patina/discussions). There are no bad questions.
