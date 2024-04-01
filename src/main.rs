#[macro_use]
extern crate tracing;

mod commands;
mod music_events;

use lavalink_rs::{model::events, prelude::*};

use poise::serenity_prelude::{self as serenity, Context as SerenityContext};
use songbird::SerenityInit;

pub struct Data {
    pub lavalink: LavalinkClient,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Source '.env' file
    dotenv::dotenv()?;

    std::env::set_var("RUST_LOG", "info,lavalink_rs=trace");
    tracing_subscriber::fmt::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::clear::clear(),
                commands::join::join(),
                commands::leave::leave(),
                commands::pause::pause(),
                commands::play::play(),
                commands::queue::queue(),
                commands::resume::resume(),
                commands::skip::skip(),
                commands::stop::stop(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("-".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                let client = new_lavalink_client(ctx).await;
                Ok(Data { lavalink: client })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(
        std::env::var("TOKEN").expect("Unable to get 'TOKEN' env var"),
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
    )
    .register_songbird()
    .framework(framework)
    .await?;

    client.start().await?;

    Ok(())
}

async fn new_lavalink_client(ctx: &SerenityContext) -> LavalinkClient {
    let events = events::Events {
        raw: Some(music_events::raw_event),
        ready: Some(music_events::ready_event),
        track_start: Some(music_events::track_start),
        ..Default::default()
    };

    let node_local = NodeBuilder {
        hostname: "localhost:2333".to_string(),
        is_ssl: false,
        events: events::Events::default(),
        password: std::env::var("LAVALINK_PASSWORD")
            .expect("Unable to get 'LAVALINK_PASSWORD' env var"),
        user_id: ctx.cache.current_user().id.into(),
        session_id: None,
    };

    LavalinkClient::new(
        events,
        vec![node_local],
        NodeDistributionStrategy::round_robin(),
    )
    .await
}
