<!--
SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>

SPDX-License-Identifier: GPL-3.0-only
-->

# Patina

**A static analysis engine for the patterns that accumulate when AI generates faster than humans review.**

> "Just a hobby, won't be big and professional."

---

## Philosophy

AI is powerful, not intelligent. It generates code that compiles, passes tests, and *looks* correct — but leaves behind patterns that accumulate as invisible technical debt. Patina exists to surface those patterns.

Patina is **pattern-aware, not author-aware.** It does not judge *who* wrote the code. It identifies *patterns* — regardless of origin. If a human writes slop, Patina catches it. If an AI writes clean code, Patina stays quiet. The patterns Patina detects are not AI-specific — they are long-recognized code quality anti-patterns (redundant comments, empty error handlers, god functions) that AI happens to produce at dramatically elevated rates.

Patina is not an AI tool. It uses no models, makes no API calls, and has no token budget. It is deterministic, fast, local, and free. Every finding is explainable, every rule is transparent, and every result is reproducible.

---

## Why Now

- AI-authored PRs produce **1.7x more issues** than human PRs, with readability problems **3x higher** (CodeRabbit, Dec 2025)
- Nearly **50% of all new code** is now AI-generated, and rising
- Developer trust in AI tools has dropped from **43% to 29%** in 18 months (Stack Overflow)
- **66%** of developers spend more time fixing "almost-right" AI code than writing it themselves
- Forrester predicts **75%** of tech leaders will face moderate-to-severe AI-related technical debt by 2026

The volume of AI-generated code is increasing exponentially. Even if per-file quality improves, the absolute surface area of pattern accumulation grows. As humans review less AI output, automated quality gates become more essential — not less.

---

## Core Principles

- **Deterministic** — Same input, same output. Always.
- **Transparent** — Every finding includes a rule ID, explanation, and rationale. No black boxes.
- **Fast** — Runs at the speed of disk I/O and pattern matching. No network calls. No inference.
- **Decoupled** — The core engine knows nothing about specific languages or rules. Parsers and heuristics are plugins.
- **Composable** — Rules are self-contained modules. Add, remove, or override them without touching the core.
- **Free forever** — Open source, no API keys, no billing tiers, no telemetry.

---

## Prior Art & Differentiation

The AI code quality space has several entrants. Patina must differentiate clearly.

| Tool | Approach | Patina's angle |
|------|----------|---------------|
| **antislop** | Rust + tree-sitter, slop detection, dual-mode (AST + regex) | antislop focuses on AI verbal tics (deferrals, hedging, stubs). Patina goes deeper: cargo-cult patterns, structural bloat, and style uniformity — categories antislop doesn't cover. |
| **KarpeSlop** | JS/TS focused, three-axis analysis (noise, lies, soul) | Narrow language scope. Patina's plugin architecture supports any tree-sitter grammar. |
| **sloplint** | ast-grep based, focuses on redundant comments and dead code | Narrower rule set. Patina's composable rule engine is designed for community extensibility. |
| **Semgrep** | AST pattern matching via tree-sitter + YAML rules | Semgrep excels at "find this code shape." Patina excels at statistical heuristics (token overlap, structural similarity) that pattern languages can't express. |
| **ESLint** | JS/TS rule-based linter | ESLint rules could cover some slop patterns. Patina is language-agnostic from day one and targets deeper structural patterns. |
| **SonarQube** | 6,500+ rules, 25+ languages. Had "AI Code Assurance" — **deprecated in 2026.1** | SonarQube's deprecation of AI code detection signals the "detect AI authorship" approach is a dead end. Patina detects patterns, not authorship. |

