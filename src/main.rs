use crate::commands::process_command;

use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

pub mod commands;
pub mod hypixel;
pub mod utility;

//pub use self::utility::formatting::apostrophe;
//pub use self::utility::formatting::rank_prefix;
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
