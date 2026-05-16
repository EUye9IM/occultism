# AGENTS.md - Occultism

## Commands

```bash
# Run all tests
cargo test --workspace

# Run a single test (e.g., in shushu)
cargo test -p shushu -- bazi::tests::known_date_2026_04_27

# Lint
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --check --all

# Auto-format
cargo fmt --all
```

Note: `cargo fmt` uses `--all`, not `--workspace` (which is invalid for fmt).

## Architecture

- **Cargo workspace** with 3 crates, `edition = "2024"` (requires Rust >= 1.85).
- **`shushu`** — core numerological types: `Yinyang`, `Wuxing`, `Tiangan`, `Dizhi`, `Jieqi`, `Shichen`, `Pillar`, `Bazi`. Has external dep: `time = "0.3"`.
- **`daliuren`** — placeholder. Currently just `pub use shushu;`. No tests.
- **`occultism`** (root crate, `src/lib.rs`) — re-exports shushu types and daliuren. No tests.
- All real logic and tests live in `shushu/`.

## 交互要求

- Thinking思考过程用中文表述
- Reply回答也要用中文回复

## Style & Conventions

- `edition = "2024"` — use postfix `.await`, `unsafe extern`, etc.
- Inline test modules: `#[cfg(test)] mod tests { ... }` in each source file.
- Chinese domain vocabulary in identifiers and display strings (e.g., `Jieqi::Lichun` → `"立春"`). This is intentional; do not translate.
- No CI, no pre-commit hooks, no custom clippy config. Codebase is small and manually verified.