**Patina's moat is not slop detection** — that's a commodity. The moat is the breadth of analysis categories (cargo-cult, bloat, uniformity) and the composable architecture that lets the community build domain-specific rule packs.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│                   CLI Layer                      │
│            (clap - command parsing)              │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│               Core Engine                        │
│                                                  │
│  ┌───────────┐  ┌────────────┐  ┌────────────┐ │
│  │  Scanner   │  │ Rule Engine │  │  Reporter  │ │
│  │           │  │            │  │            │ │
│  │ Walks the │  │ Loads and  │  │ Formats    │ │
│  │ file tree │  │ executes   │  │ findings   │ │
│  │ and reads │  │ heuristic  │  │ as text,   │ │
│  │ source    │  │ rules      │  │ JSON, etc. │ │
│  │ files     │  │ against    │  │            │ │
│  │           │  │ parsed AST │  │            │ │
│  └─────┬─────┘  └─────┬──────┘  └─────┬──────┘ │
│        │              │               │         │
└────────┼──────────────┼───────────────┼─────────┘
         │              │               │
         ▼              ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│   Language   │ │    Rules     │ │   Output     │
│   Parsers    │ │   (Plugins)  │ │   Formats    │
│              │ │              │ │              │
│ tree-sitter  │ │ slop.rs      │ │ terminal     │
│ JS/TS        │ │ bloat.rs     │ │ json         │
│ Python       │ │ cargo_cult.rs│ │ sarif        │
│ Rust         │ │ uniformity.rs│ │ (future)     │
│ (future)     │ │ ...          │ │              │
│              │ │ (community)  │ │              │
└──────────────┘ └──────────────┘ └──────────────┘
```

---

## Components

### CLI Layer

The user-facing interface. Built with `clap`.

```
patina scan <path>              # Scan a file or directory
patina scan src/ --rule slop    # Scan with a specific rule
patina scan . --format json     # Output as JSON
patina scan . --severity warn   # Filter by severity
patina rules                    # List all available rules
patina rules --explain slop-001 # Explain a specific rule
patina init                     # Generate a .patina.toml config
```

### Scanner

Responsible for file discovery and orchestration.

- Walks the file tree, respects `.gitignore` and `.patinaignore`
- Identifies file types by extension
- Delegates to the appropriate language parser
- **Regexp prefiltering:** For rules with known trigger patterns (e.g., slop-003 checks for "Note:", "Important:"), skip files that can't possibly match before incurring the cost of an AST parse. Inspired by Semgrep's prefiltering architecture.
- Passes the parsed AST to the rule engine
- Parallelized via `rayon` for multi-core scanning. Parsers are allocated per-thread (tree-sitter parsers are not thread-safe).

### Language Parsers

Thin wrappers around tree-sitter grammars. Each parser:

- Takes raw source bytes
- Returns a tree-sitter `Tree` (the CST)
- Exposes language-specific semantic node mappings via `NodeTypeMap`

The parser trait:

```rust
pub trait LanguageParser: Send + Sync {
    fn language_id(&self) -> &str;
    fn file_extensions(&self) -> &[&str];
    fn parse(&self, source: &[u8]) -> Result<Tree>;
    fn node_types(&self) -> &NodeTypeMap;
}
```

#### NodeTypeMap

The `NodeTypeMap` bridges semantic concepts to grammar-specific node type names. Different tree-sitter grammars use different names for equivalent constructs. This mapping is the single most important abstraction for multi-language support.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticNode {
    Comment,
    LineComment,
    BlockComment,
    FunctionDeclaration,
    ArrowFunction,
    Identifier,
    StringLiteral,
    TryCatch,
    CatchClause,
    CallExpression,
    MemberExpression,
    // ...extend as rules require
}

pub type NodeTypeMap = HashMap<SemanticNode, Vec<&'static str>>;
```

Each `LanguageParser` implementation populates this map with grammar-specific node types. For example, the JavaScript parser maps `SemanticNode::Comment` to `["comment"]`, while a future Python parser would also map it to `["comment"]` — but other nodes may diverge.

**Design this mapping before writing any rules.** It determines whether adding new languages is easy or painful.

**Phase 1:** JavaScript/TypeScript via `tree-sitter-javascript` and `tree-sitter-typescript`
**Future:** Python, Rust, Go, and others via community contributions

### Rule Engine

The heart of Patina. Loads heuristic rules and executes them against parsed ASTs.

