use anyhow::Result;
use tonic_live::server;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init(); // Initialize the logger

    // Start the server
    server::start().await;
    Ok(())
}
