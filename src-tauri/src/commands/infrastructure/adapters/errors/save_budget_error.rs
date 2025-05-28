use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use super::open_file_error::OpenFileError;

pub struct SaveBudgetError {
    error: String,
}

impl From<OpenFileError> for SaveBudgetError {
    fn from(error: OpenFileError) -> Self {
        SaveBudgetError {
            error: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for SaveBudgetError {
    fn from(error: serde_json::Error) -> Self {
        SaveBudgetError {
            error: error.to_string(),
        }
    }
}

impl From<std::io::Error> for SaveBudgetError {
    fn from(error: std::io::Error) -> Self {
        SaveBudgetError {
            error: error.to_string(),
        }
    }
}

impl Display for SaveBudgetError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Failed to save budget !")
    }
}

impl Debug for SaveBudgetError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{{ file: {}, line: {}, error: {} }}",
            file!(),
            line!(),
            self.error
        )
    }
}
