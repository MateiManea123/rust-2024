use std::{env, path::Path};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fs::File;
use std::io::{self,BufRead};
use rand::seq::SliceRandom;
use rusqlite::{Connection, Result as SqlResult};


const QUOTE_COMMAND: &str = "!quote";
const DOCTOR_COMMAND: &str = "!doctor";
const EPISODE_COMMAND: &str = "!episode";
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
        if msg.content.starts_with(EPISODE_COMMAND)
        {
            let query = msg.content.strip_prefix(EPISODE_COMMAND).unwrap_or("").trim();
            if query.is_empty()
            {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Please provide a slice of the title's episode.").await
                {
                    println!("Error sending message {:?}",why);
                }
                return ;
            }
            match search_episode(query).await
            {
                Ok(response) =>
                {
                    if let Err(why) = msg.channel_id.say(&ctx.http, response).await
                    {
                        println!("Error sending message {:?}",why);
                    }
                }
                Err(err)=>
                {
                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Error searching for episode {:?}", err)).await
                    {
                        println!("Error sending message : {:?}", why);
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
async fn search_episode(query: &str) ->SqlResult<String>
{
    let conn = Connection::open("doctor-who-episodes/doctor_who.db")?;
    let mut stmt = conn.prepare(
        "SELECT e.title, e.runtime, s.season_id,s.serial 
         FROM episodes e
         JOIN serials s on s.title = e.title
         WHERE e.title LIKE ?",
    )?;

    let rows = stmt.query_map([format!("%{}%", query)], |row| 
    {
        Ok((
            row.get::<_, String>(0)?,  // Title
            row.get::<_, String>(1)?,    // Runtime
            row.get::<_, i32>(2)?,    // Name
            row.get::<_, i32>(3)?,    // Episode Order
        ))
    })?;


    let mut results = Vec::new();
    for row in rows 
    {
        let (title, runtime, season, episode) = row?;
        results.push(format!(
            "Title: {}\nRuntime: {} minutes\nSeason: {}\nEpisode: {}",title, runtime, season, episode
        ));
    }

    if results.is_empty() 
    {
        Ok("No episodes found.".to_string())
    } 
    else 
    {
        Ok(results.join("\n\n"))
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