use std::io::{Seek, SeekFrom, Write};

use crate::{
    commands::{
        application::ports::budget_repository::BudgetRepositoryPort,
        domain::aggregates::budget::Budget,
        infrastructure::adapters::errors::save_budget_error::SaveBudgetError,
    },
    shared::adapters::file_system_adapter::FileSystemAdapter,
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
    fn create_budget(&self, budget: Budget) -> Result<(), SaveBudgetError> {
        let mut file = self.open_file()?;
        let file_content = self.read_file()?;

        let mut existing_budgets: Vec<Budget> =
            serde_json::from_str(&file_content).unwrap_or_default();

        if existing_budgets.iter().any(|b| b.id() == budget.id()) {
            return Err(SaveBudgetError::new(String::from("Budget already exists")));
        }

        existing_budgets.push(budget);

        let serialized_budgets =
            serde_json::to_string(&existing_budgets).map_err(SaveBudgetError::from)?;

        file.seek(SeekFrom::Start(0))
            .map_err(SaveBudgetError::from)?;

        file.write_all(serialized_budgets.as_bytes())
            .map_err(SaveBudgetError::from)?;

        Ok(())
    }
}

// TO-DO: mock file system and add tests
// #[cfg(test)]
// mod tests {
//     use crate::shared::{
//         entities::operation::Operation,
//         types::array::Array,
//         value_objects::{
//             budget_date::BudgetDate, expense_type::ExpenseType, income_type::IncomeType,
//             operation_type::OperationType,
//         },
//     };

//     use super::*;

//     #[test]
//     fn should_create_budget() {
//         let repo = BudgetRepositoryFSAdapter::new();
//         let budget = Budget::new(
//             String::from("fasdf"),
//             BudgetDate {
//                 year: 2023,
//                 month: 1,
//             },
//             Array::from_vec(vec![
//                 Operation::new(
//                     String::from("1"),
//                     String::from("Salary"),
//                     1600.0,
//                     OperationType::Income(IncomeType::Salary),
//                 ),
//                 Operation::new(
//                     String::from("3"),
//                     String::from("Rent"),
//                     600.0,
//                     OperationType::Expense(ExpenseType::Housing),
//                 ),
//             ]),
//         );
//         repo.create_budget(budget).unwrap();
//     }
// }
