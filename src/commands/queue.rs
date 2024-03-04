use crate::utils::{check_msg, format_duration};
use serenity::{framework::standard::CommandResult, model::prelude::Message, prelude::Context};
use std::time::Duration;

pub async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).unwrap().id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue().current_queue();

        let queue_str = queue
            .iter()
            .enumerate()
            .map(|(idx, track_handle)| {
                format!(
                    "{}: **{}** (`{}`)\n",
                    idx + 1,
                    track_handle
                        .metadata()
                        .title
                        .clone()
                        .unwrap_or("idk lmfao".to_owned()),
                    format_duration(
                        track_handle
                            .metadata()
                            .duration
                            .unwrap_or(Duration::from_secs(0))
                    )
                )
            })
            .collect::<Vec<String>>()
            .join("");

        check_msg(msg.channel_id.say(&ctx.http, queue_str).await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Queue is currently empty")
                .await,
        );
    }

    Ok(())
}
