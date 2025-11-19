use std::process::Command;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if in git repo
    if !is_git_repo() {
        eprintln!("Not in a git repository");
        return Ok(());
    }

    // Configuration
    let min_length: usize = std::env::var("COMMIT_MSG_MIN_LENGTH").unwrap_or_else(|_| "20".to_string()).parse().unwrap_or(20);

    // Get commit history
    let history = get_commit_history()?;
    println!("Commit history:\n{}", history);

    // Get staged files
    let staged = get_staged_files()?;
    println!("Staged files:\n{}", staged);

    // Generate commit message using Gemini
    let mut message = generate_commit_message(&history, &staged).await?;
    let mut attempts = 1;
    while !validate_commit_message(&message, min_length) && attempts < 3 {
        eprintln!("Generated message failed quality check, regenerating... (attempt {})", attempts + 1);
        message = generate_commit_message(&history, &staged).await?;
        attempts += 1;
    }
    if !validate_commit_message(&message, min_length) {
        eprintln!("Failed to generate a quality commit message after {} attempts. Please check your staged changes or try again.", attempts);
        return Ok(());
    }
    println!("Generated message: {}", message);

    // Commit
    commit(&message)?;

    // Ask to push
    if ask_push()? {
        push()?;
    }

    Ok(())
}

fn is_git_repo() -> bool {
    Command::new("git")
        .arg("rev-parse")
        .arg("--git-dir")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn get_commit_history() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg("-10")
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err("Failed to get commit history".into())
    }
}

fn get_staged_files() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--name-status")
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err("Failed to get staged files".into())
    }
}

async fn generate_commit_message(history: &str, staged: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = std::env::var("GOOGLE_API_KEY")?;
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-lite-latest:generateContent?key={}", api_key);

    let prompt = format!(
        "Based on the following Conventional Commits specification and the provided git commit history and staged changes, generate a single, high-quality commit message that follows the specification exactly.\n\nKey requirements for the commit message:\n- Strictly follow Conventional Commits format: type(scope): description\n- Include a scope in parentheses if the change affects a specific component (e.g., feat(auth): add login validation)\n- Provide a detailed, specific description that explains what changed and why, avoiding generic terms like 'update' or 'fix issue'\n- Ensure the message is informative and provides context for future developers\n- Keep the description concise but meaningful (aim for 50-100 characters)\n- If the change is a breaking change, mark it with ! and include BREAKING CHANGE in the body if needed\n\nConventional Commits Specification:\n{}\n\nRecent Commit History (for style and context):\n{}\n\nStaged Changes (analyze these to understand what was modified):\n{}\n\nGenerate only the commit message, nothing else. Make it detailed and professional.",
        include_str!("../conventional_commits.txt"),
        history,
        staged
    );

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let client = reqwest::Client::new();
    let response = client.post(&url)
        .json(&body)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let message = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or("Failed to parse response")?
        .trim()
        .to_string();

    Ok(message)
}

fn validate_commit_message(message: &str, min_length: usize) -> bool {
    // Check minimum length for meaningful messages
    if message.len() < min_length {
        return false;
    }

    // Check conventional commits format: type(scope): description
    let types = ["feat", "fix", "docs", "style", "refactor", "test", "chore", "perf", "ci", "build", "revert"];
    let mut valid = false;
    for t in &types {
        if message.starts_with(&format!("{}: ", t)) || message.starts_with(&format!("{}!", t)) {
            valid = true;
            break;
        }
        // Check for scope
        if let Some(colon_pos) = message.find(':') {
            let prefix = &message[..colon_pos];
            if prefix.starts_with(&format!("{}(", t)) && prefix.ends_with(')') {
                valid = true;
                break;
            }
            if prefix.starts_with(&format!("{}!(", t)) && prefix.ends_with(')') && prefix.contains('!') {
                valid = true;
                break;
            }
        }
    }
    if !valid {
        return false;
    }

    // Check for generic terms that indicate low quality
    let generic_terms = ["update", "fix issue", "change", "modify", "improve"];
    for term in &generic_terms {
        if message.to_lowercase().contains(term) && message.len() < 50 {
            return false; // If short and contains generic term, likely low quality
        }
    }

    true
}

fn commit(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .status()?;
    if status.success() {
        println!("Committed successfully");
        Ok(())
    } else {
        Err("Commit failed".into())
    }
}

fn ask_push() -> Result<bool, Box<dyn std::error::Error>> {
    print!("Do you want to push? (y/n): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() == "y")
}

fn push() -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("git")
        .arg("push")
        .status()?;
    if status.success() {
        println!("Pushed successfully");
        Ok(())
    } else {
        Err("Push failed".into())
    }
}
