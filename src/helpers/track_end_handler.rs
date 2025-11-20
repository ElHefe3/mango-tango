use std::time::Duration;

use anyhow::Result;
use serenity::async_trait;
use serenity::{prelude::Context, model::id::GuildId};
use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler};

use crate::helpers;
use crate::lifecycle::idle::IdleManager;

pub struct TrackEndIdleHandler {
    pub ctx: Context,
    pub guild_id: GuildId,
}

#[async_trait]
impl VoiceEventHandler for TrackEndIdleHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(_tracks) = ctx {
            let result: Result<_> = helpers::get_queue::run(&self.ctx, self.guild_id).await;

            match result {
                Ok(snapshot) => {
                    if snapshot.in_voice && snapshot.queue_len == 0 && !snapshot.is_playing {
                        IdleManager::arm(&self.ctx, self.guild_id, Duration::from_secs(300));
                    } else {
                        IdleManager::disarm(self.guild_id);
                    }
                }
                Err(_e) => {
                    IdleManager::disarm(self.guild_id);
                }
            }
        }

        None
    }
}
