use serde::Deserialize;
use twitch_irc::{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    ClientConfig, SecureTCPTransport, TwitchIRCClient,
};
// use mongodb::{Client, options::ClientOptions};
// use mongodb::bson::{Document};

use std::sync::Arc;

const PREFIX: char = '-';
const OAUTH: &str = env!("TWITCH_TOKEN");
const LOGIN_NAME: &str = env!("LOGIN_NAME");
// const MONGO: &str = env!("MONGO");
const HYPIXEL_API_KEY: &str = env!("HYPIXEL_API_KEY");

#[tokio::main]
pub async fn main() {
    let login_name = LOGIN_NAME.to_owned();

    // let client_options = ClientOptions::parse(MONGO.to_owned()).await.unwrap();
    // let client = Client::with_options(client_options).unwrap();

    // let db = client.database("hypixelstatistics");
    // let collection = db.collection::<Document>("channels");

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(
        login_name,
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

    client.join(LOGIN_NAME.to_owned()).unwrap();
    join_handle.await.unwrap();
}

async fn handle_message(
    msg: ServerMessage,
    client: &Arc<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>
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
                uuid, HYPIXEL_API_KEY
            ))
            .await
            .unwrap()
            .json::<HypixelInfo>()
            .await
            .unwrap()
            .player;

            client
                .reply_to_privmsg(
                    format!("the player name of this player is {}!", player.playername).to_owned(),
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
    playername: String,
}