Each rule implements a trait:

```rust
pub trait Rule: Send + Sync {
    fn id(&self) -> &'static str;       // e.g., "slop-001"
    fn name(&self) -> &'static str;     // e.g., "Redundant Comment"
    fn description(&self) -> &'static str; // Human-readable explanation
    fn severity(&self) -> Severity;     // Error, Warn, Info
    fn category(&self) -> Category;     // Slop, Bloat, CargoCult, Uniformity
    fn languages(&self) -> &[&str];     // Which languages this rule applies to

    /// Accept per-rule configuration from .patina.toml.
    /// Default implementation is a no-op for rules with no config.
    fn configure(&mut self, _config: &toml::Value) -> Result<()> { Ok(()) }

    /// Optional: return a set of string patterns that MUST appear in the source
    /// for this rule to be relevant. Enables prefiltering to skip files early.
    fn prefilter_patterns(&self) -> Option<&[&str]> { None }

    fn check(&self, ctx: &RuleContext) -> Vec<Finding>;
}
```

The `RuleContext` provides raw data plus utility methods to reduce boilerplate in rules:

```rust
pub struct RuleContext<'a> {
    pub source: &'a [u8],              // Raw source bytes
    pub tree: &'a Tree,                // Parsed AST
    pub file_path: &'a Path,           // File being analyzed
    pub language: &'a str,             // Language identifier
    pub node_types: &'a NodeTypeMap,   // Language-specific node mappings
}

impl<'a> RuleContext<'a> {
    /// Execute a tree-sitter query and return matching nodes.
    pub fn query(&self, query_str: &str) -> Vec<Node<'a>>;

    /// Get the text content of a node.
    pub fn node_text(&self, node: Node) -> &'a str;

    /// Find the next named sibling of a node (skipping unnamed nodes).
    pub fn next_named_sibling(&self, node: Node) -> Option<Node<'a>>;

    /// Get all nodes matching a semantic type.
    pub fn nodes_by_type(&self, semantic: SemanticNode) -> Vec<Node<'a>>;
}
```

A `Finding` is:

```rust
pub struct Finding {
    pub rule_id: &'static str,
    pub message: String,
    pub severity: Severity,
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub span: Range<usize>,            // Byte range in source
    pub suggestion: Option<String>,    // Optional fix suggestion
}
```

Rules are registered with the engine at startup. The engine iterates over all applicable rules for a given file's language and collects findings.

### Reporter

Formats and outputs findings. Supports multiple output formats:

- **Terminal** (default) — colored, human-readable output with code snippets and line numbers. Rendered via `ariadne`, which provides multi-line labels, color coding, and the "show a code snippet with annotations" display that linters need.
- **JSON** — machine-readable for CI/CD integration
- **SARIF** (future) — Static Analysis Results Interchange Format, for GitHub Code Scanning integration

---

## Configuration

Project-level configuration via `.patina.toml` in the repo root:

```toml
[patina]
# Severity threshold: "error", "warn", "info"
severity = "warn"

# Output format: "terminal", "json"
format = "terminal"

# Paths to ignore (in addition to .gitignore)
ignore = ["vendor/", "dist/", "*.min.js"]

[rules]
# Disable specific rules
disable = ["slop-003"]

# Override severity for specific rules
[rules.severity]
"slop-001" = "error"
"bloat-002" = "info"

# Rule-specific configuration
[rules.config.slop-001]
threshold = 0.7
min_comment_words = 3
```

---

## Heuristic Categories

### Implementation Tiers

Rules are organized by implementation feasibility and false positive risk. This determines shipping order.

**Tier 1 — Ship in Phase 1** (AST-detectable, low false positive risk, straightforward):

