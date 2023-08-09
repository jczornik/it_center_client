// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod messages;

use messages::{MessageDTO, ProcessingMessageError};
use notify_rust::Notification;
use tokio::sync::watch::{Receiver, Sender};

static SERVER_ADDRESS: &str = "http://localhost:8080";
static CHECK_MESSAGE_INTERVAL: u64 = 10;

type MessagesReceived = Vec<MessageDTO>;

async fn download_messages_thread(tx: Sender<MessagesReceived>) {
    loop {
        let response = messages::download_all_new_messages().await.unwrap();
        tx.send(response).unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(CHECK_MESSAGE_INTERVAL)).await;
    }
}

async fn notify_thread(mut rx: Receiver<MessagesReceived>) {
    while rx.changed().await.is_ok() {
        let messages = rx.borrow_and_update();
        if !messages.is_empty() {
            for message in messages.iter() {
                let _ = Notification::new()
                    .summary(&message.title)
                    .body(&message.message)
                    .timeout(0)
                    .show();
            }
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|_| {
            let (tx, rx) = tokio::sync::watch::channel(MessagesReceived::new());
            // tauri::async_runtime::spawn(async move { notify_thread(rx).await });
            // tauri::async_runtime::spawn(async move { download_messages_thread(tx).await });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            messages::download_all_messages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
