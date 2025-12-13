mod message_box;

pub use message_box::{MessageBox, display_commit_message};

/// User action choices
#[derive(Debug, Clone, PartialEq)]
pub enum UserAction {
    Accept,
    Edit,
    Regenerate,
    Quit,
}
