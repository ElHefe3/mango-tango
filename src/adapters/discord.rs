use anyhow::Result;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use songbird::SerenityInit;

use crate::commands;
use crate::init;

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("Logged in as {}", ready.user.name);

        if let Err(e) = init::after_ready(&ctx).await {
            tracing::error!("after_ready failed: {e:#}");
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let content = msg.content.trim();

        if content == "!join" {
            if let Err(e) = commands::join::execute(&ctx, &msg).await {
                let _ = msg.channel_id
                    .say(&ctx.http, format!("join error: {e:#}"))
                    .await;
            }
        } else if let Some(rest) = content.strip_prefix("!play ") {
            if let Err(e) = commands::play_link::execute(&ctx, &msg, rest.trim()).await {
                let _ = msg.channel_id
                    .say(&ctx.http, format!("play error: {e:#}"))
                    .await;
            }
        } else if let Some(rest) = content.strip_prefix("!search ") {
            if let Err(e) = crate::commands::search::execute(&ctx, &msg, rest.trim()).await {
                let _ = msg.channel_id
                    .say(&ctx.http, format!("search error: {e:#}"))
                    .await;
            }
        } else if content == "!leave" {
            if let Some(gid) = msg.guild_id {
                let manager = songbird::get(&ctx).await.unwrap().clone();
                let _ = manager.remove(gid).await;
                let _ = msg.channel_id.say(&ctx.http, "👋 left voice").await;
            }
        }
    }
}

pub async fn run(token: String) -> Result<()> {
    init::app_startup()?;

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = serenity::Client::builder(token, intents)
        .event_handler(Bot)
        .register_songbird()
        .await?;

    client.start().await?;
    Ok(())
}
