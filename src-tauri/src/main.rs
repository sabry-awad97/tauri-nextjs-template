// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicUsize, Ordering};

use specta::collect_types;
use tauri_specta::ts;

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
#[specta::specta]
async fn greet(name: String, counter: tauri::State<'_, VisitorCounter>) -> Result<String, String> {
    counter.increment();
    println!("Visitor: {}", counter.get());
    Ok(format!("Hello {name}!"))
}

fn main() {
    #[cfg(debug_assertions)] // only include this code on debug builds
    ts::export(collect_types![greet], "../app/types/bindings.ts").unwrap();

    tauri::Builder::default()
        .manage(VisitorCounter::default())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
