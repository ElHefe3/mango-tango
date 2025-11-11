use anyhow::{anyhow, Result};
use serenity::{
    all::{
        CommandInteraction, CreateCommand, CreateCommandOption, CommandOptionType,
        CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    model::id::{ChannelId, GuildId},
    prelude::*,
};
use serenity::model::mention::Mentionable;
use songbird::input::Input;

pub fn register() -> CreateCommand {
    CreateCommand::new("search")
        .description("Use a keyword to find a song")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "query", "What to search for")
                .required(true),
        )
}

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let gid = cmd.guild_id.ok_or_else(|| anyhow!("DMs not supported"))?;

    let imediate_response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("Searching...")
    );
    cmd.create_response(&ctx.http, imediate_response).await?;

    let query = cmd
        .data
        .options
        .iter()
        .find(|o| o.name == "query")
        .and_then(|o| o.value.as_str()) 
        .ok_or_else(|| anyhow!("missing 'query' string option"))?;

    let (voice_chan_id, guild_id_copy): (ChannelId, GuildId) = {
        let g = gid.to_guild_cached(&ctx.cache).ok_or_else(|| anyhow!("guild not in cache"))?;
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

    let content = format!("🔎 ▶️ queued first result for “{query}” in {}", voice_chan_id.mention());
    cmd.create_response(
        &ctx.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content),
        ),
    )
    .await?;

    Ok(())
}
