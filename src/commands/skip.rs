use anyhow::{anyhow, Result};
use serenity::{model::channel::Message, prelude::Context};

pub async fn execute(ctx: &Context, msg: &Message) -> Result<()> {
    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("DMs not supported"))?;

    let manager = songbird::get(ctx).await
        .ok_or_else(|| anyhow!("Songbird Voice client not initialized"))?
        .clone();

    let handler_lock = manager
        .get(guild_id)
        .ok_or_else(|| anyhow!("Not in a voice channel (use !join first)"))?;

    let handler = handler_lock.lock().await;
    let queue = handler.queue();

    if queue.is_empty() {
        msg.channel_id.say(&ctx.http, "⏭️ Nothing to skip — the queue is empty.").await?;
        return Ok(());
    }

    if queue.len() <= 1 {
        queue.stop();
        msg.channel_id.say(&ctx.http, "⏹️ No next track queued — stopped playback.").await?;
        return Ok(());
    }

    match queue.skip() {
        Ok(()) => {
            msg.channel_id.say(&ctx.http, "⏭️ Skipped to the next track.").await?;
        }
        Err(err) => {
            queue.stop();
            msg.channel_id.say(&ctx.http, format!("⏹️ Couldn't skip ({err}). Stopped playback.")).await?;
        }
    }

    Ok(())
}
