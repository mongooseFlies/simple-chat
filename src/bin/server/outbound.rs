use async_std::sync::Mutex;
use async_std::net::TcpStream;
use simple_chat::Server;
use simple_chat::utils::{self, ChatResult};

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(client: TcpStream) -> Self {
        Outbound(Mutex::new(client))
    }

    pub async fn send(&self, packet: Server) -> ChatResult<()> {
        let mut guard = self.0.lock().await;
        utils::send_as_json(&mut *guard, &packet).await?;
        Ok(())
    }
}
