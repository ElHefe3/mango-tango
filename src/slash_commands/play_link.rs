use anyhow::{anyhow, Result};
use serenity::{
    all::{
        CommandInteraction, 
        CommandOptionType, 
        CreateCommand, 
        CreateCommandOption, 
        CreateInteractionResponseFollowup
    },
    model::id::{ChannelId, GuildId},
    prelude::*,
};
use serenity::model::mention::Mentionable;
use songbird::input::Input;

use crate::helpers;
use crate::adapters;

pub fn register() -> CreateCommand {
    CreateCommand::new("play_link")
        .description("Queue audio by URL")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "url", "Direct media/YouTube URL")
                .required(true),
        )
}

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    helpers::anxious_reply::run(ctx, cmd).await?;

    let gid = cmd.guild_id.ok_or_else(|| anyhow!("DMs not supported"))?;

    let (voice_chan_id, guild_id_copy): (ChannelId, GuildId) = {
        let g = gid
            .to_guild_cached(&ctx.cache)
            .ok_or_else(|| anyhow!("guild not in cache"))?;
        let chan = g
            .voice_states
            .get(&cmd.user.id)
            .and_then(|vs| vs.channel_id)
            .ok_or_else(|| anyhow!("you must be in a voice channel"))?;
        (chan, gid)
    };

    let url = cmd
        .data
        .options
        .iter()
        .find(|o| o.name == "url")
        .and_then(|o| o.value.as_str())
        .ok_or_else(|| anyhow!("missing 'url' string option"))?;

    let input: Input = adapters::youtube::ytdlp_input(url);
    let manager = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Songbird voice manager not found"))?
        .clone();

    let handler_lock = manager
        .get(guild_id_copy)
        .ok_or_else(|| anyhow!("not in a voice channel (use /join first)"))?;

    {
        let mut handler = handler_lock.lock().await;
        handler.enqueue_input(input).await;
    }

    let content = format!("▶️ queued: {url} in {}", voice_chan_id.mention());
    cmd.create_followup(
        &ctx.http,
        CreateInteractionResponseFollowup::new()
            .content(content),
    ).await?;

    Ok(())
}
