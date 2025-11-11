use anyhow::Result;
use serenity::{
    all::{CommandInteraction, CreateInteractionResponse, CreateInteractionResponseMessage},
    builder::CreateCommand,
    prelude::*,
};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A simple ping command")
}

pub async fn run(ctx: &Context, cmd: &CommandInteraction) -> Result<()> {
    let resp = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().content("🏓 Pong!")
    );
    cmd.create_response(&ctx.http, resp).await?;
    Ok(())
}
