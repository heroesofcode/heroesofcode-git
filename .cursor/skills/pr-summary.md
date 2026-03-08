# PR Summary Skill

## Description

Reviews local git changes and generates a pull request summary following the project template.

## Steps

1. **Gather change context** by running the following git commands:
   - `git diff --stat HEAD` — list changed files and line counts
   - `git diff HEAD` — full diff to understand what changed
   - `git log --oneline origin/main..HEAD` (or `origin/master..HEAD`) — list commits in this branch

2. **Infer type of change** from the diff and file list:
   - Source code changes in `src/` → likely Enhancement, Bug fix, New feature, or Refactor
   - Changes to `CHANGELOG.md` or version files → likely New release
   - Changes to `*.md` files only → likely Documentation
   - Changes related to security or auth logic → likely Security fix
   - Changes that remove or modify public API signatures → likely Breaking change
   - Changes to CI/build configuration → likely Enhancement or Refactor

3. **Write the summary** as concise topic bullets describing *what* changed and *why*.

4. **Output the result** in the project PR template format:

```markdown
## ✨ Summary

- <bullet describing what changed>
- <bullet describing motivation or impact, if relevant>

## 🔧 Type of Change

- [ ] ✨ Enhancement
- [ ] 🐞 Bug fix
- [ ] 🔐 Security fix
- [ ] 💥 Breaking change
- [ ] 🚀 New feature
- [ ] 📦 New release
- [ ] 📚 Documentation
- [ ] ♻️ Refactor
```

   Mark the appropriate checkbox(es) with `[x]` based on the inferred type.

5. **Deliver the result** wrapped in a markdown code block in the chat response, and write the same content to `pr-description.md` in the repository root.
