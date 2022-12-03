use serenity::{
    async_trait,
    model::{
        prelude::{Activity, Channel, Guild, Ready},
        user::OnlineStatus,
        Permissions,
    },
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: bool) {
        let channels = guild.channels;
        for (_channel_id, channel) in channels {
            match channel {
                Channel::Guild(guild_channel) => {
                    if guild_channel.is_text_based() {
                        if let Ok(my_permissions) = guild_channel
                            .permissions_for_user(&ctx.cache, &ctx.cache.current_user().id)
                        {
                            if my_permissions.contains(Permissions::SEND_MESSAGES) {
                                guild_channel
                                    .send_message(&ctx, |create_message| {
                                        create_message
                                            .content("이 서버에 추가해 주셔서 감사합니다!")
                                    })
                                    .await
                                    .expect("An error occurred while sending the message.");
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    async fn ready(&self, ctx: Context, status: Ready) {
        ctx.set_presence(Some(Activity::listening("앨랠래")), OnlineStatus::Online)
            .await;
        println!("Login succeeded as user {}.", &status.user.name);
    }
}

pub async fn start(token: &String, _id: String) {
    let mut client = Client::builder(token, GatewayIntents::GUILDS)
        .event_handler(Handler)
        .await
        .expect("An error occurred while creating the client.");

    if let Err(error) = client.start().await {
        println!("Error on client: {:?}", error);
    };
}
