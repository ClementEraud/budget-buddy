use std::fs::exists;

use commands::{
    application::use_cases::create_budget::CreateBudgetCommand,
    infrastructure::{
        adapters::file_system::budget_adapter::BudgetRepositoryFSAdapter,
        controllers::create_budget::create_budget,
    },
};
use directories::ProjectDirs;
use queries::{
    application::use_cases::get_account_summary::GetAccountSummaryQuery,
    infrastructure::{
        adapters::in_memory::account_adapter::AccountRepositoryInMemoryAdapter,
        controllers::get_account_summary::get_account_summary,
    },
};
use tauri::Manager;

mod commands;
mod queries;
mod shared;

pub struct Queries {
    get_account_summary: GetAccountSummaryQuery<AccountRepositoryInMemoryAdapter>,
}

pub struct Commands {
    create_budget: CreateBudgetCommand<BudgetRepositoryFSAdapter>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .setup(|app| {
            app.manage(Queries {
                get_account_summary: GetAccountSummaryQuery::new(
                    AccountRepositoryInMemoryAdapter::new(),
                ),
            });

            app.manage(Commands {
                create_budget: CreateBudgetCommand::new(BudgetRepositoryFSAdapter::new()),
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
        .invoke_handler(tauri::generate_handler![get_account_summary, create_budget]);

    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtools);
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
