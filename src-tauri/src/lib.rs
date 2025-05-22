use application::use_cases::queries::{
    get_account_balance::GetAccountBalanceQuery, get_account_expenses::GetAccountExpensesQuery,
    get_account_incomes::GetAccountIncomesQuery,
};
use infrastructure::{
    adapters::queries::account::AccountQueryRepository,
    controllers::queries::{get_account_balance, get_account_expenses, get_account_incomes},
};
use tauri::Manager;

mod application;
mod domain;
mod infrastructure;
mod shared;

pub struct Queries {
    get_account_balance_query: GetAccountBalanceQuery<AccountQueryRepository>,
    get_account_incomes_query: GetAccountIncomesQuery<AccountQueryRepository>,
    get_account_expenses_query: GetAccountExpensesQuery<AccountQueryRepository>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .setup(|app| {
            app.manage(Queries {
                get_account_balance_query: GetAccountBalanceQuery::new(
                    AccountQueryRepository::new(),
                ),
                get_account_incomes_query: GetAccountIncomesQuery::new(
                    AccountQueryRepository::new(),
                ),
                get_account_expenses_query: GetAccountExpensesQuery::new(
                    AccountQueryRepository::new(),
                ),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_account_balance,
            get_account_expenses,
            get_account_incomes,
        ]);

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
