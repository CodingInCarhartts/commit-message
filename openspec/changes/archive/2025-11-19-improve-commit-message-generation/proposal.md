# Change: Improve Conventional Commit Message Quality

## Why
The current AI-generated commit messages sometimes result in very basic messages that lack sufficient detail and context, which is unacceptable for maintaining high-quality commit history and conventional commits compliance.

## What Changes
- Enhance the Gemini AI prompt to generate more detailed and contextually rich commit messages
- Add message quality validation to ensure messages meet minimum standards for detail and conventional commits format
- Implement fallback mechanisms for when AI generates inadequate messages

## Impact
- Affected specs: commit-message-generation
- Affected code: src/main.rs (generate_commit_message function and related logic)