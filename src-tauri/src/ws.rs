use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{future::Future, net::SocketAddr, sync::Arc};
use tauri::AppHandle;
use tauri_plugin_shell::{process::CommandEvent, ShellExt};
use tokio::{
    net::TcpListener,
    sync::{mpsc::unbounded_channel, Mutex},
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use uuid::Uuid;

type CustomResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

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
    // Add a reader field to store the receiving half of the split WebSocket
    reader: Arc<
        Mutex<
            futures_util::stream::SplitStream<
                tokio_tungstenite::WebSocketStream<
                    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
                >,
            >,
        >,
    >,
}

impl Clone for WebSocketClient {
    fn clone(&self) -> Self {
        Self {
            writer: Arc::clone(&self.writer),
            reader: Arc::clone(&self.reader),
        }
    }
}

impl WebSocketClient {
    // Create a new WebSocketClient by connecting to the specified URL
    pub async fn new(url: &str) -> CustomResult<Self> {
        // Connect to the WebSocket server
        let (ws_stream, _) = connect_async(url).await?;
        println!("WebSocket handshake has been successfully completed");

        // Split the WebSocket stream
        let (write, read) = ws_stream.split();

        // Wrap both halves in Arc<Mutex<>> for thread-safe access
        let writer = Arc::new(Mutex::new(write));
        let reader = Arc::new(Mutex::new(read));

        Ok(Self { writer, reader })
    }

    // Send a binary message to the WebSocket server
    pub async fn send_message(&self, data: Vec<u8>) -> CustomResult<()> {
        let mut writer = self.writer.lock().await;
        writer.send(Message::Binary(data)).await?;
        Ok(())
    }

    // Send a text message to the WebSocket server
    pub async fn send_text(&self, text: String) -> CustomResult<()> {
        let mut writer = self.writer.lock().await;
        writer.send(Message::Text(text)).await?;
        Ok(())
    }

    pub async fn send_json<T>(&self, data: &T) -> CustomResult<()>
    where
        T: Serialize,
    {
        let json_string = serde_json::to_string(data)?;
        self.send_text(json_string).await
    }

    // Receive a message from the WebSocket server
    pub async fn receive_message(&self) -> CustomResult<Option<Message>> {
        let mut reader = self.reader.lock().await;
        match reader.next().await {
            Some(Ok(msg)) => Ok(Some(msg)),
            Some(Err(e)) => Err(Box::new(e)),
            None => Ok(None), // Connection closed
        }
    }

    // Helper method to receive and parse JSON messages
    pub async fn receive_json<T>(&self) -> CustomResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(msg) = self.receive_message().await? {
            match msg {
                Message::Text(text) => {
                    let parsed = serde_json::from_str(&text)?;
                    Ok(Some(parsed))
                }
                Message::Binary(bin) => {
                    let parsed = serde_json::from_slice(&bin)?;
                    Ok(Some(parsed))
                }
                _ => Ok(None), // Ignore other message types
            }
        } else {
            Ok(None) // Connection closed
        }
    }

    pub async fn listen<F, Fut>(&self, mut callback: F) -> CustomResult<()>
    where
        F: FnMut(Message) -> Fut,
        Fut: Future<Output = CustomResult<()>>,
    {
        loop {
            match self.receive_message().await? {
                Some(msg) => {
                    callback(msg).await?;
                }
                None => {
                    println!("WebSocket connection closed");
                    break;
                }
            }
        }
        Ok(())
    }

    pub async fn start_listening(&self) -> CustomResult<()> {
        println!("Started listening for WebSocket messages");

        self.listen(|msg| async move {
            match msg {
                Message::Text(text) => {
                    println!("Received text message: {}", text);
                    match serde_json::from_str::<serde_json::Value>(&text) {
                        Ok(json) => {
                            println!("Parsed JSON: {:?}", json);
                        }
                        Err(e) => {
                            println!("Not a valid JSON: {}", e);
                        }
                    }
                }
                Message::Binary(data) => {
                    println!("Received binary message, {} bytes", data.len());
                }
                Message::Ping(_) => {
                    println!("Received ping");
                }
                Message::Pong(_) => {
                    println!("Received pong");
                }
                Message::Close(frame) => {
                    println!("Received close frame: {:?}", frame);
                }
                _ => {
                    println!("Received other message type");
                }
            }

            // Return with the correct error type
            Ok(())
        })
        .await
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

    let listener_client = client.clone();

    tokio::spawn(async move {
        listener_client
            .start_listening()
            .await
            .expect("Listening task failed");
    });
    client
}
