use serde::Deserialize;
use twitch_irc::{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

use dotenv;

pub mod utility;

pub use self::utility::formatting::apostrophe;
pub use self::utility::formatting::rank_prefix;
// use mongodb::{Client, options::ClientOptions};
// use mongodb::bson::{Document};

use std::sync::Arc;

const PREFIX: char = '-';

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();

    let oauth = dotenv::var("TWITCH_TOKEN").unwrap();
    let login_name = dotenv::var("LOGIN_NAME").unwrap().to_owned();
    // let mongo = dotenv::var("mongo").unwrap();
    let hypixel_api_key = dotenv::var("HYPIXEL_API_KEY").unwrap();

    // let client_options = ClientOptions::parse(mongo.to_owned()).await.unwrap();
    // let client = Client::with_options(client_options).unwrap();

    // let db = client.database("hypixelstatistics");
    // let collection = db.collection::<Document>("channels");

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        login_name.clone(),
        Some(oauth.to_owned()),
    ));

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let client = Arc::new(client);

    let other_client = client.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            handle_message(message, &other_client, hypixel_api_key.clone()).await;
        }
    });

    client.join(login_name.to_owned()).unwrap();
    join_handle.await.unwrap();
}

async fn handle_message(
    msg: ServerMessage,
    client: &Arc<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
    hypixel_api_key: String,
) {
    match msg {
        ServerMessage::Privmsg(msg) => {
            let msg_content = msg.message_text.trim();

            if msg_content.starts_with(PREFIX) && msg_content.len() > PREFIX.len_utf8() {
                let message_without_prefix =
                    msg_content[PREFIX.len_utf8()..msg_content.len()].to_owned();

                let message_args: Vec<&str> = message_without_prefix.split_whitespace().collect();
                process_command(
                    msg.clone(),
                    message_args,
                    client.clone(),
                    hypixel_api_key.clone(),
                )
                .await;
            }
        }
        _ => {}
    }
}

async fn process_command(
    msg: PrivmsgMessage,
    args: Vec<&str>,
    client: Arc<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
    hypixel_api_key: String
) {
    match args[0] {
        "get_username" | "gu" => {
            if args.len() < 2 {
                println!("Not enough parameters passed to get_username!");
                return;
            }
            let uuid = args[1];
            let player = reqwest::get(format!(
                "https://api.hypixel.net/player?uuid={}&key={}",
                uuid, hypixel_api_key
            ))
            .await
            .unwrap()
            .json::<HypixelInfo>()
            .await
            .unwrap()
            .player;

            client
                .reply_to_privmsg(
                    format!("you looked up {}{} statistics!", rank_prefix(player.rank, player.monthlyPackageRank, player.newPackageRank), apostrophe(player.displayname)).to_owned(),
                    &msg,
                )
                .await
                .unwrap();
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

#[derive(Deserialize)]
struct HypixelInfo {
    player: Player,
}

#[derive(Deserialize)]
struct Player {
    displayname: String,
    rank: String,
    monthlyPackageRank: String,
    newPackageRank: String
}
