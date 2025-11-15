use anyhow::Result;
use serenity::{
    all::{
        CommandInteraction,
        CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    prelude::Context,
};

use crate::lifecycle::status::StatusManager;

pub fn register() -> serenity::all::CreateCommand {
    serenity::all::CreateCommand::new("status")
        .description("Show the current status of the bot")
}

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let status_mgr = StatusManager::global();
    let snapshot = status_mgr.snapshot().await;

    let last_error = snapshot
        .last_error
        .as_deref()
        .unwrap_or("none");

    let content = format!(
        "**Bot status**\n\
        phase: `{}`\n\
        uptime: `{}`\n\
        last error: `{}`",
        snapshot.phase,
        snapshot.uptime_human(),
        last_error,
    );

    cmd.create_response(
        &ctx.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content),
        ),
    )
    .await?;

    Ok(())
}
