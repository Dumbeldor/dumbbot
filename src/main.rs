#[macro_use]
extern crate lazy_static;

use serenity::{
    prelude::*,
};

mod settings;
mod handlers;

#[tokio::main]
async fn main() {
    let mut client = Client::builder(&settings::CONFIG.discord.token)
        .event_handler(handlers::Handler)
        .await
        .expect("Err creating client");
    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
