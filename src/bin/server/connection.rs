use crate::group_table::GroupTable;
use crate::outbound::Outbound;
use simple_chat::utils::{self, ChatResult};
use simple_chat::{Client, Server};
use async_std::io::BufReader;
use async_std::net::TcpStream;
use async_std::sync::Arc;
use async_std::prelude::*;

pub async fn server(socket: TcpStream, groups: Arc<GroupTable>) -> ChatResult<()> {
    let outbound = Arc::new(Outbound::new(socket.clone()));

    let buffered = BufReader::new(socket);
    let mut from_client = utils::receive_as_json(buffered);

    while let Some(request_result) = from_client.next().await {
        let request = request_result?;

        let result = match request {
            Client::Subscribe { group_name } => {
                let group = groups.get_or_create(group_name);
                group.join(outbound.clone());
                Ok(())
            }

            Client::Post {
                group_name,
                message,
            } => match groups.get(&group_name) {
                Some(group) => {
                    group.post(message);
                    Ok(())
                }
                None => Err(format!("Group '{}' does not exist", group_name)),
            },
        };

        if let Err(message) = result {
            let report = Server::Err(message);
            outbound.send(report).await?;
        }
    }
    Ok(())
}
