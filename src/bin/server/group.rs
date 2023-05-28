use crate::outbound::Outbound;
use async_std::task;
use simple_chat::Server;
use std::sync::Arc;
use text_colorizer::*;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Group {
        let (sender, _) = broadcast::channel(10000);
        Group { name, sender }
    }

    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe();

        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>) {
        let _ = self.sender.send(message);
    }

}
async fn handle_subscriber(
    group_name: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<Outbound>
) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => Server::Message {
                group_name: group_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => Server::Err(format!(
                "{}: Dropped {} messages from {}",
                "Warning".yellow().bold(),
                n,
                group_name.clone()
            )),
            Err(RecvError::Closed) => break,
        };

        if outbound.send(packet).await.is_err() {
            break;
        }
    }
}