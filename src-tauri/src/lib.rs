use application::use_cases::queries::{
    get_balance::GetBalanceQuery, get_expenses::GetExpensesQuery, get_incomes::GetIncomesQuery,
};
use infrastructure::{
    adapters::queries::account::AccountQueryRepository,
    controllers::queries::{get_balance, get_expenses, get_incomes},
};
use tauri::Manager;

mod application;
mod domain;
mod infrastructure;
mod shared;

pub struct Queries {
    get_balance_query: GetBalanceQuery<AccountQueryRepository>,
    get_incomes_query: GetIncomesQuery<AccountQueryRepository>,
    get_expenses_query: GetExpensesQuery<AccountQueryRepository>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .setup(|app| {
            app.manage(Queries {
                get_balance_query: GetBalanceQuery::new(AccountQueryRepository::new()),
                get_incomes_query: GetIncomesQuery::new(AccountQueryRepository::new()),
                get_expenses_query: GetExpensesQuery::new(AccountQueryRepository::new()),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_balance,
            get_incomes,
            get_expenses,
        ]);

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
