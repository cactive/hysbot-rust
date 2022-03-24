use crate::{
    hypixel::{get_hypixel_player, rank_prefix},
    utility::apostrophe,
};

use twitch_irc::{
    login::StaticLoginCredentials, message::PrivmsgMessage, SecureTCPTransport, TwitchIRCClient,
};

use std::sync::Arc;

pub async fn process_command(
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

async fn get_username(
    msg: &PrivmsgMessage,
    client: Arc<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>,
    uuid: &str,
) {
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
