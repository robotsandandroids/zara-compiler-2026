# zara

A Rust project.

## Git Hooks

This project uses a pre-commit hook that runs `cargo fmt` and `cargo clippy` on every commit to ensure code quality.

The hook is located at `.git/hooks/pre-commit` and performs the following checks:

1. **`cargo fmt --check`** - Verifies the code is properly formatted
2. **`cargo clippy -- -D warnings`** - Runs the linter with all warnings treated as errors

If either check fails, the commit is aborted. To commit after making changes, run `cargo fmt` first, then `git add` the formatted files, then `git commit`.

## License

This project is licensed under the [Unlicense](LICENSE).