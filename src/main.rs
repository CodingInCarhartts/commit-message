mod config;
mod provider;
mod git;
mod emoji;
mod message;
mod prompt;
mod ui;

use config::Config;
use provider::create_provider;
use git::{is_git_repo, get_commit_history, get_staged_diff, get_diff_stat, count_staged_files, commit, push, GitError};
use emoji::{add_emoji_prefix, remove_emoji_prefix};
use message::CommitMessage;
use prompt::build_commit_prompt;
use ui::{display_commit_message, UserAction};
use std::io::{self, Write};
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Verify we're in a git repository
    if !is_git_repo() {
        return Err("Not in a git repository. Please run this command from within a git repository.".into());
    }

    // Load configuration
    let config = Config::from_env()?;

    // Get git context
    let commit_history = get_commit_history(10).unwrap_or_default();

    let staged_diff = match get_staged_diff(config.max_diff_lines) {
        Ok(diff) => diff,
        Err(GitError::NoStagedChanges) => {
            return Err("No staged changes. Use 'git add <files>' to stage changes first.".into());
        }
        Err(e) => return Err(e.into()),
    };

    let diff_stat = get_diff_stat();
    let file_count = count_staged_files();

    // Create AI provider
    let provider = create_provider(&config);
    println!("🚀 Using {} ({})", provider.name(), provider.model());
    println!("📁 {} file(s) changed", file_count);

    // Build prompt
    let prompt_text = build_commit_prompt(&staged_diff, &commit_history, &diff_stat);

    // Main interaction loop
    let mut attempts = 0u32;

    loop {
        attempts += 1;

        if attempts > config.max_retries {
            return Err(format!(
                "Failed to generate a valid commit message after {} attempts",
                config.max_retries
            ).into());
        }

        // Generate message
        println!("\n⏳ Generating commit message (attempt {})...", attempts);

        let response = match provider.generate(&prompt_text).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("⚠️  API error: {}. Retrying...", e);
                continue;
            }
        };

        // Parse response
        let mut commit_msg = CommitMessage::parse_from_ai_response(&response);

        // Apply emoji prefix if enabled
        if config.emoji_enabled {
            commit_msg.subject = add_emoji_prefix(&commit_msg.subject);
        }

        // Display the message with iocraft
        display_commit_message(
            &commit_msg.subject,
            commit_msg.body.as_deref(),
            provider.name(),
            provider.model(),
        );

        // Get user choice
        let action = prompt_action()?;

        match action {
            UserAction::Accept => {
                let git_message = commit_msg.to_git_message();
                println!("\n⏳ Committing...");
                commit(&git_message)?;
                println!("✓ Committed successfully!");

                // Ask about push
                if ask_push()? {
                    println!("⏳ Pushing...");
                    push()?;
                    println!("✓ Pushed successfully!");
                }

                break;
            }
            UserAction::Edit => {
                let display = commit_msg.to_git_message();
                // Remove emoji for editing (will be re-added after)
                let for_edit = if config.emoji_enabled {
                    let mut lines: Vec<&str> = display.lines().collect();
                    if let Some(first) = lines.first_mut() {
                        let stripped = remove_emoji_prefix(first);
                        let mut result = vec![stripped];
                        result.extend(lines.into_iter().skip(1).map(String::from));
                        result.join("\n")
                    } else {
                        display
                    }
                } else {
                    display
                };

                let edited = edit_message(&for_edit)?;

                if edited.trim().is_empty() {
                    println!("⚠️  Empty message, aborting commit");
                    return Ok(());
                }

                // Re-add emoji if enabled
                let final_message = if config.emoji_enabled {
                    let mut lines: Vec<String> = edited.lines().map(String::from).collect();
                    if let Some(first) = lines.first_mut() {
                        *first = add_emoji_prefix(first);
                    }
                    lines.join("\n")
                } else {
                    edited
                };

                println!("\n⏳ Committing...");
                commit(&final_message)?;
                println!("✓ Committed successfully!");

                if ask_push()? {
                    println!("⏳ Pushing...");
                    push()?;
                    println!("✓ Pushed successfully!");
                }

                break;
            }
            UserAction::Regenerate => {
                println!("🔄 Regenerating...");
                attempts = 0; // Reset attempts for regeneration
                continue;
            }
            UserAction::Quit => {
                println!("👋 Aborted");
                return Ok(());
            }
        }
    }

    Ok(())
}

fn prompt_action() -> io::Result<UserAction> {
    println!();
    print!(
        "  {}  {}  {}  {} : ",
        "\x1b[32m[A]ccept\x1b[0m",
        "\x1b[33m[E]dit\x1b[0m",
        "\x1b[36m[R]egenerate\x1b[0m",
        "\x1b[31m[Q]uit\x1b[0m"
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().chars().next() {
        Some('a') | Some('y') => Ok(UserAction::Accept),
        Some('e') => Ok(UserAction::Edit),
        Some('r') => Ok(UserAction::Regenerate),
        Some('q') | Some('n') => Ok(UserAction::Quit),
        _ => {
            println!("Invalid choice. Please enter A, E, R, or Q.");
            prompt_action()
        }
    }
}

fn ask_push() -> io::Result<bool> {
    print!("\n🔼 Push to remote? [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase() == "y")
}

fn edit_message(message: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::fs;
    use std::env;
    use std::process::Command;

    let path = env::temp_dir().join(".cm_commit_msg_edit");
    fs::write(&path, message)?;

    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "nano".to_string());

    println!("📝 Opening {}...", editor);

    let status = Command::new(&editor).arg(&path).status()?;

    if !status.success() {
        return Err(format!("Editor '{}' exited with error", editor).into());
    }

    let edited = fs::read_to_string(&path)?;
    let _ = fs::remove_file(&path);

    Ok(edited.trim().to_string())
}
