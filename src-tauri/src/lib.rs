use std::fs::exists;

use commands::{
    application::use_cases::create_budget::CreateBudgetCommand,
    infrastructure::{
        adapters::file_system::budget_adapter::BudgetRepositoryFSAdapter as BudgetRepositoryCommandFSAdapter,
        controllers::create_budget::create_budget,
    },
};
use directories::ProjectDirs;
use queries::{
    application::use_cases::{
        get_account_summary_for_date::GetAccountSummaryForDate,
        get_budget_summary_for_date::GetBudgetSummaryForDate,
    },
    infrastructure::{
        adapters::{
            file_system::budget_adapter::BudgetRepositoryFSAdapter as BudgetRepositoryQueryFSAdapter,
            in_memory::account_adapter::AccountRepositoryInMemoryAdapter,
        },
        controllers::{
            get_account_summary_for_date::get_account_summary_for_date,
            get_budget_summary_for_date::get_budget_summary_for_date,
        },
    },
};
use tauri::Manager;

mod commands;
mod queries;
mod shared;

pub struct Queries {
    get_account_summary_for_date: GetAccountSummaryForDate<AccountRepositoryInMemoryAdapter>,
    get_budget_summary_for_date: GetBudgetSummaryForDate<BudgetRepositoryQueryFSAdapter>,
}

pub struct Commands {
    create_budget: CreateBudgetCommand<BudgetRepositoryCommandFSAdapter>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .setup(|app| {
            app.manage(Queries {
                get_account_summary_for_date: GetAccountSummaryForDate::new(
                    AccountRepositoryInMemoryAdapter::new(),
                ),
                get_budget_summary_for_date: GetBudgetSummaryForDate::new(
                    BudgetRepositoryQueryFSAdapter::new(),
                ),
            });

            app.manage(Commands {
                create_budget: CreateBudgetCommand::new(BudgetRepositoryCommandFSAdapter::new()),
            });

            let path;

            match ProjectDirs::from("com", "BudgetBuddy", "BudgetBuddy") {
                Some(proj_dirs) => path = proj_dirs.data_local_dir().to_path_buf(),
                None => panic!("Failed to get home directory"),
            };

            dbg!("Using directory to save data: {}", path.display());

            // Create directory if it doesn't exist
            match exists(&path) {
                Ok(true) => (),
                Ok(false) => std::fs::create_dir_all(&path).expect("Failed to create directory"),
                Err(e) => panic!("Error checking directory existence: {}", e),
            }

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_account_summary_for_date,
            create_budget,
            get_budget_summary_for_date
        ]);

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
