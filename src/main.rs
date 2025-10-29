use std::process::Command;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check if in git repo
    if !is_git_repo() {
        eprintln!("Not in a git repository");
        return Ok(());
    }

    // Get commit history
    let history = get_commit_history()?;
    println!("Commit history:\n{}", history);

    // Get staged files
    let staged = get_staged_files()?;
    println!("Staged files:\n{}", staged);

    // Generate commit message using Gemini
    let message = generate_commit_message(&history, &staged).await?;
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
        "Based on the following Conventional Commits specification and the provided git commit history and staged changes, generate a single commit message that follows the specification exactly.\n\nConventional Commits Specification:\n{}\n\nCommit History:\n{}\n\nStaged Changes:\n{}\n\nGenerate only the commit message, nothing else.",
        include_str!("../conventional_commits.txt"), // Wait, need to create this file
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
