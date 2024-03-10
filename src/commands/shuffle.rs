use rand::seq::SliceRandom;

use crate::{handler::MessageContext, utils::check_msg};
use anyhow::Result;

use super::Command;

pub struct Shuffle;

impl Command for Shuffle {
    async fn call(ctx: MessageContext) -> Result<()> {
        let guild_id = ctx.msg.guild(&ctx.ctx.cache).unwrap().id;

        let manager = songbird::get(&ctx.ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();

        if let Some(handler_lock) = manager.get(guild_id) {
            let handler = handler_lock.lock().await;

            handler.queue().modify_queue(|queue| {
                if !queue.is_empty() {
                    let mut rng = rand::thread_rng();
                    let first_song = queue.pop_front().unwrap();
                    queue.make_contiguous().shuffle(&mut rng);
                    queue.push_front(first_song);
                }
            });

            check_msg(
                ctx.msg
                    .channel_id
                    .say(&ctx.ctx.http, "The queue has been shuffled")
                    .await,
            );
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
        String::from("**-shuffle**: shuffles all songs in the current queue")
    }
}
