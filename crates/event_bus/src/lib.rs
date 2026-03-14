use tokio::{
    net::TcpListener,
    io::{AsyncWriteExt, AsyncBufReadExt, BufReader},
    sync::mpsc,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use common::events::{BotEvent, ControlCommand};
use tracing::{info, error};

pub struct EventBus {
    clients: Arc<Mutex<Vec<mpsc::UnboundedSender<BotEvent>>>>,
}

impl EventBus {
    pub async fn start(cmd_tx: mpsc::UnboundedSender<ControlCommand>) -> anyhow::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:7001").await?;
        info!("Event bus listening on 127.0.0.1:7001");
        
        let clients = Arc::new(Mutex::new(Vec::new()));
        let clients_clone = clients.clone();

        tokio::spawn(async move {
            while let Ok((stream, addr)) = listener.accept().await {
                info!("TUI connected from: {}", addr);
                let (reader, mut writer) = tokio::io::split(stream);
                let (client_tx, mut client_rx) = mpsc::unbounded_channel::<BotEvent>();
                
                clients_clone.lock().await.push(client_tx);

                // Read task (TUI -> Daemon)
                let cmd_tx_inner = cmd_tx.clone();
                tokio::spawn(async move {
                    let mut lines = BufReader::new(reader).lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        if let Ok(cmd) = serde_json::from_str::<ControlCommand>(&line) {
                            info!("Received command: {:?}", cmd);
                            let _ = cmd_tx_inner.send(cmd);
                        }
                    }
                });

                // Write task (Daemon -> TUI)
                tokio::spawn(async move {
                    while let Some(event) = client_rx.recv().await {
                        if let Ok(json) = serde_json::to_string(&event) {
                            if let Err(e) = writer.write_all((json + "\n").as_bytes()).await {
                                error!("Failed to write to client {}: {:?}", addr, e);
                                break;
                            }
                        }
                    }
                });
            }
        });

        Ok(Self { clients })
    }

    pub fn broadcast(&self, event: BotEvent) {
        let clients = self.clients.clone();
        tokio::spawn(async move {
            let mut clients_lock = clients.lock().await;
            clients_lock.retain(|tx| {
                tx.send(event.clone()).is_ok()
            });
        });
    }
}
