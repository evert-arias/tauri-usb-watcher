// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod usb_watcher;

use tauri::Emitter;
use usb_watcher::UsbWatcher;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let watcher = UsbWatcher::new();

            watcher.start(move |message| {
                let _ = app_handle.emit("usb-event", message);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
