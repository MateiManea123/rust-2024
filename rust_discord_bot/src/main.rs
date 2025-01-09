use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use rusqlite::{params, Connection, OptionalExtension, Result as SqlResult};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
};
use std::fs::File;
use std::io::{self, BufRead};
use std::{env, path::Path, sync::Arc};
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

const QUOTE_COMMAND: &str = "!quote";
const DOCTOR_COMMAND: &str = "!doctor";
const EPISODE_COMMAND: &str = "!episode";
const SCORE_COMMAND: &str = "!score";
const WELCOME_MESSSAGE: &str = "Hello! I am a Discord Bot made for Doctor Who fans!
 I will periodically ask random trivia questions, the first who answers correctly gets a point!
 You can also use any of this commands at any time!
 - !quote
 - !doctor<number from 1-13>(no spaces allowed)
 - !episode <Slice from an episode>
 - !score";
struct TriviaData {
    current_question: Option<(String, String)>,
    last_correct: bool,
}

struct Handler {
    trivia: Arc<Mutex<TriviaData>>,
    conn: Arc<Mutex<Connection>>,
}

impl Handler {
    fn new(conn: Connection) -> Self {
        Self {
            trivia: Arc::new(Mutex::new(TriviaData {
                current_question: None,
                last_correct: true,
            })),
            conn: Arc::new(Mutex::new(conn)),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut trivia = self.trivia.lock().await;
        if let Some((_, correct_answer)) = &trivia.current_question {
            if msg.content.trim().eq_ignore_ascii_case(correct_answer) {
                if let Err(err) =
                    update_user_score(&self.conn, msg.author.id.0, &msg.author.name).await
                {
                    println!("Error updating score: {:?}", err);
                }

                trivia.last_correct = true;
                trivia.current_question = None;

                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("Correct! {} earned a point!", msg.author.name),
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
                return;
            }
        }

        if msg.content == SCORE_COMMAND {
            match get_user_score(&self.conn, msg.author.id.0, &msg.author.name).await {
                Ok(score) => {
                    if let Err(why) = msg
                        .channel_id
                        .say(&ctx.http, format!("{}'s score: {}", msg.author.name, score))
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(err) => println!("Error fetching score: {:?}", err),
            }
        }

        if msg.content == QUOTE_COMMAND {
            match get_random_quote("Quotes.txt") {
                Ok(quote) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, quote).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(error_message) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, error_message).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        } else if msg.content.starts_with(DOCTOR_COMMAND) {
            if let Some(doctor_number) = msg.content.strip_prefix(DOCTOR_COMMAND) {
                let file_name = format!("doctors_img/{}-doctor.jpg", doctor_number);
                let image_path = Path::new(&file_name);

                if image_path.exists() {
                    let result = msg
                        .channel_id
                        .send_files(&ctx.http, vec![image_path], |m| {
                            m.content(format!("Here is Doctor {}!", doctor_number))
                        })
                        .await;

                    if let Err(why) = result {
                        println!("Error sending image: {:?}", why);
                    }
                } else {
                    let result = msg
                        .channel_id
                        .say(
                            &ctx.http,
                            format!("I couldn't find an image for doctor {}.", doctor_number),
                        )
                        .await;

                    if let Err(why) = result {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        } else if msg.content.starts_with(EPISODE_COMMAND) {
            let query = msg
                .content
                .strip_prefix(EPISODE_COMMAND)
                .unwrap_or("")
                .trim();
            if query.is_empty() {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Please provide a slice of the title's episode.")
                    .await
                {
                    println!("Error sending message {:?}", why);
                }
                return;
            }
            match search_episode(query).await {
                Ok(response) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                        println!("Error sending message {:?}", why);
                    }
                }
                Err(err) => {
                    if let Err(why) = msg
                        .channel_id
                        .say(&ctx.http, format!("Error searching for episode {:?}", err))
                        .await
                    {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let channel_id = 1322549846867054615;
        let trivia = Arc::clone(&self.trivia);
        if let Err(why) = ChannelId(channel_id)
            .send_message(&ctx.http, |m| m.content(WELCOME_MESSSAGE))
            .await
        {
            println!("Error sending welcome message! {:?}", why);
        }
        tokio::spawn(async move {
            let questions = vec![
                ("Who is the first Doctor?", "William Hartnell"),
                ("What is the TARDIS?", "Time and Relative Dimension in Space"),
                ("Who has conditioned River Song to kill The Doctor?", "The Silence"),
                ("What is the name of the artificial life form with Rory's memories?", "Auton"),
                ("What animal head do the 'Judoon' have?", "Rhino"),
                ("What condiment can be used to dispose of the Slitheen?", "Vinegar"),
                ("What is the name of the alien life form that concealed itself in television signal?", "The Wire"),
                ("During the 'Dalek Civil War', what was the name of Davros' followers?", "Imperial Daleks"),
            ];

            loop {
                {
                    let mut trivia_data = trivia.lock().await;
                    if trivia_data.last_correct {
                        let mut rng = StdRng::from_entropy();
                        if let Some((question, answer)) = questions.choose(&mut rng) {
                            trivia_data.current_question =
                                Some((question.to_string(), answer.to_string()));
                            trivia_data.last_correct = false;

                            if let Err(why) = ChannelId(channel_id)
                                .send_message(&ctx.http, |m| {
                                    m.content(format!("Trivia time! {}", question))
                                })
                                .await
                            {
                                println!("Error sending question: {:?}", why);
                            }
                        }
                    }
                }
                time::sleep(Duration::from_secs(60)).await;
            }
        });
    }
}

fn get_random_quote(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let quotes: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut rng = StdRng::from_entropy();
    if let Some(random_quote) = quotes.choose(&mut rng) {
        Ok(random_quote.clone())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "No quotes found"))
    }
}

async fn search_episode(query: &str) -> SqlResult<String> {
    let conn = Connection::open("doctor-who-episodes/doctor_who.db")?;
    let mut stmt = conn.prepare(
        "SELECT e.title, e.runtime, s.season_id,s.serial 
         FROM episodes e
         JOIN serials s on s.title = e.title
         WHERE e.title LIKE ?",
    )?;

    let rows = stmt.query_map([format!("%{}%", query)], |row| {
        Ok((
            row.get::<_, String>(0)?, // Title
            row.get::<_, String>(1)?, // Runtime
            row.get::<_, i32>(2)?,    // Name
            row.get::<_, i32>(3)?,    // Episode Order
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        let (title, runtime, season, episode) = row?;
        results.push(format!(
            "Title: {}\nRuntime: {} minutes\nSeason: {}\nEpisode: {}",
            title, runtime, season, episode
        ));
    }

    if results.is_empty() {
        Ok("No episodes found.".to_string())
    } else {
        Ok(results.join("\n\n"))
    }
}

async fn get_user_score(
    conn: &Arc<Mutex<Connection>>,
    user_id: u64,
    username: &str,
) -> SqlResult<i32> {
    let conn = conn.lock().await;
    let mut stmt = conn.prepare("SELECT score FROM scores WHERE user_id = ?")?;
    if let Some(row) = stmt
        .query_row(params![user_id], |row| row.get(0))
        .optional()?
    {
        Ok(row)
    } else {
        conn.execute(
            "INSERT INTO scores (user_id, username, score) VALUES (?, ?, 0)",
            params![user_id, username],
        )?;
        Ok(0)
    }
}

async fn update_user_score(
    conn: &Arc<Mutex<Connection>>,
    user_id: u64,
    username: &str,
) -> SqlResult<()> {
    let conn = conn.lock().await;
    conn.execute(
        "INSERT INTO scores (user_id, username, score)
         VALUES (?, ?, 1)
         ON CONFLICT(user_id)
         DO UPDATE SET score = score + 1",
        params![user_id, username],
    )?;
    Ok(())
}

#[tokio::main]
async fn main() -> SqlResult<()> {
    let conn = Connection::open("trivia_scores.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS scores (
            user_id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            score INTEGER DEFAULT 0
        )",
        [],
    )?;

    let handler = Handler::new(conn);

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
