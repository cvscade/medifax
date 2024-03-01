#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modals;
mod utils;

use utils::generics::generic_details;
use utils::interop::{create_ticket, redeem_ticket};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generic_details, create_ticket, redeem_ticket])
        .run(tauri::generate_context!())
        .expect("Tauri crashed! Fatal error.");
}
