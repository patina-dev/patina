# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build                      # Debug build
cargo build --release            # Release build
cargo test                       # All tests (unit + integration)
cargo test --lib                 # Unit tests only (tokens.rs)
cargo test --test integration    # Integration tests only
cargo test <test_name>           # Single test by name
cargo clippy                     # Lint
```

## Running

```bash
cargo run -- scan <path>                    # Scan with terminal output
cargo run -- scan <path> --format json      # JSON output
```

Exit code 0 = no findings, 1 = findings detected.

## Architecture

The pipeline flows linearly:

```
CLI (cli.rs) → Scanner (scanner.rs) → Parser (parsers/) → Engine (engine.rs) → Rules (rules/) → Reporter (reporters/)
```

- **scanner.rs** — walks files with the `ignore` crate (respects `.gitignore`), filters to `.js/.jsx/.ts/.tsx`
- **parsers/javascript.rs** — wraps tree-sitter grammars. `parser_for_extension()` maps file extensions to the correct parser. Each `parse()` call creates a fresh `tree_sitter::Parser` internally
- **engine.rs** — holds registered rules, iterates them over each (source, tree, path), collects and sorts findings
- **rules/mod.rs** — `all_rules()` returns every active rule. New rules must be added here
- **reporters/** — `terminal.rs` uses ariadne for annotated code snippets; `json.rs` serializes findings via serde_json
- **tokens.rs** — shared utilities for identifier splitting (CamelCase/snake_case), suffix stemming, stop word filtering, and token extraction from comments and code
- **types.rs** — `Finding`, `Severity`, `Category` structs shared across the codebase

## Adding a New Rule

1. Create `src/rules/<category>/<rule_name>.rs`
2. Implement the `Rule` trait (`id`, `name`, `description`, `severity`, `check`)
3. Add `pub mod <rule_name>;` in the category's `mod.rs`
4. Register in `rules::all_rules()` with `Box::new(YourRule)`
5. Create test fixtures in `tests/fixtures/<category>/` with `// expect: <rule-id>` annotations
6. Add SPDX header to all new files (`reuse annotate`)

## slop-001 Detection Algorithm

The core heuristic in `rules/slop/redundant_comment.rs`:

1. Walk the AST with `TreeCursor`, find `"comment"` nodes
2. Skip: JSDoc (`/**`), directives (TODO/FIXME/eslint-disable/@ts-ignore), comments with <3 meaningful words
3. Find adjacent code via `next_named_sibling()`, skipping other comments
4. Collect `"identifier"` and `"property_identifier"` nodes from the code subtree
5. Extract tokens from both (strip markers, remove stop words, stem suffixes, split identifiers)
6. If `|comment_tokens ∩ code_tokens| / |comment_tokens| >= 0.7`, emit a finding

## Test Fixture Convention

Fixtures use `// expect: <rule-id>` on the line immediately before the comment that should be flagged:

```javascript
// expect: slop-001
// Set the user name
user.setUserName(name);
```

The `expect_annotations_match_findings` integration test parses these annotations and verifies they match actual findings exactly. When adding SPDX headers or modifying fixtures, line numbers in hardcoded assertions (`scan_test.rs`) must be updated.

## Tree-sitter Version Pinning

All tree-sitter crates must resolve to compatible major versions. After adding or updating any tree-sitter dependency, run `cargo tree -d` to verify no duplicate `tree-sitter` core versions appear. The grammar crates expose `LanguageFn` constants (e.g., `tree_sitter_javascript::LANGUAGE`), which must be converted via `Language::from()` before passing to `Parser::set_language()`.

## License & Headers

GPL-3.0-only. All files must carry SPDX headers. Run `reuse lint` to verify compliance. Use `reuse annotate --copyright "Steven Mosley <astrosteveo>" --license "GPL-3.0-only" --year 2026` for new files.
