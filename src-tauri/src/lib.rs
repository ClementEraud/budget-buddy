// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use operations::{Array, Expense, Income, Operations};
use tauri::command;

mod operations;

#[command]
fn get_balance() -> i32 {
    Operations::new().get_balance()
}

#[command]
fn get_incomes() -> Array<Income> {
    Operations::new().get_incomes().clone()
}

#[command]
fn get_planned_expenses() -> Array<Expense> {
    Operations::new().get_planned_expenses()
}

#[command]
fn get_actual_expenses() -> Array<Expense> {
    Operations::new().get_actual_expenses()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
