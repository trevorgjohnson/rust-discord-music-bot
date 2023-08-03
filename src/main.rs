mod commands;
mod utils;

use dotenv::dotenv;

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::GatewayIntents,
};
use songbird::SerenityInit;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }
}

#[group]
#[commands(join, play, queue, shuffle, skip, leave)]
struct General;

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    crate::commands::join(ctx, msg).await
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    crate::commands::play(ctx, msg, args).await
}

#[command]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    crate::commands::queue(ctx, msg).await
}

#[command]
#[only_in(guilds)]
async fn shuffle(ctx: &Context, msg: &Message) -> CommandResult {
    crate::commands::shuffle(ctx, msg).await
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    crate::commands::skip(ctx, msg).await
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    crate::commands::leave(ctx, msg).await
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // load all secrets in '.env' into the environment
    dotenv().ok();
    let token = std::env::var("TOKEN").expect("'TOKEN' was not found");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("-"))
        .group(&GENERAL_GROUP);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
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
