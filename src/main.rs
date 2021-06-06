use std::env;
use regex::Regex;
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
<https://github.com/Sonic1594/rust_discord_help_bot/tree/master>
";
const HELP_MESSAGE: &str = "

peepeepoopoo

";
const HEADS: &str = "heads!";
const TAILS: &str = "tails!";
const EDGE: &str = "edge!";

 
const HELP_COMMAND: &str = "!peepeepoopoo";
const ORIGIN_COMMAND: &str = "!origin";
const COIN_FLIP_COMMAND: &str = "!flip";


struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx: Context, msg: Message) {

        let content: &str = &msg.content;
        
        let gex = Regex::new(r"^!(\d*?)[Dd](\d+?)$").unwrap();
    
        match content {

            HELP_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        
            COIN_FLIP_COMMAND => {
                let rng: i32 = rand::thread_rng().gen_range(0..1002);
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

            ORIGIN_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, ORIGIN_STORY).await {
                    println!("Error sending message: {:?}", why);
                }
            }

            _ => (),
        }
        if gex.is_match(content) {
            let caps = gex.captures(content).unwrap();
            let front = caps.get(1).map_or("", |m| m.as_str());
            let back = caps.get(2).map_or("", |m| m.as_str());

            if parse_dice(front, back).is_ok() {
                let roll = roll_dice(parse_dice(front, back).unwrap());
                let s: String = roll.to_string();
                let s = &s;
                if let Err(why) = msg.channel_id.say(&ctx.http, s).await {
                    println!("Error sending message {:?}", why);
                }
            } else {
                if let Err(s) = parse_dice(front, back) {
                    if let Err(why) = msg.channel_id.say(&ctx.http, s).await {
                        println!("Error sending message {:?}", why);
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn parse_dice<'a> (front: &'a str, back: &'a str) -> Result<(i32, i32), &'a str> {

    let dice_type = match back.parse::<i32>() {
        Ok(num) => num,
        Err(_) => return Err("Error: Message could not be read."),
    };

    // STANDARD DND DICE
    // filters non-real dice type
    if !vec!{4, 6, 8, 10, 12, 20}.contains(&dice_type) {
        return Err("Error: This dice type is not supported.");
    }

    let number_of_dice = front.parse::<i32>().unwrap_or(1);
    
    if number_of_dice > 100 {
        return Err("I can only roll up to 1000 dice at a time.");
    }

    let tuple: (i32, i32) = (number_of_dice, dice_type);
    Ok(tuple)
  
}

fn roll_dice (mut dice: (i32, i32)) -> i32 {
    // created mutable total which will be the sum of the rolls added
    let mut total: i32 = 0;
  
    // rand to simulate rolls
    let mut roll = rand::thread_rng();

    while dice.0 > 0 {
        total += roll.gen_range(0..dice.1)+1;
        dice.0 -= 1;
    }
  
    total
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
