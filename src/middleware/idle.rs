use std::{future::Future, time::Duration};

use anyhow::Result;
use serenity::{all::CommandInteraction, prelude::Context};

use crate::helpers;
use crate::lifecycle::idle::IdleManager;

pub async fn run_music_with_idle<Fut>(
    ctx: &Context,
    cmd: &CommandInteraction,
    fut: Fut,
) where
    Fut: Future<Output = Result<()>>,
{
    let guild_id = cmd.guild_id;

    let _ = fut.await;

    let Some(guild_id) = guild_id else {
        return;
    };

    if let Ok(snapshot) = helpers::get_queue::run(ctx, guild_id).await {
        if snapshot.in_voice && snapshot.queue_len == 0 && !snapshot.is_playing {
            IdleManager::arm(ctx, guild_id, Duration::from_secs(5));
        } else {
            IdleManager::disarm(guild_id);
        }
    } else {
        IdleManager::disarm(guild_id);
    }
}
