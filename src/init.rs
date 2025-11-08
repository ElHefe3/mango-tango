use anyhow::{anyhow, Context as _, Result};
use serenity::all::{ActivityData};
use serenity::prelude::Context;
use std::process::Command;
use tracing::{info};

use serenity::prelude::TypeMapKey;
pub struct YtDlpPath;
impl TypeMapKey for YtDlpPath {
    type Value = String;
}

pub fn app_startup() -> Result<()> {
    ensure_bin("yt-dlp")?;
    ensure_bin("ffmpeg")?;
    Ok(())
}

fn ensure_bin(name: &str) -> Result<()> {
    let version_flag = if name.ends_with("yt-dlp") { "--version" } else { "-version" };

    let out = Command::new(name)
        .arg(version_flag)
        .output()
        .with_context(|| format!("failed to spawn {name}"))?;

    if out.status.success() {
        info!("{name} OK");
        Ok(())
    } else {
        Err(anyhow!("{name} not usable (exit {status})", status = out.status))
    }
}


pub async fn after_ready(ctx: &Context) -> Result<()> {
    ctx.set_activity(Some(ActivityData::listening("!play")));

    {
        let mut data = ctx.data.write().await;
        data.insert::<YtDlpPath>("yt-dlp".to_string());
    }

    Ok(())
}
