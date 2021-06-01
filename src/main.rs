use std::env;

use rand::Rng;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const ORIGIN_STORY: &str = "
Hello! My name is dingding.

I was created by my master Adam on the twenty-eigth day,
of the fifth month, on the two thousand and twenty-first year after
christ.

I am coded in rust, and my files can be found here
<git url placeholder>
";
const HELP_MESSAGE: &str = "

peepeepoopoo

";
const HEADS: &str = "heads!";
const TAILS: &str = "tails!";
const EDGE: &str = "edge!";

const D12: [&str; 12] = ["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];
 
const HELP_COMMAND: &str = "!peepeepoopoo";
const ORIGIN_COMMAND: &str = "!origin";
const COIN_FLIP_COMMAND: &str = "!flip";
const D12_COMMAND: &str = "!d12";


struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx: Context, msg: Message) {

        let content: &str = &msg.content;

        match content {

            HELP_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        
            COIN_FLIP_COMMAND => {
                let rng = rand::thread_rng().gen_range(0..1002);
                if rng <= 500 {
                    if let Err(why) = msg.channel_id.say(&ctx.http, HEADS).await {
                        println!("Error sending message: {:?}", why);
                    }
                } else if rng > 500 && rng < 1000 {
                    if let Err(why) = msg.channel_id.say(&ctx.http, TAILS).await {
                        println!("Error sending message: {:?}", why);
                    }
               } else {
                    if let Err(why) = msg.channel_id.say(&ctx.http, EDGE).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
         }

            D12_COMMAND => {
                let rng = rand::thread_rng().gen_range(1..13);
                if let Err(why) = msg.channel_id.say(&ctx.http, D12[rng-1]).await {
                    println!("Error sending message: {:?}", why);
                }
            }

            ORIGIN_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, ORIGIN_STORY).await {
                    println!("Error sending message: {:?}", why);
                }
            }

            _ => (),
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
