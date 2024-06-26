use lavalink_rs::{hook, model::events, prelude::*};
use poise::serenity_prelude::{model::id::ChannelId, Http};

#[hook]
pub async fn raw_event(_: LavalinkClient, session_id: String, event: &serde_json::Value) {
    if event["op"].as_str() == Some("event") || event["op"].as_str() == Some("playerUpdate") {
        info!("{:?} -> {:?}", session_id, event);
    }
}

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    client.delete_all_player_contexts().await.unwrap();
    info!("{:?} -> {:?}", session_id, event);
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &events::TrackStart) {
    let (channel_id, http) = {
        let player_context = client.get_player_context(event.guild_id).unwrap();
        let data = player_context
            .data::<(ChannelId, std::sync::Arc<Http>)>()
            .unwrap();
        (data.0, data.1.clone())
    };

    let track = &event.track;
    let msg = match &track.info.uri {
        Some(uri) => format!(
            "Now playing: [{} - {}](<{}>)",
            track.info.author, track.info.title, uri
        ),
        None => format!("Now playing: {} - {}", track.info.author, track.info.title),
    };

    let _ = channel_id.say(http, msg).await;
}
