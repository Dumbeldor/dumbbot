#[macro_use]
extern crate lazy_static;

use serenity::{
    prelude::*,
};

mod settings;
mod handlers;

lazy_static! {
    static ref CONFIG: settings::Settings = settings::Settings::new().expect("confing can't be loaded");
}

#[tokio::main]
async fn main() {
    println!("Token : {}", &CONFIG.token);
    let mut client = Client::builder(&CONFIG.token)
        .event_handler(handlers::Handler)
        .await
        .expect("Err creating client");
    
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
