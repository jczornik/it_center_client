mod messages;

use notify_rust::Notification;

static SERVER_ADDRESS: &str = "http://localhost:8080";

#[tokio::main]
async fn main() {
    let response = messages::download_all_new_messages().await;

    let response = response.unwrap();

    for message in response {
        let _ = Notification::new()
            .summary(&message.title)
            .body(&message.message)
            .timeout(0)
            .show();
    }
}
