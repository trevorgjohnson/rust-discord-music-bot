use std::sync::Arc;

use poise::serenity_prelude::{self as serenity};

use crate::{Context, Error};

pub async fn _join(
    ctx: &Context<'_>,
    guild_id: serenity::GuildId,
    channel_id: Option<serenity::ChannelId>,
) -> Result<bool, Error> {
    let lava_client = ctx.data().lavalink.clone();

    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();

    if lava_client.get_player_context(guild_id).is_some() {
        return Ok(false);
    }

    let connect_to = match channel_id {
        Some(id) => id,
        None => {
            let user_channel_id = ctx
                .guild()
                .unwrap()
                .voice_states
                .get(&ctx.author().id)
                .and_then(|voice_state| voice_state.channel_id);

            match user_channel_id {
                Some(id) => id,
                None => return Err("Not in a voice channel".into()),
            }
        }
    };

    let handler = manager.join_gateway(guild_id, connect_to).await;

    match handler {
        Ok((connection_info, _)) => {
            lava_client
                .create_player_context_with_data(
                    guild_id,
                    connection_info,
                    Arc::new((ctx.channel_id(), ctx.serenity_context().http.clone())),
                )
                .await?;

            Ok(true)
        }
        Err(why) => {
            ctx.say(format!("Error joining the channel: {}", why))
                .await?;
            Err(why.into())
        }
    }
}

/// Join the specified voice channel or the one you are currently in.
#[poise::command(slash_command, prefix_command)]
pub async fn join(
    ctx: Context<'_>,
    #[description = "The channel ID to join to."]
    #[channel_types("Voice")]
    channel_id: Option<serenity::ChannelId>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    _join(&ctx, guild_id, channel_id).await?;

    Ok(())
}
