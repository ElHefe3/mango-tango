use anyhow::{anyhow, Result};
use serenity::{prelude::Context, model::id::GuildId};

pub struct QueueSnapshot {
    pub in_voice: bool,
    pub queue_len: usize,
    pub is_playing: bool,
}

pub async fn run(ctx: &Context, guild_id: GuildId) -> Result<QueueSnapshot> {
    let manager = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Songbird voice manager not found"))?;

    let Some(call_lock) = manager.get(guild_id) else {
        return Ok(QueueSnapshot {
            in_voice: false,
            queue_len: 0,
            is_playing: false,
        });
    };

    let call = call_lock.lock().await;
    let queue = call.queue();

    let queue_len = queue.len();
    let is_playing = queue.current().is_some();

    Ok(QueueSnapshot {
        in_voice: true,
        queue_len,
        is_playing,
    })
}
