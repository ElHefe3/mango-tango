use anyhow::{anyhow, Result};
use serenity::{
    all::{
        CommandInteraction,
        CommandOptionType,
        CreateCommand,
        CreateCommandOption,
        CreateInteractionResponseFollowup,
        CreateActionRow,
        CreateButton,
    },
    model::id::{ChannelId, GuildId},
    prelude::*,
};
use serenity::model::mention::Mentionable;

use crate::adapters::youtube::yt_search;
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

        let gid = cmd
            .guild_id
            .ok_or_else(|| anyhow!("DMs not supported"))?;

        let query = cmd
            .data
            .options
            .iter()
            .find(|o| o.name == "query")
            .and_then(|o| o.value.as_str())
            .ok_or_else(|| anyhow!("missing 'query' string option"))?;

        let (voice_chan_id, _guild_id_copy): (ChannelId, GuildId) = {
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

        let results = yt_search(query, 5).await?;
        if results.is_empty() {
            cmd.create_followup(
                &ctx.http,
                CreateInteractionResponseFollowup::new()
                    .content(format!("🔎 No results found for “{query}”."))
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }

        helpers::search_state::store_results(gid, cmd.user.id, results.clone());

        use std::fmt::Write as _;
        let mut content = String::new();

        let _ = writeln!(
            &mut content,
            "🔎 Results for “{}” in {}:\n",
            query,
            voice_chan_id.mention()
        );

        for (idx, r) in results.iter().enumerate() {
            let n = idx + 1;
            let dur = r
                .duration
                .map(|d| format!("{}:{:02}", d / 60, d % 60))
                .unwrap_or_else(|| "?".to_string());

            let _ = writeln!(
                &mut content,
                "{n}. **{}** ({dur})\n   <{}>",
                r.title,
                r.url
            );
        }

        let _ = writeln!(
            &mut content,
            "\n▶️ Click one of the buttons below to queue that result."
        );

        let mut buttons = Vec::new();
        for (idx, _r) in results.iter().enumerate().take(5) {
            let label = (idx + 1).to_string();
            let custom_id = format!("search_pick:{idx}");

            buttons.push(
                CreateButton::new(custom_id)
                    .label(label),
            );
        }
        let row = CreateActionRow::Buttons(buttons);

        cmd.create_followup(
            &ctx.http,
            CreateInteractionResponseFollowup::new()
                .content(content)
                .components(vec![row]),
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
