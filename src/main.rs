use serde::Deserialize;
use twitch_irc::{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

pub mod utility;

pub use self::utility::formatting::apostrophe;
pub use self::utility::formatting::rank_prefix;
// use mongodb::{Client, options::ClientOptions};
// use mongodb::bson::{Document};

use std::env;

const OAUTH: &str = env!("TWITCH_TOKEN");
const HYPIXEL_API_KEY: &str = env!("HYPIXEL_API_KEY");
// const MONGO: &str = env!("MONGO");

use std::sync::Arc;

const PREFIX: char = '-';

#[tokio::main]
pub async fn main() {
    // let client_options = ClientOptions::parse(MONGO.to_owned()).await.unwrap();
    // let client = Client::with_options(client_options).unwrap();

    // let db = client.database("hypixelstatistics");
    // let collection = db.collection::<Document>("channels");

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        "cakier".to_owned(),
        Some(OAUTH.to_owned()),
    ));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let client = Arc::new(client);

    let other_client = client.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            handle_message(message, &other_client).await;
        }
    });

    client.join("cakier".to_owned()).unwrap();
    join_handle.await.unwrap();
}

async fn handle_message(
    msg: ServerMessage,
    client: &Arc<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
) {
    match msg {
        ServerMessage::Privmsg(msg) => {
            let msg_content = msg.message_text.trim();

            if msg_content.starts_with(PREFIX) && msg_content.len() > PREFIX.len_utf8() {
                let message_without_prefix =
                    msg_content[PREFIX.len_utf8()..msg_content.len()].to_owned();

                let message_args: Vec<&str> = message_without_prefix.split_whitespace().collect();
                process_command(msg.clone(), message_args, client.clone()).await;
            }
        }
        _ => {}
    }
}

async fn process_command(
    msg: PrivmsgMessage,
    args: Vec<&str>,
    client: Arc<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
) {
    match args[0] {
        "get_username" | "gu" => {
            if args.len() < 2 {
                println!("Player uuid not given!");
                return;
            }
            let uuid = args[1];
            get_username(&msg, client, uuid).await;
        }
        "ping" => {
            client
                .reply_to_privmsg("pong".to_owned(), &msg)
                .await
                .unwrap();
        }
        _ => {}
    }
}

async fn get_username(msg: &PrivmsgMessage, client: Arc<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>, uuid: &str) {
    let player = get_hypixel_player(uuid).await;

    match player {
        Ok(player) => {
            client
                .reply_to_privmsg(
                    format!(
                        "you looked up {}{} statistics!",
                        rank_prefix(
                            player.rank,
                            player.monthlyPackageRank,
                            player.newPackageRank
                        ),
                        apostrophe(player.displayname)
                    )
                    .to_owned(),
                    &msg,
                )
                .await
                .unwrap();
        }
        Err(err) => {
            client
                .reply_to_privmsg(format!("The error was {}", err).to_owned(), &msg)
                .await
                .unwrap();
        }
    }
}

#[derive(Deserialize)]
struct HypixelInfo {
    success: bool,
    cause: Option<String>,
    player: Option<Player>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Player {
    displayname: Option<String>,
    rank: Option<String>,
    monthlyPackageRank: Option<String>,
    newPackageRank: Option<String>,
}

// async fn get_player_uuid

async fn get_hypixel_player(uuid: &str) -> Result<Player, String> {
    let request = match reqwest::get(format!(
        "https://api.hypixel.net/player?uuid={}&key={}",
        uuid, HYPIXEL_API_KEY
    ))
    .await
    { // checks if server request succeeds
        Ok(req) => req,
        Err(e) => return Err(format!("Errored with: {}", e)),
    };

    let hypixel_info = match request.json::<HypixelInfo>().await { // checks if json parsing failed
        Ok(json) => json,
        Err(e) => return Err(format!("Errored with: {}", e)),
    };

    if !hypixel_info.success {
        return Err(format!("Hypixel json fetching failed: {}!", hypixel_info.cause.unwrap()).to_owned());
    }

    match hypixel_info.player { // checks if player is null
        Some(player) => Ok(player),
        None => Err(format!("Player uuid not foud:n {}!", uuid)),
    }
}
