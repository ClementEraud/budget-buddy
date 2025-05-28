use crate::{
    queries::{
        application::ports::budget_repository::BudgetRepositoryPort,
        domain::aggregates::budget::Budget,
        infrastructure::adapters::errors::get_budget_error::GetBudgetError,
    },
    shared::{
        adapters::file_system_adapter::FileSystemAdapter, value_objects::budget_date::BudgetDate,
    },
};

pub struct BudgetRepositoryFSAdapter {}

impl BudgetRepositoryFSAdapter {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystemAdapter for BudgetRepositoryFSAdapter {
    fn filename(&self) -> String {
        String::from("budget.json")
    }
}

impl BudgetRepositoryPort for BudgetRepositoryFSAdapter {
    fn get_budget_for_date(&self, date: &BudgetDate) -> Result<Option<Budget>, GetBudgetError> {
        let file_content = self.read_file()?;

        let budgets: Vec<Budget> = serde_json::from_str(&file_content)?;

        if budgets.is_empty() {
            return Ok(None);
        }

        Ok(budgets
            .into_iter()
            .find(|budget| budget.date().year == date.year && budget.date().month == date.month))
    }
}
