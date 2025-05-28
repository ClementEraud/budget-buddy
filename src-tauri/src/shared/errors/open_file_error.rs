use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io;

use super::get_file_path_error::GetFilePathError;

pub struct OpenFileError {
    pub error: String,
}

impl From<io::Error> for OpenFileError {
    fn from(error: io::Error) -> Self {
        OpenFileError {
            error: error.to_string(),
        }
    }
}

impl From<GetFilePathError> for OpenFileError {
    fn from(error: GetFilePathError) -> Self {
        OpenFileError {
            error: error.to_string(),
        }
    }
}

impl Display for OpenFileError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Failed to open save file: {}", self.error)
    }
}

impl Debug for OpenFileError {
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
