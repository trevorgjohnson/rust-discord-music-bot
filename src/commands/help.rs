use anyhow::{Context, Result};
use serenity::all::{ArgumentConvert, Emoji, EmojiId, MessageBuilder};
use strum::IntoEnumIterator;

use crate::{handler::MessageContext, utils::check_msg, HttpKey};

use super::{AvailableCommands, Command};

pub struct Help;

impl Command for Help {
    async fn call(ctx: MessageContext) -> Result<()> {
        let guild_id = ctx.msg.guild(&ctx.ctx.cache).unwrap().id;

        let chud_emoji =
            Emoji::convert(&ctx.ctx, Some(guild_id), None, "1202061487208927294").await?;
        // let chud_emoji = "<:lol:1202061487208927294:>";

        let mut response = MessageBuilder::new();

        response
            .push_line(format!(
                "Tf you need help for bruh are you stupid {} {} {}",
                chud_emoji, chud_emoji, chud_emoji,
            ))
            .push_line("")
            .push_line("aight bruh, here are the current available commands:")
            .push_line("");

        for command in AvailableCommands::iter() {
            response.push_line(command.get_description());
        }

        let response = &response.build();

        check_msg(ctx.msg.channel_id.say(&ctx.ctx.http, response).await);

        Ok(())
    }

    fn description() -> String {
        String::from("**-help**: returns all of the currently available commands")
    }
}
