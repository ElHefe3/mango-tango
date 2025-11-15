use anyhow::{anyhow, Result};
use serenity::{
    all::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage},
    builder::CreateCommand,
    prelude::*,
};

pub fn register() -> CreateCommand {
    CreateCommand::new("skip").description("Skip the current song")
}

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let guild_id = cmd
        .guild_id
        .ok_or_else(|| anyhow!("DMs not supported"))?;

    let manager = songbird::get(ctx)
        .await
        .ok_or_else(|| anyhow!("Songbird Voice client not initialized"))?
        .clone();

    let Some(handler_lock) = manager.get(guild_id) else {
        let resp = CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("❌ I'm not in a voice channel.")
                .ephemeral(true),
        );
        cmd.create_response(&ctx.http, resp).await?;
        return Ok(());
    };

    let handler = handler_lock.lock().await;
    let queue = handler.queue();

    let content = if queue.is_empty() {
        "⏭️ Nothing to skip — the queue is empty.".to_string()
    } else if queue.len() <= 1 {
        queue.stop();
        "⏹️ No next track queued — stopped playback.".to_string()
    } else {
        match queue.skip() {
            Ok(()) => "⏭️ Skipped to the next track.".to_string(),
            Err(err) => {
                queue.stop();
                format!("⏹️ Couldn't skip ({err}). Stopped playback.")
            }
        }
    };

    let resp = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content(content)
            .ephemeral(true),
    );
    cmd.create_response(&ctx.http, resp).await?;

    Ok(())
}
