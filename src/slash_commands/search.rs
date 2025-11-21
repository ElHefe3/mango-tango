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

use crate::lifecycle::status::{StatusManager, Phase};
use crate::helpers;

pub fn register() -> CreateCommand {
    CreateCommand::new("search")
        .description("Use a keyword to find a song")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "query", "What to search for")
                .required(true),
        )
}

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let status = StatusManager::global();
    status.set_phase(Phase::Ready).await;

    let result = async {
        helpers::anxious_reply::run(ctx, cmd).await?;
        
        let gid = cmd.guild_id.ok_or_else(|| anyhow!("DMs not supported"))?;

        let query = cmd
            .data
            .options
            .iter()
            .find(|o| o.name == "query")
            .and_then(|o| o.value.as_str())
            .ok_or_else(|| anyhow!("missing 'query' string option"))?;

        let (voice_chan_id, guild_id_copy): (ChannelId, GuildId) = {
            let g = gid
                .to_guild_cached(&ctx.cache)
                .ok_or_else(|| anyhow!("guild not in cache"))?;
            let chan = g.voice_states.get(&cmd.user.id)
                .and_then(|vs| vs.channel_id)
                .ok_or_else(|| anyhow!("you must be in a voice channel"))?;
            (chan, gid)
        };

        let search = format!("ytsearch1:{query}");
        let input: Input = crate::adapters::youtube::ytdlp_input(&search);

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

        let content = format!(
            "🔎 ▶️ queued first result for “{query}” in {}",
            voice_chan_id.mention()
        );

        cmd.create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new().content(content),
        )
        .await?;

        Ok(())
    }
    .await;

    match &result {
        Ok(()) => {
            status.clear_error().await;
        }
        Err(e) => {
            status.set_phase(Phase::Errored).await;
            status
                .set_error(format!("slash /search failed: {e:#}"))
                .await;
        }
    }

    result
}
