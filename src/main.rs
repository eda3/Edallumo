use std::{//fs::File, io::Read,
    sync::Arc, env, collections::HashSet};

use serde::{Deserialize, Serialize};
use serenity::{async_trait,
    model::gateway::Ready,
    prelude::*,
    client::bridge::gateway::ShardManager,
    http::Http, framework::StandardFramework,
    framework::standard::macros::group};

mod commands;
mod init_check;

use commands::{frames::*, update::*, print_moves::*, print_aliases::*, help::*};

#[group]
#[commands(frames, update, moves, aliases, help)]
struct General;

#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    page: String,
    link: String,
    pageid: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frames {
    input: String,
    r#move: String,
    damage: String,
    guard: String,
    invincibility: String,
    startup: String,
    block: String,
    hit: String,
    active: String,
    recovery: String,
    counter: String,
    img_link: String,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct MoveAliases {
    input: String,
    aliases: Vec<String>,
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer{
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, _: Context, ready: Ready) {
        println!("\n{} is connected!", ready.user.name);
        init_check::init_check();
    }
}

pub const CHARS: ([&str; 19], [u16; 19]) = (
    ["Jack-O", "Nagoriyuki", "Millia_Rage", "Chipp_Zanuff", "Sol_Badguy", "Ky_Kiske", "May", "Zato-1", "I-No", "Happy_Chaos", "Baiken", "Anji_Mito", "Leo_Whitefang", "Faust", "Axl_Low", "Potemkin", "Ramlethal_Valentine", "Giovanna", "Goldlewis_Dickinson"],
    [27121, 25406, 25419, 25425, 25177, 25428, 25429, 25427, 25422, 29465, 32523, 25421, 23572, 25409, 25424, 25423, 25426, 25420, 26496]
);

#[tokio::main]
async fn main() {
   
    dotenv::dotenv().expect("Failed to load .env file");
    
    // Debuging
    //tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);
    
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new().configure(|c| c.owners(owners).prefix("b.")).group(&GENERAL_GROUP);

    // Creating a new bot instance
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("\nError creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    // Starting the bot instance
    if let Err(why) = client.start().await {
        println!("\nClient error: {:?}", why);
    }
}
