use anyhow::Result;
use serenity::{
    all::Interaction, 
    async_trait, model::{gateway::Ready}, prelude::*
};
use songbird::SerenityInit;
use tokio::signal;

use crate::{helpers, init, slash_commands, middleware};
use crate::lifecycle::status::{StatusManager, Phase};

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("Logged in as {}", ready.user.name);

        let bot_status = StatusManager::global();
        bot_status.set_phase(Phase::Starting).await;

        if let Err(e) = slash_commands::register_all(&ctx.http).await {
            tracing::error!("command registration failed: {e:#}");
        }

        if let Err(e) = init::after_ready(&ctx).await {
            tracing::error!("after_ready failed: {e:#}");
            bot_status.set_phase(Phase::Errored).await;
            bot_status
                .set_error(format!("after_ready failed: {e:#}"))
                .await;
        } else {
            bot_status.clear_error().await;
            bot_status.set_phase(Phase::Ready).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            let command = cmd.data.name.as_str();

            if command != "ping" {
                if let Err(_e) = helpers::join_channel::run(&ctx, &cmd).await {
                    return;
                }
            }

            match command {
                "ping" => { let _ = slash_commands::ping::run(&ctx, &cmd).await; }
                "status" => { let _ = slash_commands::status::run(&ctx, &cmd).await; }
                "search" => {
                    middleware::idle::run_music_with_idle(
                        &ctx,
                        &cmd,
                        slash_commands::search::run(&ctx, &cmd),
                    )
                    .await;
                }
                "play_link" => {
                    middleware::idle::run_music_with_idle(
                        &ctx,
                        &cmd,
                        slash_commands::play_link::run(&ctx, &cmd),
                    )
                    .await;
                }
                "skip" => { let _ = slash_commands::skip::run(&ctx, &cmd).await; }
                _ => {}
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

        {
            let shard_manager = client.shard_manager.clone();

            tokio::spawn(async move {
                if let Err(e) = signal::ctrl_c().await {
                    tracing::error!("ctrl_c signal handler error: {e:#}");
                    return;
                }

                let status = StatusManager::global();
                status.set_phase(Phase::ShuttingDown).await;
                status.clear_error().await;

                tracing::info!("Shutting down shards…");
                shard_manager.shutdown_all().await;
            });
        }

    client.start().await?;
    Ok(())
}
