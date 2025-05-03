use std::sync::Arc;

use tauri::AppHandle;
use tauri_plugin_shell::{process::CommandEvent, ShellExt};
use tokio::{io::{AsyncWriteExt, WriteHalf}, net::TcpStream, sync::{mpsc::unbounded_channel, Mutex}};


pub struct Client {
    writer: Arc<Mutex<WriteHalf<TcpStream>>>,
}

impl Client {
    pub async fn write(&self, s: String){
        self.writer
        .lock()
        .await
        .write_all((s + "\n").as_bytes())
        .await
        .expect("cannt write data to TCP");
    }
}

pub async fn spwan(app_handle: &AppHandle) -> u32 {
    let command = app_handle.shell().sidecar("whip").expect("no sidecar");

    let (port_tx, mut port_rx) = unbounded_channel::<u32>();
    let (mut rx, _child) = command.spawn().expect("spawn failed");

    tauri::async_runtime::spawn(async move{
        let mut port_parsed = false;

        while let Some(event) = rx.recv().await {
            match event {
               CommandEvent::Stdout(bytes) => {
                if !port_tx.is_closed() && !port_parsed {
                    if let Ok(port) = String::from_utf8_lossy(&bytes).trim().parse() {
                        port_tx.send(port).expect("unable to send port");
                        port_parsed = true;
                        continue;
                    }
                }
                dbg!("{}", String::from_utf8_lossy(&bytes).trim());
               }
               _ => {} 
            }
        }
    });

    let port = port_rx.recv().await.expect("couldn't receive daemon port number");
    port_rx.close();

    port
}

pub async fn connect(port:u32) -> Client {
    let addr = format!("127.0.0.1:{}", port);
    let stream = TcpStream::connect(addr).await.expect("connect failed");

    // NOTE:
    let (_read_stream, write_stream) = tokio::io::split(stream);

    Client {
        writer: Arc::new(Mutex::new(write_stream)),
    }
}