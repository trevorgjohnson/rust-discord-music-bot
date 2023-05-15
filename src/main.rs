mod weather;

use anyhow::anyhow;

use serenity::http::CacheHttp;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::*;
use serenity::{async_trait, model::prelude::GuildId};

use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot {
    weather_api_key: String,
    client: reqwest::Client,
    discord_guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let commands =
            GuildId::set_application_commands(&self.discord_guild_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| {
                        command.name("hello").description("Say Hello")
                    })
                    .create_application_command(|command| {
                        command
                            .name("weather")
                            .description("Display the weather")
                            .create_option(|option| {
                                option
                                    .name("place")
                                    .description("City for lookup forecast")
                                    .kind(CommandOptionType::String)
                                    .required(true)
                            })
                    })
            })
            .await
            .unwrap();

        info!("{:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let response_content = match command.data.name.as_str() {
                "hello" => "hello".to_owned(),
                "weather" => {
                    let argument = command
                        .data
                        .options
                        .iter()
                        .find(|opt| opt.name == "place")
                        .cloned();

                    let value = argument.unwrap().value.unwrap();
                    let place = value.as_str().unwrap();
                    let result =
                        weather::get_forecast(place, &self.weather_api_key, &self.client).await;

                    match result {
                        Ok(forecast) => {
                            format!(
                                "It is {}°C/{}°F in {}, {}",
                                forecast.current.temp_c,
                                forecast.current.temp_f,
                                forecast.location.name,
                                forecast.location.region
                            )
                        }
                        Err(err) => {
                            format!("Err: {}", err)
                        }
                    }
                }
                command => unreachable!("unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(err) = create_interaction_response.await {
                eprintln!("Cannot respond to slash command: {}", err)
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = secret_store
        .get("TOKEN")
        .expect("'DISCORD_TOKEN' was not found");
    let weather_api_key = secret_store
        .get("WEATHER_API_KEY")
        .expect("'WEATHER_API_KEY' was not found");
    let guild_id = secret_store
        .get("GUILD_ID")
        .expect("'GUILD_ID' was not found");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot {
            weather_api_key,
            client: reqwest::Client::new(),
            discord_guild_id: GuildId(guild_id.parse().unwrap()),
        })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
