<!--
SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>

SPDX-License-Identifier: GPL-3.0-only
-->

# slop-001: Redundant Comment

## What It Detects

Comments that restate the adjacent code without adding meaningful context. These are the most common form of AI-generated comment noise — syntactically correct but semantically empty.

## Detection Algorithm

1. **Walk the AST** using a tree-sitter `TreeCursor`
2. **Find comment nodes** (node kind `"comment"`)
3. **Skip non-candidates:**
   - JSDoc blocks (`/** ... */`) — deferred to slop-002
   - TODO/FIXME/eslint-disable directives
   - Comments with fewer than `MIN_COMMENT_WORDS` (default: 3) meaningful words
4. **Find the annotated code:** the next named sibling of the comment node in the AST
5. **Extract tokens from the comment:**
   - Strip comment markers (`//`, `/*`, `*/`)
   - Split on whitespace
   - Remove stop words (the, a, an, this, that, to, of, in, for, is, it, be, as, with)
   - Apply basic suffix stemming (-ing, -ed, -s, -tion, -ment, -ly)
   - Lowercase all tokens
6. **Extract tokens from the code:**
   - Recursively collect all `identifier` and `property_identifier` nodes from the sibling subtree
   - Split CamelCase and snake_case identifiers into component words (`setUserName` → `[set, user, name]`)
   - Apply the same stemming and lowercasing
7. **Compute overlap:**
   ```
   overlap = |comment_tokens ∩ code_tokens| / |comment_tokens|
   ```
8. **If overlap ≥ `OVERLAP_THRESHOLD` (default: 0.7), emit a finding**

## Constants

| Name | Default | Description |
|------|---------|-------------|
| `OVERLAP_THRESHOLD` | 0.7 | Minimum ratio of comment tokens found in code tokens to trigger |
| `MIN_COMMENT_WORDS` | 3 | Minimum meaningful words in a comment before analysis applies |

## Examples

### Flagged

```javascript
// Set the user's name
user.setName(name);
// Comment tokens: [set, user, name]
// Code tokens:    [user, set, name, name]
// Overlap: 3/3 = 1.0 → FLAGGED

// Increment the counter
counter++;
// Comment tokens: [increment, counter]  (after stemming "increment")
// Code tokens:    [counter]
// Overlap: 1/2 = 0.5 → NOT flagged (but close — "increment" doesn't appear in code)

// Loop through the items
for (const item of items) {
// Comment tokens: [loop, through, item]
// Code tokens:    [item, items]  (after stemming "items" → "item")
// Overlap: 1/3 = 0.33 → NOT flagged
```

Wait — "increment" requires special handling. Let's be precise:

```javascript
// Increment the counter
counter++;
// "Increment" stems to "increment", "counter" stays "counter"
// Code tokens from `counter++`: identifier "counter" → [counter]
// Overlap: 1/2 = 0.5 → NOT flagged
```

```javascript
// Get the user name
const userName = getUserName();
// Comment tokens: [get, user, name]
// Code tokens from identifiers: userName → [user, name], getUserName → [get, user, name]
// Overlap: 3/3 = 1.0 → FLAGGED
```

### Not Flagged

```javascript
// Normalize Unicode before comparison to handle locale-specific equivalence
user.setName(normalizeName(name));
// Comment tokens: [normal, unicod, comparison, handl, local, specif, equival]
// Code tokens: [user, set, name, normal, name, name]
// Overlap: 1/7 = 0.14 → NOT flagged

// Using post-increment here because the value is read before update
counter++;
// Comment tokens: [use, post, increment, valu, read, updat]
// Code tokens: [counter]
// Overlap: 0/6 = 0.0 → NOT flagged

// Early return — downstream expects null, not undefined
return result ?? null;
// Comment tokens: [earli, return, downstream, expect, null, undefin]
// Code tokens: [result, null]  (null may or may not appear as identifier)
// Overlap: 1/6 = 0.17 → NOT flagged
```

## Edge Cases

1. **Inline comments** (`x = 1; // set x`): The comment node's next sibling is the *next* statement, not the current line. Use the comment's parent context when the comment appears at the end of a statement.

2. **JSDoc blocks** (`/** @param ... */`): Skip entirely. These are structured documentation, not freeform comments. Detection: comment text starts with `/**` or contains `@param`, `@returns`, `@type`, etc.

3. **Multi-line comments**: Join all lines before token extraction. Strip `*` line prefixes from block comment interiors.

4. **Directive comments**: Skip comments matching patterns like `TODO`, `FIXME`, `HACK`, `eslint-disable`, `@ts-ignore`, `prettier-ignore`, `istanbul ignore`.

5. **Very short comments**: Comments with fewer than `MIN_COMMENT_WORDS` meaningful words are skipped. A comment like `// x` or `// done` is too terse to meaningfully evaluate for redundancy.

6. **No adjacent code**: If a comment has no next named sibling (e.g., it's the last node in a block), skip it.

## Severity

**Warning** — redundant comments are noise but not bugs. They degrade readability without causing runtime issues.

## Suggestion

When a finding is emitted, the suggestion is: `"Remove this comment — it restates the code without adding context."`
