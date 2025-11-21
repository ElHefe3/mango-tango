pub mod ping;
pub mod skip;
pub mod search;
pub mod status;
pub mod play_link;

use anyhow::Result;
use serenity::{all::Command, http::Http};
use tracing::{debug, error, info};

pub async fn register_all(http: &Http) -> Result<()> {
    info!("register_all: start");

    debug!("Upserting /ping…");
    match Command::create_global_command(http, ping::register()).await {
        Ok(c) => info!("registered /{} (id={})", c.name, c.id),
        Err(e) => {
            error!("failed to upsert /ping: {e:#}");
            return Err(e.into());
        }
    }

    debug!("Upserting /search…");
    match Command::create_global_command(http, search::register()).await {
        Ok(c) => info!("registered /{} (id={})", c.name, c.id),
        Err(e) => {
            error!("failed to upsert /search: {e:#}");
            return Err(e.into());
        }
    }

    debug!("Upserting /play_link…");
    match Command::create_global_command(http, play_link::register()).await {
        Ok(c) => info!("registered /{} (id={})", c.name, c.id),
        Err(e) => {
            error!("failed to upsert /play_link: {e:#}");
            return Err(e.into());
        }
    }

    debug!("Upserting /status…");
    match Command::create_global_command(http, status::register()).await {
        Ok(c) => info!("registered /{} (id={})", c.name, c.id),
        Err(e) => {
            error!("failed to upsert /status: {e:#}");
            return Err(e.into());
        }
    }

    debug!("Upserting /skip…");
    match Command::create_global_command(http, skip::register()).await {
        Ok(c) => info!("registered /{} (id={})", c.name, c.id),
        Err(e) => {
            error!("failed to upsert /skip: {e:#}");
            return Err(e.into());
        }
    }

    info!("register_all: done");
    Ok(())
}
