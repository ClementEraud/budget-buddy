use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::shared::errors::{
    get_file_path_error::GetFilePathError, open_file_error::OpenFileError,
    read_file_error::ReadFileError,
};

pub trait FileSystemAdapter {
    fn filename(&self) -> String;

    fn get_file_path(&self) -> Result<PathBuf, GetFilePathError> {
        match ProjectDirs::from("com", "BudgetBuddy", "BudgetBuddy") {
            Some(proj_dirs) => Ok(proj_dirs.data_local_dir().to_path_buf()),
            None => Err(GetFilePathError {
                model: String::from("Budget"),
            }),
        }
    }

    fn open_file(&self) -> Result<File, OpenFileError> {
        let file_path = self.get_file_path()?;

        OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path.join(self.filename()))
            .map_err(OpenFileError::from)
    }

    fn read_file(&self) -> Result<String, ReadFileError> {
        let file_path = self.get_file_path()?;

        fs::read_to_string(file_path.join(self.filename())).map_err(ReadFileError::from)
    }
}
