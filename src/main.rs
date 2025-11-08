use anyhow::Result;
use tracing::{info, warn};
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

mod adapters;
mod commands;
mod init;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry().with(filter).with(fmt::layer().compact()).init();

    let token = std::env::var("DISCORD_TOKEN")?;

    // Spawn the bot; keep a heartbeat so the process lives even if the client stops.
    let bot = tokio::spawn(async move {
        if let Err(e) = adapters::discord::run(token).await {
            tracing::error!("bot error: {e:#}");
        }
    });

    // Graceful shutdown
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            warn!("shutdown signal received");
        }
        _ = bot => { /* client ended */ }
    }

    info!("app exiting");
    Ok(())
}
