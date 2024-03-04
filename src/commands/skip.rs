use crate::utils::check_msg;
use serenity::{framework::standard::CommandResult, model::prelude::Message, prelude::Context};

pub async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).unwrap().id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let mut skipped_song_title = String::from("");
        if let Some(current_metadata) = handler.queue().current() {
            skipped_song_title = current_metadata
                .metadata()
                .title
                .clone()
                .unwrap_or("".to_owned());
        }

        match handler.queue().skip() {
            Ok(_) => check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Skipped **{}**", skipped_song_title))
                    .await,
            ),
            Err(_) => check_msg(
                msg.channel_id
                    .say(&ctx.http, "There are no songs to skip")
                    .await,
            ),
        };
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Queue is currently empty")
                .await,
        );
    }

    Ok(())
}