| Rule ID   | Name                        | Description | FP Risk |
|-----------|-----------------------------|-------------|---------|
| slop-001  | Redundant Comment           | Comment restates the code it annotates with no additional insight. e.g., `// Set the name` above `setName(name)` | Medium |
| slop-003  | Apologetic Comments         | Comments containing hedging language: "Note:", "Important:", "Please note that", "It's worth noting" — common AI verbal tics. | Low |
| slop-004  | Section Banners             | Decorative comment blocks that segment code unnecessarily. e.g., `// ========== HELPER FUNCTIONS ==========` | Low |
| cult-001  | Empty Error Handler         | `try/catch` blocks where the catch does nothing, or only logs and swallows the error. | Medium |
| bloat-001 | Deep Nesting                | Functions exceeding a configurable nesting depth (default: 4 levels). | Low |

**Tier 2 — Ship in Phase 1.5** (AST-detectable, moderate complexity or FP risk):

| Rule ID   | Name                        | Description | FP Risk |
|-----------|-----------------------------|-------------|---------|
| slop-005  | Console.log Residue         | Leftover `console.log` statements that suggest unfinished debugging or AI-generated test code. | Medium |
| bloat-002 | God Function                | Functions exceeding a configurable line count or cyclomatic complexity threshold. | Low |

**Tier 3 — Defer to Phase 2+** (hard to implement accurately, higher FP risk):

| Rule ID   | Name                        | Description | FP Risk | Blocker |
|-----------|-----------------------------|-------------|---------|---------|
| slop-002  | Over-Documentation          | Excessive JSDoc/docstrings on trivial functions where the signature is self-documenting. | High | Judging function "triviality" requires a complexity heuristic |
| uniform-001 | Identical Structure       | Multiple functions sharing an eerily identical structure — same parameter count, same control flow shape, same comment placement. | High | Defining "structural similarity" precisely is research-level hard |
| uniform-002 | Template Naming           | Variable and function names following a rigid template pattern that doesn't match the rest of the codebase. | High | Needs statistical baseline of naming patterns |

**Dropped** (infeasible with tree-sitter alone):

| Rule ID   | Name                        | Why |
|-----------|-----------------------------|-----|
| cult-002  | Redundant Null Check        | Requires type information that tree-sitter cannot provide. Would need a full type checker. |
| cult-003  | Unused Abstraction          | Requires cross-file call graph analysis. Single-file detection has unacceptable FP rates. |
| bloat-003 | Premature Abstraction       | Requires project-wide call graph to determine if a function is called once. |

---

## Phase 1: Minimum Viable Tool

The goal of Phase 1 is a working CLI that can scan JavaScript/TypeScript files and report findings from Tier 1 rules.

### Deliverables

1. `patina scan <path>` works on `.js`, `.ts`, `.jsx`, `.tsx` files
2. Tree-sitter parses files into ASTs
3. All five Tier 1 rules are implemented: slop-001, slop-003, slop-004, cult-001, bloat-001
4. `patina rules` and `patina rules --explain <id>` are functional (self-documenting from day one)
5. Terminal output shows findings with file path, line number, code snippet, and explanation (via `ariadne`)
6. `.patina.toml` configuration is respected (including per-rule config)
7. `--format json` outputs machine-readable results

### Implementation Order

1. **Scaffolding:** CLI with clap, file walker with `ignore`, config loader
2. **NodeTypeMap + JS parser:** Define `SemanticNode` enum, implement JS/TS parser with node mappings
3. **Token utilities:** CamelCase/snake_case splitting, basic suffix stemming, stop-word list
4. **RuleContext + helpers:** query, node_text, next_named_sibling, nodes_by_type
5. **slop-001** (Redundant Comment) — exercises the full pipeline
6. **Reporters:** terminal (ariadne) + JSON (serde_json)
7. **slop-003** (Apologetic Comments) — easy, validates the rule plugin system
8. **slop-004** (Section Banners) — easy, builds out the slop category
9. **cult-001** (Empty Error Handler) — first non-slop rule, validates cross-category architecture
10. **bloat-001** (Deep Nesting) — simple tree traversal, completes Tier 1

### Crate Dependencies (Phase 1)

