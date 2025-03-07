use application::use_cases::queries::{
    get_actual_expenses::GetActualExpensesQuery, get_balance::GetBalanceQuery,
    get_incomes::GetIncomesQuery, get_planned_expenses::GetPlannedExpensesQuery,
};
use domain::{array::Array, expense::Expense, income::Income};
use infrastructure::{
    adapters::queries::operations::OperationsQueryRepository,
    controllers::queries::{get_actual_expenses, get_balance, get_incomes, get_planned_expenses},
};
use tauri::Manager;

mod application;
mod domain;
mod infrastructure;

pub struct Queries {
    get_balance_query: GetBalanceQuery<OperationsQueryRepository>,
    get_incomes_query: GetIncomesQuery<OperationsQueryRepository>,
    get_actual_expenses_query: GetActualExpensesQuery<OperationsQueryRepository>,
    get_planned_expenses_query: GetPlannedExpensesQuery<OperationsQueryRepository>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Queries {
                get_balance_query: GetBalanceQuery::new(OperationsQueryRepository::new()),
                get_incomes_query: GetIncomesQuery::new(OperationsQueryRepository::new()),
                get_actual_expenses_query: GetActualExpensesQuery::new(
                    OperationsQueryRepository::new(),
                ),
                get_planned_expenses_query: GetPlannedExpensesQuery::new(
                    OperationsQueryRepository::new(),
                ),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_balance,
            get_incomes,
            get_planned_expenses,
            get_actual_expenses
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
