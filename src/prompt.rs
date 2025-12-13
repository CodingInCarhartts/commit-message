/// Build the prompt for AI commit message generation
pub fn build_commit_prompt(diff_content: &str, commit_history: &str, diff_stat: &str) -> String {
    format!(
        r#"You are an expert at writing clear, professional git commit messages following the Conventional Commits specification.

## Your Task
Generate a commit message for the staged changes shown below.

## Conventional Commits Specification
{}

## Requirements

### Subject Line (REQUIRED)
- Format: `type(scope): description` or `type: description`
- Types: feat, fix, docs, style, refactor, test, chore, perf, ci, build, revert
- Scope: optional, describes the affected component (e.g., auth, api, ui)
- Description: imperative mood, lowercase, no period at end, max 72 chars
- Be specific! Avoid vague words like "update", "fix issue", "changes"

### Body (OPTIONAL but recommended for complex changes)
- Explain WHAT changed and WHY (not HOW - the code shows that)
- Wrap at 72 characters
- Use bullet points for multiple changes

## Context

### Recent Commit History (for style reference)
```
{}
```

### Change Statistics
```
{}
```

### Actual Diff Content
```diff
{}
```

## Response Format
Respond in EXACTLY this format (no markdown, no extra text):

SUBJECT: <your subject line here>
BODY: <your body here, or just "none" if not needed>

Generate the commit message now:"#,
        include_str!("../conventional_commits.txt"),
        if commit_history.is_empty() { "(no previous commits)" } else { commit_history },
        diff_stat,
        diff_content,
    )
}
