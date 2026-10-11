# AGENTS.md

Strict operational instructions for all AI agents working on Pumpkin-Duels.

Every agent MUST adhere to these rules without exception. Deviations, shortcuts, or temporary workarounds will be rejected.

---

## 1. Core Mandates

- Simplicity First: Implement the most straightforward, minimal solution that completely solves the problem. Never introduce speculative abstractions, unnecessary wrappers, or premature generalizations.
- Root Cause Only: Always locate and resolve the underlying issue. Never apply patches, monkey-patches, sleeps, or workarounds.
- Zero Regressions: Existing commands, configurations, permissions, and duel workflows MUST continue functioning as designed.
- Never Add Comments: Do NOT write comments in source code unless explicitly requested by the user. Code must be self-explanatory through naming and structure.
- Never Use Emojis: Absolutely zero emojis in documentation, code, log output, or commit messages.

---

## 2. Rust Code Standards

- No Unused Code: Zero unused imports, variables, functions, structs, fields, or dead code. Remove all dead code immediately.
- Compiler Warnings: The workspace MUST compile with zero compiler warnings and zero errors.
- Explicit Error Handling: Never use `.unwrap()` or `.expect()` in production runtime paths where a fallible error can occur. Handle all errors explicitly with `Result`, `Option`, or proper error propagation.
- Dependencies: Do NOT add new crates or external dependencies without explicit user instruction. Use existing workspace dependencies.
- Formatting: Follow standard Rust idioms and `rustfmt` formatting.

---

## 3. Architecture & Crates

The repository is organized into distinct crates. Preserve these boundaries:

- `duels-core`: Pure duel logic (queue, matchmaker, match state machine, combat tracking, arena models, kits, challenges). MUST remain decoupled from WASM runtime APIs to enable native unit testing.
- `duels-storage`: SQLite persistence layer (`rusqlite`). Manages schema migrations, kits, arenas, match history, and player statistics.
- `duels-config`: Configuration engine managing `config.toml` and `messages.toml` parsing, reconciliation of missing keys, and thread-safe hot reloading.
- `duels-gui`: Inventory menus, dialog actions, item representations, and interactive screens.
- `pumpkin-duels`: Root plugin crate compiling to `wasm32-wasip2`. Bridges Pumpkin server plugin APIs (`pumpkin_plugin_api`), command dispatch, event listeners, and scheduled tickers.

---

## 4. WASM & Concurrency Rules

- Compilation Target: The plugin crate must strictly compile for `wasm32-wasip2`.
- Cooperative Execution: Pumpkin plugins operate under the WASI component model. Do NOT attempt OS-level thread spawning or blocking I/O calls that violate WASI P2 capabilities.
- Thread Safety: Protect shared runtime state using `Arc<Mutex<T>>` or `Arc<RwLock<T>>`. Keep lock contention minimal and never hold mutex locks across slow or external operations.
- Scheduled Tasks: Use `Context::schedule_repeating_task` for match ticks and queue updates. Ensure ticks run efficiently every 1 tick (20 TPS).

---

## 5. Minecraft In-Game Text & MiniMessage Rules

- Color Format: Use MiniMessage color tags (e.g., `<color:#A0A0A0>text</color>`, `<color:#00E262>success</color>`).
- No Bold or Italic: Strictly forbid `<b>`, `<bold>`, `<i>`, `<italic>` styling in messages. All text must be clean and unstyled aside from color.
- Unicode Symbols: Limited, tasteful unicode glyphs are permitted (e.g., `❤`, `⚔`, `»`) where explicitly configured. Do not clutter chat with excessive symbols.
- Nametag & Belowname Health: Keep heart displays synchronized with player health. Use accurate color indicators for absorption (`#FFEE2E`), wither (`#303030`), poison (`#2DAD48`), and standard damage.

---

## 6. Console Logging Rules

- Keep Logs Compact: Startup, reload, and shutdown console messages must be concise (1 to 2 lines max). Never output large ASCII art banners, command lists, or wall-of-text diagnostics.
- Log Levels:
  - `tracing::info!` for key lifecycle milestones (startup summary, reload, shutdown).
  - `tracing::warn!` for actionable administrative issues (e.g., missing arenas, missing kits).
  - `tracing::error!` for actionable failures (database connection failures, permission registration failures).

---

## 7. Mandatory Verification Checklist

Before reporting completion or declaring any task finished, the agent MUST run and verify:

1. Native Unit Tests:
   `cargo test --workspace --target x86_64-unknown-linux-gnu`
   Must pass with 0 failures.

2. WebAssembly Release Build:
   `cargo build --release --target wasm32-wasip2`
   Must complete with code 0 and zero warnings.

3. Documentation Build (if docs or markdown were touched):
   `npm run docs:build`
   Must build successfully with VitePress.

4. Workspace Cleanliness:
   No unrelated modified files, untracked artifacts, or temporary files left behind.

---

## 8. Git Commit Guidelines

- Format: Conventional commit format (`feat: ...`, `fix: ...`, `refactor: ...`, `docs: ...`).
- Message Style: Clear, lowercase, imperative, describing the exact root change.
- No Emojis: Never include gitmoji or emoji icons in commit subjects or bodies.
