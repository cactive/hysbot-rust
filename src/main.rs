mod commands;
mod hypixel;
mod utility;

use crate::commands::process_command;

use twitch_irc::{
    login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
    TwitchIRCClient,
};

use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, Client, Collection};

use std::{env, sync::Arc};

const OAUTH: &str = env!("TWITCH_TOKEN");
const HYPIXEL_API_KEY: &str = env!("HYPIXEL_API_KEY");
const MONGO: &str = env!("MONGO");

const PREFIX: &str = "-";

#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse(MONGO.to_owned()).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database("hypixelstatistics");
    let collection = db.collection::<Document>("channels");

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
            handle_message(message, &other_client, &collection).await;
        }
    });

    client.join("cakier".to_owned()).unwrap();
    join_handle.await.unwrap();
}

async fn handle_message(
    msg: ServerMessage,
    client: &Arc<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
    collection: &Collection<Document>,
) {
    match msg {
        ServerMessage::Privmsg(msg) => {
            let msg_content = msg.message_text.trim();

            let channel_id = msg.channel_id.to_owned();

            let prefix = get_prefix(&channel_id, collection).await;

            if msg_content.starts_with(&prefix) && msg_content.len() > prefix.len() {
                let message_without_prefix =
                    msg_content[prefix.len()..msg_content.len()].to_owned();

                let message_args: Vec<&str> = message_without_prefix.split_whitespace().collect();
                process_command(msg.clone(), message_args, client.clone()).await;
            }
        }
        _ => {}
    }
}

async fn get_prefix(channel_id: &str, collection: &Collection<Document>) -> String {
    let channel_id = channel_id.to_owned();
    match collection
        .find_one(doc! {"channel_id": channel_id}, None)
        .await
    {
        Ok(response) => match response {
            Some(item) => item.get_str("prefix").unwrap_or(PREFIX).to_owned(),
            None => PREFIX.to_owned(),
        },
        Err(e) => {
            println!("err: {}", e);
            PREFIX.to_owned()
        }
    }
}
