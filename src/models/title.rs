use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug, Clone, Hash, Eq, Serialize, Deserialize)]
pub struct Title {
    title: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TitleError {
    details: String,
}

impl TitleError {
    fn new(msg: &str) -> TitleError {
        TitleError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for TitleError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl Title {
    pub fn new(title: String) -> Result<Title, TitleError> {
        if title.is_empty() {
            Err(TitleError::new("Title Cannot be empty"))
        } else {
            Ok(Title { title })
        }
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.title)?;
        Ok(())
    }
}

#[cfg(test)]
mod title_tests {
    use crate::models::Title;

    #[test]
    fn creating_an_empty_title_should_fail() {
        let new_title = Title::new("".to_string());
        assert!(new_title.is_err())
    }
}
