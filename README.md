<!--
SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>

SPDX-License-Identifier: GPL-3.0-only
-->

# Patina

**Static analysis for the patterns that accumulate when AI generates faster than humans review.**

[![REUSE status](https://api.reuse.software/badge/github.com/patina-dev/patina)](https://api.reuse.software/info/github.com/patina-dev/patina)

---

Patina detects code quality anti-patterns — redundant comments, empty error handlers, structural bloat — that AI produces at dramatically elevated rates. It doesn't detect *who* wrote the code. It detects *patterns*, regardless of origin.

No models. No API keys. No telemetry. Deterministic, fast, local, and free.

## Quick Start

```bash
# Build from source
git clone https://github.com/patina-dev/patina.git
cd patina
cargo build --release

# Scan a project
./target/release/patina scan path/to/your/project

# JSON output for CI
./target/release/patina scan path/to/your/project --format json
```

## What It Catches

Given this JavaScript:

```javascript
// Set the user name
user.setUserName(name);

// Initialize the application config
initializeApplicationConfig();
```

Patina reports:

```
Warning: [slop-001] Redundant Comment: comment restates the adjacent code
   ╭─[ src/app.js:2:1 ]
   │
 2 │ // Set the user name
   │ ──────────┬─────────
   │           ╰─────────── Remove this comment — it restates the code without adding context.
───╯
```

These comments add nothing that the code doesn't already say. They're noise that slows down every reviewer who reads them.

## What It Doesn't Catch

Comments that add context beyond the code are left alone:

```javascript
// Normalize Unicode before comparison to handle locale-specific equivalence
user.setName(normalizeName(name));

// Using post-increment here because the value is read before update
counter++;

// TODO: Replace with a proper LRU cache when we exceed 10k entries
const cache = new Map();
```

These explain *why*, not *what*. Patina knows the difference.

## How It Works

1. Walks your file tree (respects `.gitignore`)
2. Parses JS/TS files into ASTs using [tree-sitter](https://tree-sitter.github.io/)
3. For each comment, extracts meaningful tokens (strips stop words, stems suffixes, splits `camelCase`/`snake_case` identifiers)
4. Computes overlap between comment tokens and adjacent code tokens
5. If ≥70% of the comment's words are just restating the code, it's flagged

The full algorithm is documented in [`docs/slop-001/SPEC.md`](docs/slop-001/SPEC.md).

## Rules

| Rule | Name | What it detects |
|------|------|-----------------|
| `slop-001` | Redundant Comment | Comments that restate adjacent code without adding context |

More rules are planned. See [`docs/VISION.md`](docs/VISION.md) for the roadmap.

## Supported Languages

- JavaScript (`.js`, `.jsx`)
- TypeScript (`.ts`, `.tsx`)

More languages are planned — the architecture supports any language with a tree-sitter grammar.

## Output Formats

| Format | Flag | Use case |
|--------|------|----------|
| Terminal | `--format terminal` (default) | Human-readable with annotated code snippets |
| JSON | `--format json` | Machine-readable for CI/CD pipelines |

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | No findings |
| `1` | Findings detected |

## Philosophy

- **Pattern-aware, not author-aware.** Patina doesn't judge who wrote the code. If a human writes a redundant comment, it gets flagged. If an AI writes clean code, it stays quiet.
- **Deterministic.** Same input, same output. Always.
- **Transparent.** Every finding has a rule ID, an explanation, and a configurable threshold.
- **Free forever.** Open source, no billing tiers, no telemetry.

## Contributing

See [`CONTRIBUTING.md`](CONTRIBUTING.md). Whether it's a bug report, a false positive, a new rule idea, or a typo fix — contributions are welcome.

## License

[GPL-3.0-only](LICENSE) — because the tooling that keeps code honest should stay honest itself.
