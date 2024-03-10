use crate::{handler::MessageContext, utils::check_msg};
use anyhow::Result;

use super::Command;

pub struct Leave;

impl Command for Leave {
    async fn call(ctx: MessageContext) -> Result<()> {
        let guild_id = ctx.msg.guild_id.unwrap();

        let manager = songbird::get(&ctx.ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();
        let has_handler = manager.get(guild_id).is_some();

        if has_handler {
            if let Err(e) = manager.remove(guild_id).await {
                check_msg(
                    ctx.msg
                        .channel_id
                        .say(&ctx.ctx.http, format!("Failed: {:?}", e))
                        .await,
                );
            }

            check_msg(
                ctx.msg
                    .channel_id
                    .say(&ctx.ctx.http, "Left voice channel")
                    .await,
            );
        } else {
            check_msg(ctx.msg.reply(&ctx.ctx, "Not in a voice channel").await);
        }

        Ok(())
    }

    fn description() -> String {
        String::from("**-leave**: makes me leave the voice channel _(fuck you)_")
    }
}