| Crate                    | Purpose                         |
|--------------------------|---------------------------------|
| `clap`                   | CLI argument parsing (derive API) |
| `tree-sitter`            | AST parsing runtime             |
| `tree-sitter-javascript` | JS grammar                      |
| `tree-sitter-typescript` | TS grammar (exposes TypeScript + TSX) |
| `rayon`                  | Parallel file scanning          |
| `ignore`                 | .gitignore-aware file walking (from ripgrep author) |
| `toml`                   | Configuration parsing           |
| `serde` + `serde_json`   | Serialization/deserialization   |
| `ariadne`                | Diagnostic rendering (code snippets with annotations) |

**Note on tree-sitter version pinning:** The `tree-sitter` core crate and all grammar crates must use matching major versions. A version mismatch causes compile errors. Pin all tree-sitter crates to a single compatible version set, and add a CI check that `cargo tree -d` shows no duplicate tree-sitter versions.

---

## Project Structure

```
patina/
├── Cargo.toml
├── .patina.toml              # Patina's own config (dog-fooding)
├── docs/
│   └── VISION.md             # This document
├── LICENSE                   # MIT

├── src/
│   ├── main.rs               # Entry point, CLI setup
│   ├── cli.rs                # Clap command definitions
│   ├── config.rs             # .patina.toml loading and validation
│   ├── scanner.rs            # File discovery and orchestration
│   ├── engine.rs             # Rule engine — loads and runs rules
│   ├── types.rs              # Shared types: Finding, Severity, Category, SemanticNode, NodeTypeMap
│   ├── tokens.rs             # Token utilities: CamelCase splitting, stemming, stop words
│
│   ├── parsers/
│   │   ├── mod.rs            # LanguageParser trait definition
│   │   ├── javascript.rs     # JS/TS tree-sitter wrapper + NodeTypeMap
│   │   └── ...               # Future language parsers
│
│   ├── rules/
│   │   ├── mod.rs            # Rule trait definition, rule registry
│   │   ├── slop/
│   │   │   ├── mod.rs        # Slop category registration
│   │   │   ├── redundant_comment.rs  # slop-001
│   │   │   ├── apologetic_comment.rs # slop-003
│   │   │   ├── section_banner.rs     # slop-004
│   │   │   └── ...
│   │   ├── cult/
│   │   │   ├── mod.rs
│   │   │   ├── empty_error_handler.rs # cult-001
│   │   │   └── ...
│   │   └── bloat/
│   │       ├── mod.rs
│   │       ├── deep_nesting.rs        # bloat-001
│   │       └── ...
│
│   └── reporters/
│       ├── mod.rs            # Reporter trait definition
│       ├── terminal.rs       # Human-readable output via ariadne
│       └── json.rs           # Machine-readable JSON output

└── tests/
    ├── fixtures/             # Sample JS/TS files with annotated expected findings
    │   ├── slop/
    │   │   ├── redundant_comments.js
    │   │   ├── apologetic_comments.js
    │   │   ├── section_banners.js
    │   │   └── ...
    │   ├── cult/
    │   │   └── empty_catch.js
    │   ├── bloat/
    │   │   └── deep_nesting.js
    │   └── clean/
    │       └── well_written.js
    └── integration/
        └── scan_test.rs
```

---

## The First Heuristic: `slop-001` — Redundant Comment

### What it detects

Comments that restate the code they annotate without adding meaningful context.

### Examples

**Flagged:**
```javascript
// Set the user's name
user.setName(name);

// Increment the counter
counter++;

// Return the result
return result;

// Loop through the items
for (const item of items) {
```

**Not flagged:**
```javascript
// Normalize Unicode before comparison to handle locale-specific equivalence
user.setName(normalizeName(name));

// Using post-increment here because the value is read before update
counter++;

// Early return — downstream expects null, not undefined
return result ?? null;
```

### Detection Strategy

