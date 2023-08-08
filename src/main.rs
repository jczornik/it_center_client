mod messages;

use messages::MessageDTO;
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

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::watch::channel(MessagesReceived::new());
    let handle = tokio::spawn(async move { download_messages_thread(tx).await });
    let handle2 = tokio::spawn(async move { notify_thread(rx).await });
    let _ = tokio::join!(handle);
    let _ = tokio::join!(handle2);
}
