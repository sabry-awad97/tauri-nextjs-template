// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
struct VisitorCounter {
    visitor: AtomicUsize,
}

impl VisitorCounter {
    fn increment(&self) {
        self.visitor.fetch_add(1, Ordering::Relaxed);
    }

    fn get(&self) -> usize {
        self.visitor.load(Ordering::Relaxed)
    }
}

#[tauri::command]
async fn greet(name: String, counter: tauri::State<'_, VisitorCounter>) -> Result<String, String> {
    counter.increment();
    println!("Visitor: {}", counter.get());
    Ok(format!("Hello {name}!"))
}

fn main() {
    tauri::Builder::default()
        .manage(VisitorCounter::default())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
