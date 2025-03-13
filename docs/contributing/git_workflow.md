# Git Workflow Guidelines

This document outlines the recommended git workflow for contributing to the Rummage project, with a special focus on writing clear, informative commit messages.

## Commit Message Guidelines

Good commit messages are essential for maintaining a clean, understandable project history. They help with:

- Understanding changes at a glance
- Generating accurate changelogs
- Tracking the evolution of features and bug fixes
- Making code reviews more efficient
- Facilitating future maintenance and debugging

### Commit Message Structure

Each commit message should follow this structure:

```
<type>: <subject>

<body>

<footer>
```

### Type

Begin your commit message with one of these types to categorize the change:

| Type | Description |
|------|-------------|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation changes |
| `style` | Code style changes (formatting, indentation) |
| `refactor` | Code refactoring (no functional changes) |
| `perf` | Performance improvements |
| `test` | Adding or updating tests |
| `chore` | Maintenance tasks, build changes, etc. |
| `ci` | CI/CD related changes |

### Subject

The subject line is a brief summary of the change:

- Write in imperative mood (e.g., "Add" not "Added" or "Adds")
- Keep it under 50 characters
- Don't end with a period
- Capitalize the first letter

### Body

The commit body provides more detailed information:

- Separate from subject with a blank line
- Explain the "what" and "why" (not "how")
- Use bullet points when appropriate (- or *)
- Wrap text at ~72 characters
- Reference issues and user stories when applicable

### Footer

The footer contains metadata about the commit:

- Reference issues with "Fixes #123" or "Closes #123"
- Breaking changes should be noted with "BREAKING CHANGE:"

## Examples

Here are examples of well-formatted commit messages:

```
feat: Add user authentication system

Implement JWT-based authentication flow with refresh tokens.
- Add login/logout endpoints
- Create token generation service
- Add middleware for protected routes

Closes #45
```

```
fix: Prevent race condition in database connection pool

When multiple requests arrived simultaneously during
initialization, connections were being created beyond
the configured maximum. Added mutex lock to ensure
proper counting.

Fixes #123
```

```
docs: Update API documentation with authentication examples

Add code samples for all authentication flows and
improve explanations of token usage.
```

## Best Practices

### Commit Organization

- When possible, limit commits to a single logical change
- Small, focused commits are easier to understand and review
- Avoid mixing multiple unrelated changes in a single commit
- When incorporating feedback from PR reviews, reference the PR number

### Working with Branches

1. Create feature branches from the main branch
   ```bash
   git checkout -b feature/my-new-feature main
   ```

2. Make regular, focused commits
   ```bash
   git commit -m "feat: Add validation for user input fields"
   ```

3. Rebase your branch on main before submitting a PR
   ```bash
   git checkout main
   git pull
   git checkout feature/my-new-feature
   git rebase main
   ```

4. Squash commits if needed to maintain a clean history
   ```bash
   git rebase -i HEAD~3  # Squash last 3 commits
   ```

### Pull Request Guidelines

- Ensure PR title follows the same commit message format
- Include a detailed description of changes
- Reference related issues
- Ensure all tests pass
- Request reviews from appropriate team members

## Git Configuration Tips

You can configure git to use your preferred editor for commit messages:

```bash
git config --global core.editor "code --wait"  # For VS Code
```

For multi-line commit messages, use:

```bash
git commit  # Opens editor for detailed message
```

## Conclusion

Following these commit message guidelines helps maintain a high-quality codebase with a clear, navigable history. Good commit messages are a form of documentation that helps future contributors (including your future self) understand the evolution of the project. 