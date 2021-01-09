#[macro_use]
extern crate lazy_static;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

mod settings;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new().expect("confing can't be loaded");
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        
        if msg.content == "!messageme" {
            let dm = msg.author.dm(&ctx, |m| {
                m.content("Hello!");
                m
            }).await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    println!("Token : {}", &CONFIG.token);
    let mut client = Client::builder(&CONFIG.token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
