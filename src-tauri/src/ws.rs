use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tauri::AppHandle;
use tauri_plugin_shell::{process::CommandEvent, ShellExt};
use tokio::{
    net::TcpListener,
    sync::{mpsc::unbounded_channel, Mutex},
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use uuid::Uuid;

pub struct WebSocketClient {
    writer: Arc<
        Mutex<
            futures_util::stream::SplitSink<
                tokio_tungstenite::WebSocketStream<
                    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
                >,
                Message,
            >,
        >,
    >,
}

impl WebSocketClient {
    // Create a new WebSocketClient by connecting to the specified URL
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Connect to the WebSocket server
        let (ws_stream, _) = connect_async(url).await?;
        println!("WebSocket handshake has been successfully completed");

        // Split the WebSocket stream
        let (write, _) = ws_stream.split();

        // Wrap the writer in Arc<Mutex<>> for thread-safe access
        let writer = Arc::new(Mutex::new(write));

        Ok(Self { writer })
    }

    // Send a binary message to the WebSocket server
    pub async fn send_message(&self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = self.writer.lock().await;
        writer.send(Message::Binary(data)).await?;
        Ok(())
    }

    // Send a text message to the WebSocket server
    pub async fn send_text(&self, text: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = self.writer.lock().await;
        writer.send(Message::Text(text)).await?;
        Ok(())
    }
    pub async fn send_json<T>(&self, data: &T) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Serialize,
    {
        let json_string = serde_json::to_string(data)?;
        self.send_text(json_string).await
    }
}

async fn find_free_port() -> Option<u16> {
    // Bind to port 0, which tells the OS to assign a free port
    let addr = SocketAddr::from(([127, 0, 0, 1], 0));

    match TcpListener::bind(addr).await {
        Ok(listener) => {
            // Get the assigned port
            if let Ok(addr) = listener.local_addr() {
                return Some(addr.port());
            }
            None
        }
        Err(_) => None,
    }
}

pub async fn spwan(app_handle: &AppHandle) -> (u16, String) {
    let token = Uuid::new_v4().to_string();
    let command = app_handle
        .shell()
        .sidecar("whip")
        .expect("whip sidecar does not exist");

    let valid_port = find_free_port().await.expect("port error");
    let (port_tx, mut port_rx) = unbounded_channel::<u16>();
    let (mut rx, _child) = command
        .args(["--port", &format!("{}", valid_port), "--token", &token])
        .spawn()
        .expect("spawn failed");

    tauri::async_runtime::spawn(async move {
        let mut port_parsed = false;

        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(bytes) => {
                    if let Ok(text) = String::from_utf8(bytes) {
                        dbg!("Send To", text);
                    }
                }
                CommandEvent::Stderr(bytes) => {
                    if !port_tx.is_closed() && !port_parsed {
                        if let Ok(text) = String::from_utf8(bytes) {
                            if text.contains("Uvicorn") {
                                dbg!("RUN: ", text);
                                port_tx.send(valid_port).expect("unable to send port");
                                port_parsed = true;
                            } else {
                                dbg!("Std Error:", text);
                            }
                            continue;
                        }
                    }
                }
                CommandEvent::Error(error) => {
                    dbg!("Error:", error);
                }

                _ => {
                    dbg!("Unkoown Error");
                }
            }
        }
    });

    let port = port_rx
        .recv()
        .await
        .expect("couldn't receive daemon port number");

    port_rx.close();

    (port, String::from(token))
}

pub async fn connect(port: u16, token: String) -> WebSocketClient {
    dbg!(port);
    let url = format!("ws://127.0.0.1:{}/ws/{}", port, token);

    let client = WebSocketClient::new(&url).await.expect("Failed to connect");

    client
}
