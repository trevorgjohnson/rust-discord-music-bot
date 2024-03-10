use crate::{handler::MessageContext, utils::check_msg};
use anyhow::Result;
use serenity::async_trait;
use songbird::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};

use super::Command;

pub struct Join;

impl Command for Join {
    async fn call(ctx: MessageContext) -> Result<()> {
        let (guild_id, channel_id) = {
            let guild = ctx.msg.guild(&ctx.ctx.cache).unwrap();
            let channel_id = guild
                .voice_states
                .get(&ctx.msg.author.id)
                .and_then(|voice_state| voice_state.channel_id);

            (guild.id, channel_id)
        };

        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                check_msg(ctx.msg.reply(&ctx.ctx, "Not in a voice channel").await);
                return Ok(());
            }
        };

        let manager = songbird::get(&ctx.ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
            let mut handler = handler_lock.lock().await;
            handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
        }

        Ok(())
    }

    fn description() -> String {
        String::from("**-join**: makes me join the current voice channel _(i report everything i hear to the fbi)_")
    }
}

struct TrackErrorNotifier;

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}
