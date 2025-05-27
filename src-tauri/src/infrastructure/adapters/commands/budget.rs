use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::{
    application::ports::commands::budget::BudgetCommandRepositoryPort,
    domain::aggregates::commands::budget::BudgetCommand,
    infrastructure::adapters::errors::{
        get_file_path_error::GetFilePathError, open_file_error::OpenFileError,
        save_budget_error::SaveBudgetError,
    },
};

#[derive(Clone)]
pub struct BudgetCommandRepositoryFSAdapter {}

impl BudgetCommandRepositoryFSAdapter {
    pub fn new() -> Self {
        Self {}
    }

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
            .append(true)
            .open(file_path.join("budget.json"))
            .map_err(OpenFileError::from)
    }
}

impl BudgetCommandRepositoryPort for BudgetCommandRepositoryFSAdapter {
    fn create_budget(&self, budget: BudgetCommand) -> Result<(), SaveBudgetError> {
        let mut file = self.open_file()?;
        let serialized_budget = serde_json::to_string(&budget).map_err(SaveBudgetError::from)?;

        file.write_all(serialized_budget.as_bytes())
            .map_err(SaveBudgetError::from)?;

        Ok(())
    }
}
