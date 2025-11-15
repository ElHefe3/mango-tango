use anyhow::{anyhow, Result};
use serenity::{
    all::{
        CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage
    },
    builder::CreateCommand,
    model::id::{ChannelId, GuildId},
    prelude::*,
};
use serenity::model::mention::Mentionable;

pub fn register() -> CreateCommand {
    CreateCommand::new("join").description("Join current voice channel")
}

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let gid = cmd.guild_id.ok_or_else(|| anyhow!("DMs not supported"))?;

    let (voice_chan_id, guild_id_copy): (ChannelId, GuildId) = {
        let g = gid.to_guild_cached(&ctx.cache).ok_or_else(|| anyhow!("guild not in cache"))?;
        let chan = g.voice_states.get(&cmd.user.id)
            .and_then(|vs| vs.channel_id)
            .ok_or_else(|| anyhow!("you must be in a voice channel"))?;
        (chan, gid)
    };

    let manager = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Songbird voice manager not found"))?
        .clone();

    let content = match manager.join(guild_id_copy, voice_chan_id).await {
        Ok(_)  => format!("Joined {}", voice_chan_id.mention()),
        Err(e) => format!("Couldn't join: {e:#}"),
    };

    cmd.create_response(
        &ctx.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content),
        ),
    ).await?;
    Ok(())
}
