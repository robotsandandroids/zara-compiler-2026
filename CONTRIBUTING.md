# Contributing to zara

Thank you for wanting to contribute! Please follow these steps to submit your changes:

## 1. Create a branch

Always create a new branch for your work. Never commit directly to `main`.

```bash
git checkout -b <branch-name>
```

Choose a descriptive name for your branch, such as `feat/add-new-feature` or `fix/typo-correction`.

## 2. Make your changes

Implement your feature or fix. You can:

- Add new functionality
- Fix bugs
- Improve documentation
- Refactor code

Run `cargo fmt` to format your code before committing.

## 3. Commit your changes

Commit with a clear, descriptive message:

```bash
git add .
git commit -m "feat: add a short description of your change"
```

Note: The pre-commit hook will automatically run `cargo fmt --check` and `cargo clippy -- -D warnings`. If either check fails, fix the issues before committing.

## 4. Push and submit a PR

Push your branch to the remote repository and create a Pull Request:

```bash
git push origin <branch-name>
```

Then go to the GitHub page and click **New Pull Request**. Fill in the template and describe what your change does and why it's needed.

## Development

- Ensure all code passes `cargo clippy -- -D warnings`
- Keep commits focused and atomic
- Update documentation if needed