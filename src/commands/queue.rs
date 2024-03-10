use crate::{
    commands::TrackMetadata,
    handler::MessageContext,
    utils::{check_msg, format_duration},
};
use std::time::Duration;

use anyhow::{Context, Result};
use serenity::futures;
use songbird::tracks::TrackHandle;
use tokio::{self, task::JoinError};

use super::Command;

pub struct Queue;

impl Command for Queue {
    async fn call(ctx: MessageContext) -> Result<()> {
        let guild_id = ctx.msg.guild(&ctx.ctx.cache).unwrap().id;

        let manager = songbird::get(&ctx.ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        let handler_lock = manager.get(guild_id).context("no handler lock")?;
        let handler = handler_lock.lock().await;

        let queue = handler.queue().current_queue();

        if queue.is_empty() {
            check_msg(
                ctx.msg
                    .channel_id
                    .say(&ctx.ctx.http, "Queue is currently empty")
                    .await,
            );
            return Ok(());
        }

        let queue_futures = queue.into_iter().enumerate().map(|(idx, track_handle)| {
            tokio::task::spawn(async move { get_queue_track_info(idx, track_handle).await })
        });

        let queue_str = futures::future::join_all(queue_futures)
            .await
            .into_iter()
            .collect::<Result<Vec<String>, JoinError>>()?
            .join("");

        check_msg(ctx.msg.channel_id.say(&ctx.ctx.http, queue_str).await);

        Ok(())
    }

    fn description() -> String {
        String::from("**-queue**: returns all of the current songs in the queue")
    }
}

async fn get_queue_track_info(idx: usize, track_handle: TrackHandle) -> String {
    let track = track_handle.typemap().read().await;
    match track.get::<TrackMetadata>() {
        Some(metadata) => format!(
            "{}: **{}** (`{}`)\n",
            idx + 1,
            metadata.title.clone().unwrap_or("idk lmfao".to_owned()),
            format_duration(metadata.duration.unwrap_or(Duration::from_secs(0)))
        ),
        None => format!("{}: `Uhh idk tf this song is lmfao`\n", idx + 1),
    }
}
