use std::env;

mod api;
mod messengers;

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Token must exist; Token not found.");
    let id = String::from("ID should be here");
    messengers::discord::client::start(&token, id).await;
}
