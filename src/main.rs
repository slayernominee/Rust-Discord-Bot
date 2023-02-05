use std::env;
use std::process::exit;
use std::io::{self, Read};

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use colored::Colorize;

struct Handler;

fn string_discriminator(discriminator: u16) -> String {
    let mut discriminator_str: String = String::new();

    if discriminator < 10 {
        discriminator_str = String::from("000");
        discriminator_str.push_str(&discriminator.to_string());
    } else if discriminator < 100 {
        discriminator_str = String::from("00");
        discriminator_str.push_str(&discriminator.to_string());
    } else if discriminator < 1000 {
        discriminator_str = String::from("0");
        discriminator_str.push_str(&discriminator.to_string());
    } else {
        discriminator_str.push_str(&discriminator.to_string());
    }
    
    return discriminator_str;
}

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        
        if msg.guild_id == None {
            // make the discriminator with 4 digits
            let discriminator = string_discriminator(msg.author.discriminator);
            println!("{}#{} wrote: {}", msg.author.name, discriminator, msg.content);
        } else {
            println!("A message on a guild was received ...");
        }

        /*
        if msg.content == "ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == "pong" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Ping!").await {
                println!("Error sending message: {:?}", why);
            }
        }
         */
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        let username = ready.user.name + "#" + &string_discriminator(ready.user.discriminator);
        println!("Logged in as {} on Discord!", username.red());
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = match env::var("DISCORD_TOKEN") {
        Ok(bearer) => bearer,
        Err(_) => {
            // handle the error if it couldnt get the DISCORD_TOKEN from the Environment
            println!("Couldnt get the Discord Token from the Environment!");
            println!("Please input the token directly here or stop the program and export the DISCORD_TOKEN as environemnt variable");

            let mut discord_token_input = String::new();

            io::stdin().read_line(&mut discord_token_input).expect("Couldnt get the console input!");

            discord_token_input
        }
    };
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}