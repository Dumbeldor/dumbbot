use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::settings;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!help" {
            if let Err(why) = msg.channel_id.say(&ctx.http, &settings::CONFIG.messages.help).await {
                println!("Error sending message: {:?}", why);
            }
        }

        else if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        
        else if msg.content == "!messageme" {
            let dm = msg.author.dm(&ctx, |m| {
                m.content("Hello!");
                m
            }).await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {:?}", why);
            }
        }

        else if msg.content == "!tabarnak" {

        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}