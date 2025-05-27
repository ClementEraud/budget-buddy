use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone)]
pub struct GetFilePathError {
    pub model: String,
}

impl Display for GetFilePathError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Failed to get file path for model: {}", self.model)
    }
}

impl Debug for GetFilePathError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{{ file: {}, line: {}, model: {} }}",
            file!(),
            line!(),
            self.model
        )
    }
}
