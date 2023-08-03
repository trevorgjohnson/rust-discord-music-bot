use std::time::Duration;

use serenity::{
    framework::standard::{Args, CommandResult},
    model::prelude::{EmojiId, Message},
    prelude::Context,
    utils::MessageBuilder,
};

use rand::prelude::*;

use crate::utils::{check_msg, format_duration};

pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;

    check_msg(msg.reply(ctx, "Joining").await);

    Ok(())
}

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

    let fetched_lock = manager.get(guild_id);

    if fetched_lock.is_none() {
        join(ctx, msg).await?
    };

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

pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}
