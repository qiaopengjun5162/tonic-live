use anyhow::Result;
use std::env;
use tokio::io::{self, AsyncBufReadExt};
use tonic_live::client::Client;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let username = env::var("TONIC_LIVE_USERNAME").unwrap_or("alice".to_string());
    info!("username: {}", username);
    let mut client = Client::new(username).await;
    client.login().await?;
    client.get_messages().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    while let Ok(Some(line)) = stdin.next_line().await {
        client.send_message("lobby", line).await?;
    }
    Ok(())
}
