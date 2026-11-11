# Commit Guidelines

- Concise, one-line messages (multi-line only when many changes)
- Group related files logically
- No emojis
- One feature, one commit. Don't merge phases; it keeps the history walkable
- Use `git diff` to understand changes before committing
- **Never** include AI-agent signatures in commits
    - No "Co-Authored-By: Claude..."
    - No "Generated with [Claude Code]..."
    - No "Written with the help of ..."

## Before you push

There's no CI yet. When it lands, copy steller's `.github/workflows/ci.yml`, `rustfmt.toml`, `clippy.toml` and the `[lints]` block (the scadoshi-rust set). Until then, run the same gate by hand:

```bash
cargo +nightly fmt                       # not `cargo fmt`, once rustfmt.toml has the unstable options
cargo clippy --all-targets -- -D warnings
cargo test
```
