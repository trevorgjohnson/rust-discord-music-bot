use lavalink_rs::{
    model::{search::SearchEngines, track::TrackLoadData},
    player_context::TrackInQueue,
};

use crate::{commands::join::_join, Context, Error};

/// Play a song in the voice channel you are connected in.
#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search term or URL"]
    #[rest]
    term: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let has_joined = _join(&ctx, guild_id, None).await?;

    let lava_client = ctx.data().lavalink.clone();

    let Some(player) = lava_client.get_player_context(guild_id) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let Some(query) = term else {
        if let Ok(player_data) = player.get_player().await {
            let queue = player.get_queue();

            if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
                player.skip()?;
            } else {
                ctx.say("The queue is empty.").await?;
            }
        }

        return Ok(());
    };

    let query = if query.starts_with("http") {
        query
    } else {
        SearchEngines::YouTube.to_query(&query)?
    };

    let loaded_tracks = lava_client.load_tracks(guild_id, &query).await?;

    let mut playlist_info = None;
    let tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }

        _ => {
            ctx.say(format!("{:?}", loaded_tracks)).await?;
            return Ok(());
        }
    };

    let track_add_msg = match playlist_info {
        Some(info) => format!("Added playlist to queue: {}", info.name),
        None => {
            let track = &tracks[0].track;
            match &track.info.uri {
                Some(uri) => format!(
                    "Added to queue: [{} - {}](<{}>)",
                    track.info.author, track.info.title, uri
                ),
                None => format!(
                    "Added to queue: {} - {}",
                    track.info.author, track.info.title
                ),
            }
        }
    };
    ctx.say(track_add_msg).await?;

    let queue = player.get_queue();
    queue.append(tracks.into())?;

    if !has_joined {
        if let Ok(player_data) = player.get_player().await {
            if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
                player.skip()?;
            }
        }
    }

    Ok(())
}
