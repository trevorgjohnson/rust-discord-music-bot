use std::sync::Arc;

use anyhow::{Context, Result};
use serenity::{
    all::{CacheHttp, ChannelId, Http},
    async_trait,
    utils::MessageBuilder,
};
use songbird::{
    input::{Compose, YoutubeDl},
    Event, EventContext,
};

use crate::{
    handler::MessageContext,
    utils::{check_msg, format_duration},
    HttpKey,
};

use super::{Command, TrackMetadata};

pub struct Play;

impl Command for Play {
    async fn call(ctx: MessageContext) -> Result<()> {
        let url = match ctx.msg.content.split_once(' ') {
            Some((_, args)) => args.to_owned(),
            None => {
                check_msg(
                    ctx.msg
                        .channel_id
                        .say(&ctx.ctx.http, "Must provide a URL to a video or audio")
                        .await,
                );
                return Ok(());
            }
        };

        let do_search = !url.starts_with("http");

        let guild_id = ctx.msg.guild_id.unwrap();

        let http_client = {
            let data = ctx.ctx.data.read().await;
            data.get::<HttpKey>()
                .cloned()
                .expect("Guaranteed to exist in the typemap.")
        };

        let manager = songbird::get(&ctx.ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        if manager.get(guild_id).is_none() {
            super::Join::call(ctx.clone()).await?
        }

        if let Some(handler_lock) = manager.get(guild_id) {
            let mut handler = handler_lock.lock().await;

            let mut src = if do_search {
                YoutubeDl::new_search(http_client, url)
            } else {
                YoutubeDl::new(http_client, url)
            };

            let song_metadata = src.aux_metadata().await?;

            let song_title = song_metadata.clone().title.context("No title found")?;
            let song_duration = format_duration(
                song_metadata
                    .clone()
                    .duration
                    .context("No duration found")?,
            );

            let track_handle = handler.enqueue(src.into()).await;

            let _ = track_handle.add_event(
                Event::Track(songbird::TrackEvent::End),
                SongEndNotifier {
                    chan_id: ctx.msg.channel_id,
                    http: ctx.ctx.http.clone(),
                },
            );

            {
                let mut typemap = track_handle.typemap().write().await;
                typemap.insert::<TrackMetadata>(song_metadata.clone());
            }

            let response = MessageBuilder::new()
                .push("Added ")
                .push_bold(song_title)
                .push(" (")
                .push_mono(song_duration)
                .push(") at position ")
                .push_mono(format!("#{}", handler.queue().len()))
                .build();

            check_msg(ctx.msg.channel_id.say(&ctx.ctx.http, &response).await);
        } else {
            check_msg(
                ctx.msg
                    .channel_id
                    .say(&ctx.ctx.http, "Not in a voice channel to play in")
                    .await,
            );
        }

        Ok(())
    }

    fn description() -> String {
        String::from("**-play**: makes me play whatever garbage you tell me to _(as long as its on youtube. i aint playin no soundcloud slop)_")
    }
}
struct SongEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[async_trait]
impl songbird::EventHandler for SongEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        dbg!(&self.http, self.chan_id);
        if let EventContext::Track(track_list) = ctx {
            dbg!(track_list);
        }
        None
    }
}
