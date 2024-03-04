use std::time::Duration;

use serenity::{
    framework::standard::{Args, CommandResult},
    model::prelude::{EmojiId, Message},
    prelude::Context,
    utils::MessageBuilder,
};

use crate::utils::{check_msg, format_duration};

pub async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let search = args.raw().collect::<Vec<&str>>().join(" ");

    if search.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Cannot be a url").await);
        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if manager.get(guild_id).is_none() {
        super::join::join(ctx, msg).await?
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::input::ytdl_search(&search).await {
            Ok(source) => source,
            Err(err) => {
                println!("Error starting source: {:?}", err);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing url").await);

                return Ok(());
            }
        };

        let song_metadata = source.metadata.clone();
        let song_title = song_metadata
            .title
            .clone()
            .unwrap_or("idk lmfao".to_owned());
        let song_duration = song_metadata.duration.unwrap_or(Duration::from_secs(0));

        let song_duration_str = format_duration(song_duration);

        handler.enqueue_source(source);

        let emoji = match guild.emoji(&ctx.http, EmojiId(1130908498985758842)).await {
            Ok(emoji) => emoji,
            Err(_) => {
                check_msg(
                    msg.channel_id
                        .say(&ctx.http, "bruh idk what emoji that is")
                        .await,
                );
                return Ok(());
            }
        };

        let response = MessageBuilder::new()
            .push("Playing ")
            .push_bold(song_title)
            .push(" (")
            .push_mono(song_duration_str)
            .push(")")
            .push_line("")
            .emoji(&emoji)
            .emoji(&emoji)
            .emoji(&emoji)
            .emoji(&emoji)
            .emoji(&emoji)
            .emoji(&emoji)
            .emoji(&emoji)
            .build();

        check_msg(msg.channel_id.say(&ctx.http, &response).await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}
