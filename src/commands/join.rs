use anyhow::{anyhow, Result};
use serenity::{
    model::{channel::Message, id::{ChannelId, GuildId}},
    prelude::Context
};
use serenity::model::mention::Mentionable;

pub async fn execute(ctx: &Context, msg: &Message) -> Result<()> {
    let guild_id = msg.guild_id.ok_or_else(|| anyhow!("DMs not supported"))?;

    let (voice_chan_id, guild_id_copy): (ChannelId, GuildId) = {
        let guild_cached = guild_id
            .to_guild_cached(&ctx.cache)
            .ok_or_else(|| anyhow!("guild not in cache"))?;

        let chan = guild_cached
            .voice_states
            .get(&msg.author.id)
            .and_then(|vs| vs.channel_id)
            .ok_or_else(|| anyhow!("you must be in a voice channel"))?;

        (chan, guild_id)
    };

    let manager = songbird::get(ctx).await.unwrap().clone();

    match manager.join(guild_id_copy, voice_chan_id).await {
        Ok(_call) => {
            msg.channel_id
                .say(&ctx.http, format!("Joined {}", voice_chan_id.mention()))
                .await?;
        }
        Err(err) => {
            msg.channel_id
                .say(&ctx.http, format!("Couldn't join: {err:#}"))
                .await?;
        }
    }

    Ok(())
}
