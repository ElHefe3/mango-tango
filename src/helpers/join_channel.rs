use anyhow::{anyhow, Result};
use serenity::{
    all::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage},
    model::id::{ChannelId, GuildId},
    prelude::*,
};
use songbird::events::{Event, TrackEvent};

use crate::helpers::track_end_handler::TrackEndIdleHandler;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let gid: GuildId = cmd.guild_id.ok_or_else(|| anyhow!("DMs not supported"))?;

    let manager = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Songbird voice manager not found"))?
        .clone();

    if manager.get(gid).is_some() {
        return Ok(());
    }

    let (voice_chan_id, guild_id_copy): (ChannelId, GuildId) = {
        let g = gid
            .to_guild_cached(&ctx.cache)
            .ok_or_else(|| anyhow!("guild not in cache"))?;
        let chan = g
            .voice_states
            .get(&cmd.user.id)
            .and_then(|vs: &serenity::all::VoiceState| vs.channel_id)
            .ok_or_else(|| anyhow!("You must be in a voice channel"))?;
        (chan, gid)
    };

    let handler_lock = match manager.join(guild_id_copy, voice_chan_id).await {
        Ok(lock) => lock,
        Err(e) => {
            let _ = cmd
                .create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content(format!("Couldn't join voice: {e:#}"))
                            .ephemeral(true),
                    ),
                )
                .await;

            return Err(anyhow!("join failed: {e:#}"));
        }
    };

    {
        let mut call = handler_lock.lock().await;

        call.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndIdleHandler {
                ctx: ctx.clone(),
                guild_id: guild_id_copy,
            },
        );
    }

    Ok(())
}
