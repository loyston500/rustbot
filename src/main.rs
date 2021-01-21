use std::env;

use serenity::async_trait;
use serenity::client::{Client, EventHandler};
// use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group
    }
};

mod commands;
mod utils;
use commands::ping::*;
use commands::misc_commands::*;

#[group]
#[commands(ping, wait, multiply, parse, emb)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("2b.")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
