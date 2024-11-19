use axum::{
    extract::{ws::WebSocket, ConnectInfo, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use clap::Parser;
use shared::{ClientCommand, ErrorCode, ServerCommand};
use std::net::SocketAddr;

#[derive(Parser, Debug)]
struct Args {
    /// The URL of the controller to connect to.
    #[arg(long, short, default_value = "0.0.0.0")]
    url: String,
    /// The port to connect to.
    #[arg(long, short, default_value = "3000")]
    port: String,
    /// The name of this machine.
    #[arg(long, short)]
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", args.url, args.port))
        .await
        .unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn root() -> Html<&'static str> {
    return Html(include_str!("../assets/index.html"));
}

async fn send_cmd(socket: &mut WebSocket, cmd: &ServerCommand) {
    let response = serde_json::to_string(cmd).unwrap();
    socket.send(response.into()).await.unwrap();
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Ok(inner) = serde_json::from_str::<ClientCommand>(msg.to_text().unwrap()) {
            match inner {
                ClientCommand::RegisterRequest { alias: name, jobs } => {
                    println!("Registering new worker \"{}\", taking {} jobs!", name, jobs);
                    // TODO: Check if the requested alias is already in use.
                    if false {
                        todo!();
                    } else {
                        send_cmd(
                            &mut socket,
                            &ServerCommand::ErrorResponse {
                                code: ErrorCode::AliasAlreadyInUse,
                            },
                        )
                        .await;
                        send_cmd(&mut socket, &ServerCommand::Drop).await;
                    }
                }
                ClientCommand::FinalizeRequest { package, data } => {
                    println!("{:?} finished package {}!", who, package);
                    // Save the built package data.
                    todo!();
                }
                ClientCommand::StatusResponse { package, progress } => {
                    // Update the saved status of the worker.
                    todo!();
                }
            }
        }
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    println!("{addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}
