use std::{env, path::Path};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fs::File;
use std::io::{self,BufRead};
use rand::seq::SliceRandom;



const QUOTE_COMMAND: &str = "!quote";
const DOCTOR_COMMAND: &str = "!doctor";
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == QUOTE_COMMAND
        {
            match get_random_quote("Quotes.txt")
            {
                Ok(quote) => 
                {
                    if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(error_message)=>
                {
                    if let Err(why) = msg.channel_id.say(&ctx.http, error_message).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
        if msg.content.starts_with(DOCTOR_COMMAND)
        {
            if let Some(doctor_number) = msg.content.strip_prefix(DOCTOR_COMMAND)
            {
                let file_name = format!("doctors_img/{}-doctor.jpg",doctor_number);
                let image_path = Path::new(&file_name);

                if image_path.exists()
                {

                
                    if let Err(why) = msg.channel_id.send_files(&ctx.http, vec![image_path], |m| {m.content(format!("Here is Doctor {}!", doctor_number))}).await
                    {
                        println!("Error sending image :{:?}",why);
                    }
                }
                else 
                {
                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("I couldn't find an image for doctor {}.",doctor_number)).await
                    {
                        println!("error sending message : {:?}",why);
                    }    
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
fn get_random_quote(file_path: &str) -> io::Result<String>
{
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let quotes: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    if let Some(random_quote) = quotes.choose(&mut rand::thread_rng())
    {
        Ok(random_quote.clone())
    }
    else 
    {
        Err(io::Error::new(io::ErrorKind::NotFound, "No quotes found"))
    }
           
    
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}