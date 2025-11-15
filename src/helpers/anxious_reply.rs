use anyhow::Result;
use serenity::{
    all::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage},
    prelude::*,
};

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    cmd.create_response(&ctx.http,
        CreateInteractionResponse::Defer(
            CreateInteractionResponseMessage::new().ephemeral(true)
        )
    ).await?;

    Ok(())
}