1. Walk the AST, find all comment nodes (via `SemanticNode::Comment` mapping)
2. For each comment, find the immediately following sibling node (the "annotated code")
3. Extract semantic tokens from both:
   - **From the comment:** strip filler words ("the", "a", "this", etc.), normalize to lowercase, apply basic suffix stemming (strip -ing, -ed, -s, -tion)
   - **From the code:** extract identifiers and method names from the AST node, split CamelCase/snake_case into component words (e.g., `setName` -> `["set", "name"]`, `get_user_by_id` -> `["get", "user", "by", "id"]`)
4. Compute overlap: what percentage of the comment's meaningful words appear as tokens in the annotated code?
5. If overlap exceeds a configurable threshold (default: 70%), flag it

### Implementation Notes

- **CamelCase/snake_case splitting is critical.** Without it, `setName` won't match the comment word "set". Build this as a shared utility in `tokens.rs`.
- **Basic stemming, not NLP.** Strip common suffixes (-ing, -ed, -s, -tion, -ment) to catch "incrementing" vs "increment". Do not pull in a full NLP library.
- **Skip JSDoc/docstring blocks.** Comments starting with `/** @param` are slop-002's domain. Only analyze line comments and freeform block comments.
- **Comment adjacency is non-trivial in tree-sitter.** Comments can be extra nodes not part of normal parent-child structure. Handle: preceding-sibling comments, first-child-of-block comments, and inline end-of-line comments separately.
- **The 70% threshold will likely need tuning.** CamelCase splitting and stemming imprecision reduce raw overlap scores. Expect to adjust to ~60% after real-world testing. Make the threshold configurable from day one.

### Configuration

```toml
[rules.config.slop-001]
# Minimum overlap ratio to trigger (0.0 - 1.0)
threshold = 0.7
# Minimum comment length to analyze (skip very short comments)
min_comment_words = 3
```

### Fixture Test Format

Test fixtures use inline annotations to declare expected findings:

```javascript
// expect: slop-001
// Set the name
user.setName(name);

// This should NOT trigger — comment adds context
// Normalize Unicode before comparison to handle locale-specific equivalence
user.setName(normalizeName(name));
```

The test harness parses `// expect: <rule-id>` annotations, runs the scanner, and asserts that findings match exactly. This format makes it trivial to add test cases.

---

## Future Considerations

### CI/CD Integration (Phase 2)
- GitHub Action: `uses: patina-dev/patina-action@v1`
- GitLab CI template
- Pre-commit hook support
- Exit code reflects severity (0 = clean, 1 = warnings, 2 = errors)
- SARIF output for GitHub Code Scanning integration

### Trend Tracking (Phase 3)
- `patina report` generates a codebase health snapshot
- Track metrics over time: slop ratio, finding density, category distribution
- Output as Markdown or HTML for inclusion in project documentation
- Git integration: compare findings between branches or commits

### Community Rules (Phase 3+)
- Plugin architecture for external rule crates
- `patina install <crate>` to add community rule packs
- Central registry or curated list of rule packs

### Language Expansion
Priority order based on AI-assisted code volume in the wild:
1. JavaScript/TypeScript (Phase 1)
2. Python
3. Rust
4. Go
5. Java
6. Community-driven additions

### ESLint Plugin Escape Hatch
If Patina-as-standalone struggles for adoption, the detection heuristics can be packaged as an ESLint plugin or Semgrep rule pack. This is a pivot option, not the primary strategy — but the rule logic should be designed to be extractable.

---

## Development Principles

1. **Dog-food it.** Run Patina on Patina from the first commit.
2. **One rule at a time.** Ship `slop-001` end to end before starting `slop-003`.
3. **Tests are fixtures.** Every rule has corresponding sample files with `// expect:` annotations for expected findings.
4. **No false positive is acceptable without recourse.** Every rule must be configurable, disable-able, and explainable.
5. **The README is the product.** If someone can't understand what Patina does in 30 seconds, that's a bug.
6. **Design NodeTypeMap first.** This mapping determines whether multi-language support is easy or painful. Get it right before writing rules.
7. **Pin tree-sitter versions.** All tree-sitter crates must use matching major versions. CI should verify no duplicate versions exist.

---

## License

MIT — because the plumbing should be free.
