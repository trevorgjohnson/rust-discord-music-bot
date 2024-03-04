use crate::utils::check_msg;
use rand::seq::SliceRandom;
use serenity::{framework::standard::CommandResult, model::prelude::Message, prelude::Context};

pub async fn shuffle(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).unwrap().id;

    let manager = songbird::get(ctx)
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
            msg.channel_id
                .say(&ctx.http, "The queue has been shuffled")
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Queue is currently empty")
                .await,
        );
    }

    Ok(())
}
