<div align="center">

# 🚀 Commit Message

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**A Rust CLI tool that generates conventional commit messages using Gemini AI**

*Automate your commit messages with AI-powered conventional commit generation*

[Installation](#-installation) •
[Setup](#-setup) •
[Usage](#-usage) •
[Features](#-features) •
[License](#-license)

</div>

---

## 📖 Overview

Commit Message is a command-line tool built in Rust that leverages Google's Gemini AI to generate high-quality, conventional commit messages based on your git history and staged changes. It follows the Conventional Commits specification to ensure consistent and meaningful commit messages.

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🤖 **AI-Powered** | Uses Gemini AI for intelligent commit message generation |
| 📋 **Conventional Commits** | Follows Conventional Commits specification |
| 📊 **Context Aware** | Analyzes commit history and staged changes |
| 🔄 **Git Integration** | Seamless integration with git workflow |
| ⚡ **Fast** | Built in Rust for maximum performance |
| 🎯 **Interactive** | Optional push confirmation |

## 📦 Installation

### From Crates.io
```bash
cargo install cm
```

### From Source
```bash
git clone https://github.com/CodingInCarhartts/commit-message
cd commit-message
cargo build --release
# Binary will be available at target/release/cm
```

## 🔧 Setup

### Prerequisites
- Google Cloud API key with Gemini API access
- Git repository

### Environment Variables
Set your Google API key:
```bash
export GOOGLE_API_KEY="your_api_key_here"
```

### API Key Setup
1. Go to [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Create a new API key
3. Set it as `GOOGLE_API_KEY` environment variable

## 🚀 Usage

### Basic Usage
```bash
# Stage your changes first
git add .

# Generate and commit
cm
```

### Workflow
1. **Check**: Verifies you're in a git repository
2. **History**: Retrieves recent commit history
3. **Staged**: Gets list of staged files
4. **Generate**: Uses Gemini AI to create commit message
5. **Commit**: Commits with generated message
6. **Push**: Optionally pushes to remote

### Interactive Mode
The tool will:
- Display commit history and staged files
- Show the generated commit message
- Ask if you want to push after committing

### Example Output
```
Commit history:
abc1234 feat: add user authentication
def5678 fix: resolve login bug

Staged files:
M src/main.rs
A src/utils.rs

Generated message: feat: implement user authentication system
Committed successfully
Do you want to push? (y/n): y
Pushed successfully
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📜 License

[MIT License](LICENSE) - See LICENSE file for details.

---

<div align="center">
  <p>Built with ❤️ in Rust using Gemini AI</p>
</div>