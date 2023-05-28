use group_table::GroupTable;
use simple_chat::utils::ChatResult;
use text_colorizer::*;
mod connection;
mod group;
mod group_table;
mod outbound;

use async_std::prelude::*;
use async_std::{net, task};
use connection::*;
use std::sync::Arc;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect(&format!(
        "{}: {}",
        "Error".red().bold(),
        "server ADDRESS".cyan()
    ));

    let chat_groups = Arc::new(GroupTable::new());

    task::block_on(async move {
        let listener = net::TcpListener::bind(address).await?;

        let mut incoming_connection = listener.incoming();

        while let Some(socket_result) = incoming_connection.next().await {
            let socket = socket_result?;
            let groups = chat_groups.clone();

            task::spawn(async {
                let result = server(socket, groups).await;

                if let Err(error) = result {
                    eprintln!("{}: {}", "Error".red().bold(), error);
                }
            });
        }
        Ok(())
    })
}
