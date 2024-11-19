#![allow(unused)]
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use shared::{ClientCommand, ServerCommand};
use tokio_tungstenite::connect_async;

#[derive(Parser, Debug)]
struct Args {
    /// The URL of the controller to connect to.
    #[arg(long, short, default_value = "localhost")]
    url: String,
    /// The port to connect to.
    #[arg(long, short, default_value = "3000")]
    port: String,
    /// The name of this machine.
    #[arg(long, short)]
    alias: String,
    /// The amount of parallel build jobs this machine can take.
    #[arg(long, short)]
    jobs: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Establish a connection to the server.
    let (mut socket, _) = connect_async(format!("ws://{}:{}/ws", args.url, args.port))
        .await
        .unwrap();

    // Tell the controller we're ready to take jobs.
    {
        let register = ClientCommand::RegisterRequest {
            alias: args.alias,
            jobs: args.jobs,
        };
        let data = serde_json::to_string(&register).unwrap();
        socket.send(data.into()).await.unwrap();
        println!("Connection to controller established.");
    }

    // Wait for the server to give us commands.
    loop {
        let msg = socket.next().await.unwrap().unwrap();
        if let Ok(inner) = serde_json::from_str::<ServerCommand>(msg.to_text().unwrap()) {
            match inner {
                ServerCommand::RunRequest {
                    package,
                    config,
                    data,
                } => {
                    todo!()
                }
                ServerCommand::Drop => {
                    panic!("Controller told us to disconnect ourselves!");
                }
                ServerCommand::StatusRequest {} => todo!(),
                ServerCommand::ErrorResponse { code } => {
                    println!("[Error] {:?}", code);
                }
            }
        }
    }
}
