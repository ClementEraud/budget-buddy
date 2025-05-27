use std::fs::exists;

use application::use_cases::{
    commands::create_budget::CreateBudgetCommand,
    queries::get_account_summary::GetAccountSummaryQuery,
};
use directories::ProjectDirs;
use infrastructure::{
    adapters::{
        commands::budget::BudgetCommandRepositoryFSAdapter,
        queries::account::AccountQueryRepository,
    },
    controllers::{commands::create_budget, queries::get_account_summary},
};
use tauri::Manager;
use tauri_plugin_fs::FsExt;

mod application;
mod domain;
mod infrastructure;
mod shared;

pub struct Queries {
    get_account_summary_query: GetAccountSummaryQuery<AccountQueryRepository>,
}

pub struct Commands {
    create_budget_command: CreateBudgetCommand<BudgetCommandRepositoryFSAdapter>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)] // only enable instrumentation in development builds
    let devtools = tauri_plugin_devtools::init();

    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            app.manage(Queries {
                get_account_summary_query: GetAccountSummaryQuery::new(
                    AccountQueryRepository::new(),
                ),
            });

            app.manage(Commands {
                create_budget_command: CreateBudgetCommand::new(
                    BudgetCommandRepositoryFSAdapter::new(),
                ),
            });

            let scope = app.fs_scope();
            let path;

            match ProjectDirs::from("com", "BudgetBuddy", "BudgetBuddy") {
                Some(proj_dirs) => path = proj_dirs.data_local_dir().to_path_buf(),
                None => panic!("Failed to get home directory"),
            };

            match scope.allow_directory(path.as_os_str(), false) {
                Ok(_) => (),
                Err(e) => panic!("Error allowing directory: {}", e),
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
