use async_std::io;
use async_std::net;
use async_std::prelude::*;
use async_std::task;
use simple_chat::utils::{self, ChatResult};
use simple_chat::{Client, Server};
use std::sync::Arc;
use text_colorizer::*;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect(&format!(
        "{}: {}{}",
        "Error".red().bold(),
        "Usage -> ".green().bold(),
        "client ADDRESS".purple().bold().italic()
    ));

    task::block_on(async {
        let socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        from_server.race(to_server).await?;

        Ok(())
    })
}

async fn send_commands(mut server: net::TcpStream) -> ChatResult<()> {
    println!(
        "|- {} : \n\
        |-----> {} GROUP \n\
        |-----> {} GROUP MESSAGE\n

        ",
        "Commands".blue().bold(),
        "Subscribe".cyan().bold(),
        "Post".cyan().bold()
    );

    let mut command_lines = io::BufReader::new(io::stdin()).lines();

    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;

        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };
        utils::send_as_json(&mut server, &request).await?;
        server.flush().await?;
    }

    Ok(())
}

async fn handle_replies(server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(server);
    let mut reply_stream = utils::receive_as_json(buffered);

    while let Some(reply) = reply_stream.next().await {
        match reply? {
            Server::Message {
                group_name,
                message,
            } => {
                println!(
                    "Message posted to {}: {}",
                    group_name.blue().bold(),
                    message.green()
                );
            }
            Server::Err(error) => {
                println!("{}: {}", "Server Error".red().bold(), error.red());
            }
        }
    }
    Ok(())
}

fn parse_command(line: &str) -> Option<Client> {
    let (command, rest) = get_next_token(line)?;
    if command.eq_ignore_ascii_case("post") {
        let (group, rest) = get_next_token(rest)?;
        let message = rest.trim_start().to_string();
        return Some(Client::Post {
            group_name: Arc::new(group.to_string()),
            message: Arc::new(message),
        });
    } else if command.eq_ignore_ascii_case("subscribe") {
        let (group, rest) = get_next_token(rest)?;
        if !rest.trim_start().is_empty() {
            eprintln!(
                "{} --> Group name can't contain spaces: {:?}",
                "Warning".yellow().bold(),
                line
            );
            return None;
        }
        return Some(Client::Subscribe {
            group_name: Arc::new(group.to_string()),
        });
    } else {
        eprintln!("{} --> Unrecognied command: {:?}", "Warning".yellow(), line);
        return None;
    }
}

fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }
    match input {
        _ => match input.find(char::is_whitespace) {
            Some(space) => Some((&input[0..space], &input[space..])),
            None => Some((input, "")),
        },
    }
}
