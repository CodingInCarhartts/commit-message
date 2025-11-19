# Project Context

## Purpose
Commit Message is a Rust CLI tool that generates conventional commit messages using Google's Gemini AI. It automates the creation of high-quality, standardized commit messages based on git history and staged changes, following the Conventional Commits specification.

## Tech Stack
- Rust (2024 edition)
- Tokio (async runtime)
- Reqwest (HTTP client)
- Serde/Serde JSON (serialization)
- Google Gemini AI API

## Project Conventions

### Code Style
- Follow Rust standard formatting (rustfmt)
- Use snake_case for functions and variables
- Use PascalCase for structs and enums
- Error handling with Result types
- Async functions where appropriate

### Architecture Patterns
- CLI application with modular functions
- Async/await for API calls
- Command pattern for git operations
- JSON serialization for API communication

### Testing Strategy
- Unit tests for core functions
- Integration tests for git operations
- Mock API responses for testing

### Git Workflow
- Conventional Commits specification
- Feature branches for development
- Pull requests for code review
- Automated commit message generation

## Domain Context
- Conventional Commits: Standardized commit message format with types (feat, fix, docs, etc.)
- Git integration: Analyzes commit history and staged changes
- AI-powered generation: Uses Gemini to create contextually appropriate messages

## Important Constraints
- Requires Google Cloud API key for Gemini access
- Must be run in a git repository
- Network connectivity needed for API calls

## External Dependencies
- Google Gemini API (generativelanguage.googleapis.com)
- Git command-line tool
