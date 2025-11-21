use std::{
    collections::HashMap,
    sync::Mutex,
    time::Duration,
};

use once_cell::sync::Lazy;
use serenity::{prelude::Context, model::id::GuildId};
use tokio::{task::JoinHandle, time::sleep};
use tracing::warn;

use crate::helpers;

static IDLE_TASKS: Lazy<Mutex<HashMap<GuildId, JoinHandle<()>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct IdleManager;

impl IdleManager {
    pub fn arm(ctx: &Context, guild_id: GuildId, timeout: Duration) {
        let ctx = ctx.clone();

        Self::disarm(guild_id);

        let handle = tokio::spawn(async move {
            sleep(timeout).await;

            let snapshot = match helpers::get_queue::run(&ctx, guild_id).await {
                Ok(s) => s,
                Err(e) => {
                    warn!("IdleManager: get_queue failed for {guild_id}: {e:#}");
                    return;
                }
            };

            if snapshot.in_voice && snapshot.queue_len == 0 && !snapshot.is_playing {
                if let Some(manager) = songbird::get(&ctx).await {
                    let _ = manager.remove(guild_id).await;
                } else {
                    warn!("IdleManager: Songbird manager not found when trying to leave");
                }
            }

            let mut tasks = IDLE_TASKS
                .lock()
                .expect("IDLE_TASKS mutex poisoned");
            tasks.remove(&guild_id);
        });

        let mut tasks = IDLE_TASKS
            .lock()
            .expect("IDLE_TASKS mutex poisoned");
        tasks.insert(guild_id, handle);
    }

    pub fn disarm(guild_id: GuildId) {
        let mut tasks = IDLE_TASKS
            .lock()
            .expect("IDLE_TASKS mutex poisoned");
        if let Some(handle) = tasks.remove(&guild_id) {
            handle.abort();
        }
    }
}
