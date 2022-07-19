use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug, Clone, Hash, Eq, Serialize, Deserialize)]
pub struct Comment {
    comment: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CommentError {
    details: String,
}

impl CommentError {
    fn new(msg: &str) -> CommentError {
        CommentError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for CommentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CommentError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl Comment {
    pub fn new(comment: String) -> Result<Comment, CommentError> {
        if comment.is_empty() {
            Err(CommentError::new("Comment cannot be empty"))
        } else {
            Ok(Comment { comment })
        }
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.comment)
    }
}

#[cfg(test)]
mod comment_tests {
    use crate::models::Comment;

    #[test]
    fn creating_empty_comment_should_fail() {
        let new_comment = Comment::new("".to_string());
        assert!(new_comment.is_err());
    }
}
