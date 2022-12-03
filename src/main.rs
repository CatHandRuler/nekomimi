use serenity::{
    async_trait,
    model::prelude::Ready,
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _status: Ready) {
        println!("Ready");
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected token; No token");
    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Err while creating Client");

    if let Err(why) = client.start().await {
        println!("Err with client: {:?}", why);
    }
}
