use std::{collections::HashMap, sync::{OnceLock, RwLock}};
use rand::random;
use serde_json::Value;

#[derive(Debug, Clone, Default)]
struct Storage {
    kv_pair_storage: HashMap<String, Value>
}

static STORAGE: RwLock<OnceLock<Storage>> = RwLock::new(OnceLock::new());

#[tauri::command]
pub async fn create_ticket(data: Value) -> String {
    let ticket = format!("{:x}", random::<u64>());

    let mut storage_guard = STORAGE.write().unwrap();

    if let Some(storage_access) = storage_guard.get_mut() {
        storage_access.kv_pair_storage.insert(ticket.clone(), data);
    } else {
        storage_guard.get_or_init(|| Storage::default());
        storage_guard.get_mut().unwrap().kv_pair_storage.insert(ticket.clone(), data);
    }

    ticket
}

#[tauri::command]
pub async fn redeem_ticket(ticket: String) -> Value {
    let mut storage_guard = STORAGE.write().unwrap();
    let guard = storage_guard.get_mut().unwrap();

    let data = guard.kv_pair_storage.get(&ticket).expect("Ticket doesn't exist?").clone();
    guard.kv_pair_storage.remove(&ticket);

    data
}