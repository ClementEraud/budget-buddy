use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io;

use super::get_file_path_error::GetFilePathError;

pub struct ReadFileError {
    pub error: String,
}

impl From<io::Error> for ReadFileError {
    fn from(error: io::Error) -> Self {
        ReadFileError {
            error: error.to_string(),
        }
    }
}

impl From<GetFilePathError> for ReadFileError {
    fn from(error: GetFilePathError) -> Self {
        ReadFileError {
            error: error.to_string(),
        }
    }
}

impl Display for ReadFileError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Failed to read save file: {}", self.error)
    }
}

impl Debug for ReadFileError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{{ file: {}, line: {}, file_path: {} }}",
            file!(),
            line!(),
            self.error
        )
    }
}
