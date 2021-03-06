use std::env;
use lazy_static::lazy_static;
use regex::Regex;
use rand::{
    Rng,
    seq::SliceRandom,
};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};
use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
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
 
struct Handler {
    insults: Insults,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        
        let content = &msg.content;
        
        lazy_static! {
            static ref PARSER: Regex = Regex::new(r"^!([0-9a-z]+)( (.+))?$").unwrap();
        }

        let content_caps = match PARSER.captures(content) {
                Some(content_caps) => content_caps,
                None => return,
            };
        let command = match content_caps.get(1) {
            Some(command) => command.as_str(),
            None => return,
        };
        let body = match content_caps.get(3) {
            Some(body) => Some(body.as_str()),
            None => None,
        };


        
        
   
        //let mut rdr = csv::Reader::from_reader(io::stdin());


        if let Err(why) = match command {

            "peepeepoopoo" => {
                msg.channel_id.say(&ctx.http, HELP_MESSAGE).await 
            }
        
            "flip" => {
                let rng: i32 = rand::thread_rng().gen_range(0..1001);
                if rng < 500 {
                    msg.channel_id.say(&ctx.http, "Heads!").await
                } else if rng > 500 {
                    msg.channel_id.say(&ctx.http, "Tails!").await
                } else {
                    msg.channel_id.say(&ctx.http, "Edge!").await 
                }
            }

            "origin" => {
                msg.channel_id.say(&ctx.http, ORIGIN_STORY).await 
            }

            "ping" => {
                let channel = match msg.channel_id.to_channel(&ctx).await {
                    Ok(channel) => channel,
                    Err(why) => {
                        println!("Error getting channel: {:?}", why);

                        return;
                    },
                };

                let response = MessageBuilder::new()
                    .push("user ")
                    .push(&msg.author)
                    .push(" used the 'ping' command in the ")
                    .mention(&channel)
                    .push(" channel")
                    .build();

                msg.channel_id.say(&ctx.http, &response).await
            }

            "wrinkle" => {
                msg.channel_id.say(&ctx.http, "Bro my brain is so fuckin wrinkly right now!").await
            }
            
            _ => continue,
        } {
            println!("Error sending message: {:?}", why);
        }

        lazy_static! {
            static ref DICE_ROLL_RE: Regex = Regex::new(r"^!(\d*?)[Dd](\d+?)$").unwrap();
        }

        lazy_static! {
            static ref INSULT_RE: Regex = Regex::new(r"^!insult (<@!\d+>)").unwrap();
        }

        if DICE_ROLL_RE.is_match(&msg.content) {
            let caps = DICE_ROLL_RE.captures(content).unwrap();
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
        } else if INSULT_RE.is_match(&msg.content) {
            let victim = INSULT_RE.captures(content).unwrap();
            let victim = victim.get(1).map_or("", |m| m.as_str());
            if let Err(why) = msg.channel_id.say(&ctx.http, self.insults.say(victim))
                .await {
                println!("Error sending message {:?}", why);
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
    
    if number_of_dice > 1000 {
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



struct Insults {
    adjectives: Vec<String>,
    nouns: Vec<String>,
}

fn read_to_vec(file: &str) -> Vec<String> {
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
}

impl Insults {
    fn read_from_files(adjectives_file: &str, nouns_file: &str) -> Insults {
        Insults {
            adjectives: read_to_vec(adjectives_file),
            nouns: read_to_vec(nouns_file),
        }
    }
    fn say(&self, victim: &str) -> String {
        // insult logic here
        let mut rng = rand::thread_rng();

        format!("{} thou {} {} {}!", victim,
                self.adjectives.as_slice().choose(&mut rng).unwrap(),
                self.adjectives.as_slice().choose(&mut rng).unwrap(),
                self.nouns.as_slice().choose(&mut rng).unwrap())
    }
}
     

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler{
            insults: Insults::read_from_files(
                         "resources/insult-adjectives.txt",
                         "resources/insult-nouns.txt"),
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
