use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::shared::errors::read_file_error::ReadFileError;

pub struct GetBudgetError {
    error: String,
}

impl From<ReadFileError> for GetBudgetError {
    fn from(error: ReadFileError) -> Self {
        GetBudgetError {
            error: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for GetBudgetError {
    fn from(error: serde_json::Error) -> Self {
        GetBudgetError {
            error: error.to_string(),
        }
    }
}

impl From<std::io::Error> for GetBudgetError {
    fn from(error: std::io::Error) -> Self {
        GetBudgetError {
            error: error.to_string(),
        }
    }
}

impl Display for GetBudgetError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Failed to save budget !")
    }
}

impl Debug for GetBudgetError {
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
