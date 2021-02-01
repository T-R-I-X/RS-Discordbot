use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(issue) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", issue);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("[CLIENT] {} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("[FILE] Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("[FILE] Error creating client");

    if let Err(why) = client.start().await {
        println!("[CLIENT] Error: {:?}", why);
    }
}