use anyhow::{anyhow, Result};
use serenity::{
    model::{channel::Message},
    prelude::Context
};
use songbird::{input::Input};

pub async fn execute(ctx: &Context, msg: &Message, url: &str) -> Result<()> {
    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("DMs not supported"))?;

    let manager = songbird::get(ctx).await.unwrap().clone();
    let handler_lock = manager
        .get(guild_id)
        .ok_or_else(|| anyhow!("not in a voice channel (use !join first)"))?;

    let input: Input = crate::adapters::youtube::ytdlp_input(url);

    {
        let mut handler = handler_lock.lock().await;
        handler.enqueue_input(input).await;
    }

    msg.channel_id
        .say(&ctx.http, format!("▶️ queued: {url}"))
        .await?;

    Ok(())
}
