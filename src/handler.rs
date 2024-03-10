use crate::{commands::AvailableCommands, utils::check_msg};

use serenity::{
    all::Message,
    async_trait,
    client::{Context, EventHandler},
    model::gateway::Ready,
};

pub struct Handler;

#[derive(Clone)]
pub struct MessageContext {
    pub ctx: Context,
    pub msg: Message,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name)
    }

    async fn message(&self, ctx: Context, msg: Message) {
        println!("Message recieved: {}", msg.content);

        let cmd_str: &str = match msg.content.split_once(' ') {
            Some((cmd, _)) => cmd,
            None => &msg.content,
        };

        let cmd = match cmd_str.parse::<AvailableCommands>() {
            Ok(c) => c,
            Err(_) => return,
        };

        let _ = cmd
            .call_func(MessageContext {
                ctx: ctx.clone(),
                msg: msg.clone(),
            })
            .await
            .map_err(|e| tracing::error!("{:?}", e));
    }
}
