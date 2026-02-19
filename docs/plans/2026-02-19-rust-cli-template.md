# Rust CLI Grep Template Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a Rust workspace template for a grep-like CLI with colorful output and TDD coverage.

**Architecture:** Workspace with `core` for search/highlight logic and `cli` for argument parsing and IO. The CLI calls into core and renders structured results to terminal output.

**Tech Stack:** Rust 2024, clap, regex, owo-colors, derive_more.

---

### Task 1: Scaffold workspace and crates

**Files:**
- Create: `Cargo.toml`
- Create: `crates/core/Cargo.toml`
- Create: `crates/core/src/lib.rs`
- Create: `crates/cli/Cargo.toml`
- Create: `crates/cli/src/main.rs`
- Create: `crates/cli/src/cli/cmd.rs`
- Create: `crates/cli/src/cli/executor.rs`
- Create: `crates/cli/src/handlers/search.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn smoke_compiles() -> Result<()> {
    Ok(())
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p core`
Expected: FAIL with missing crate layout

**Step 3: Write minimal implementation**

```rust
pub fn ping() -> &'static str {
    "ok"
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p core`
Expected: PASS

**Step 5: Commit**

Skip commit unless user explicitly requests.

---

### Task 2: Core literal search with line numbers

**Files:**
- Modify: `crates/core/src/lib.rs`
- Create: `crates/core/src/error.rs`
- Create: `crates/core/src/search.rs`
- Create: `crates/core/src/model.rs`
- Test: `crates/core/src/search.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn finds_literal_matches() -> Result<()> {
    let input = "alpha\nbeta\nalp";
    let matches = search_literal(input, "alp")?;
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0].line_number, 1);
    assert_eq!(matches[1].line_number, 3);
    Ok(())
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p core`
Expected: FAIL with missing functions/types

**Step 3: Write minimal implementation**

```rust
pub fn search_literal(input: &str, pattern: &str) -> Result<Vec<MatchLine>> {
    // minimal implementation to satisfy test
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p core`
Expected: PASS

**Step 5: Commit**

Skip commit unless user explicitly requests.

---

### Task 3: Regex search and case-insensitive option

**Files:**
- Modify: `crates/core/src/search.rs`
- Modify: `crates/core/src/model.rs`
- Test: `crates/core/src/search.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn finds_regex_matches_ignore_case() -> Result<()> {
    let input = "Alpha\nbeta\nALP";
    let matches = search_regex(input, "alp", true)?;
    assert_eq!(matches.len(), 2);
    Ok(())
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p core`
Expected: FAIL with missing function/behavior

**Step 3: Write minimal implementation**

```rust
pub fn search_regex(input: &str, pattern: &str, ignore_case: bool) -> Result<Vec<MatchLine>> {
    // minimal implementation to satisfy test
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p core`
Expected: PASS

**Step 5: Commit**

Skip commit unless user explicitly requests.

---

### Task 4: Highlighting with colorful output model

**Files:**
- Modify: `crates/core/src/model.rs`
- Create: `crates/core/src/render.rs`
- Modify: `crates/core/src/lib.rs`
- Test: `crates/core/src/render.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn highlights_matches_with_colors() -> Result<()> {
    let line = "alpha beta";
    let rendered = highlight_line(line, &[0..5])?;
    assert!(rendered.contains("\u{1b}["));
    Ok(())
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p core`
Expected: FAIL with missing function

**Step 3: Write minimal implementation**

```rust
pub fn highlight_line(line: &str, spans: &[std::ops::Range<usize>]) -> Result<String> {
    // minimal implementation to satisfy test
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p core`
Expected: PASS

**Step 5: Commit**

Skip commit unless user explicitly requests.

---

### Task 5: CLI parsing and handler integration

**Files:**
- Modify: `crates/cli/src/cli/cmd.rs`
- Modify: `crates/cli/src/cli/executor.rs`
- Modify: `crates/cli/src/handlers/search.rs`
- Modify: `crates/cli/src/main.rs`
- Test: `crates/cli/tests/cli_search.rs`

**Step 1: Write the failing test**

```rust
#[test]
fn cli_reads_stdin_and_outputs_matches() -> Result<()> {
    let output = run_cli_with_stdin("alp", "alpha\nbeta\n")?;
    assert!(output.contains("alpha"));
    Ok(())
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p cli`
Expected: FAIL with missing CLI wiring

**Step 3: Write minimal implementation**

```rust
pub fn run(args: Cmd) -> Result<i32> {
    // wire CLI to core
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test -p cli`
Expected: PASS

**Step 5: Commit**

Skip commit unless user explicitly requests.

---

### Task 6: Workspace-wide verification

**Files:**
- No file changes

**Step 1: Run tests**

Run: `cargo test`
Expected: PASS

**Step 2: Run formatting**

Run: `cargo fmt --all`
Expected: PASS

**Step 3: Run lint**

Run: `cargo clippy --all-targets --all-features -- -D warnings`
Expected: PASS

---

### Constraints

- Do not add code comments unless explicitly requested.
- Commit only if explicitly requested.
- rust10x reference docs are not present in the repository; follow the rules directly.
