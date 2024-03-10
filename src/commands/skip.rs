use crate::{handler::MessageContext, utils::check_msg};

use anyhow::{Context, Result};
use serenity::all::MessageBuilder;

use super::{Command, TrackMetadata};

pub struct Skip;

impl Command for Skip {
    async fn call(ctx: MessageContext) -> Result<()> {
        let guild_id = ctx.msg.guild(&ctx.ctx.cache).unwrap().id;

        let manager = songbird::get(&ctx.ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        if let Some(handler_lock) = manager.get(guild_id) {
            let handler = handler_lock.lock().await;

            let queue = handler.queue();

            let curr_track = queue.current().context("No song is playing to skip")?;
            let curr_track = curr_track.typemap().read().await;
            let skipped_track_metadata = curr_track
                .get::<TrackMetadata>()
                .context("No metadata found")?;

            queue.skip()?;

            let response = format!(
                "Skipped **{}**\n",
                skipped_track_metadata
                    .title
                    .clone()
                    .unwrap_or("song".to_owned())
            );

            check_msg(ctx.msg.channel_id.say(&ctx.ctx.http, response).await);
        } else {
            check_msg(
                ctx.msg
                    .channel_id
                    .say(&ctx.ctx.http, "Queue is currently empty")
                    .await,
            );
        }

        Ok(())
    }

    fn description() -> String {
        String::from("**-skip**: skips the current song that im playing")
    }
}
