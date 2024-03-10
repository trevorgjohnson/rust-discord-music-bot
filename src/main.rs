mod commands;
mod handler;
mod utils;

use dotenv::dotenv;
use handler::Handler;

use reqwest::Client as HttpClient;
use serenity::{client::Client, prelude::GatewayIntents};
use songbird::{typemap::TypeMapKey, SerenityInit};

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // load all secrets in '.env' into the environment
    dotenv().ok();
    let token = std::env::var("TOKEN").expect("'TOKEN' was not found");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let http_client = HttpClient::new();

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .type_map_insert::<HttpKey>(http_client)
        .register_songbird()
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|err| println!("Client ended: {:?}", err));
    });

    let _ = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}
