use crate::HYPIXEL_API_KEY;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HypixelInfo {
    pub success: bool,
    pub cause: Option<String>,
    pub player: Option<Player>,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Player {
    pub displayname: Option<String>,
    pub rank: Option<String>,
    pub monthlyPackageRank: Option<String>,
    pub newPackageRank: Option<String>,
}

pub async fn get_hypixel_player(uuid: &str) -> Result<Player, String> {
    let request = match reqwest::get(format!(
        "https://api.hypixel.net/player?uuid={}&key={}",
        uuid, HYPIXEL_API_KEY
    ))
    .await
    {
        // checks if server request succeeds
        Ok(req) => req,
        Err(e) => return Err(format!("Errored with: {}", e)),
    };

    let hypixel_info = match request.json::<HypixelInfo>().await {
        // checks if json parsing failed
        Ok(json) => json,
        Err(e) => return Err(format!("Errored with: {}", e)),
    };

    if !hypixel_info.success {
        return Err(format!(
            "Hypixel json fetching failed: {}!",
            hypixel_info.cause.unwrap()
        )
        .to_owned());
    }

    match hypixel_info.player {
        // checks if player is null
        Some(player) => Ok(player),
        None => Err(format!("Player uuid not foud:n {}!", uuid)),
    }
}

pub fn rank_prefix(
    rank: Option<String>,
    monthly_package_rank: Option<String>,
    new_package_rank: Option<String>,
) -> &'static str {
    if rank.is_some() {
        match &rank.unwrap()[..] {
            "YOUTUBER" => return "[YOUTUBE] ",
            "MODERATOR" => return "[MOD] ",
            "HELPER" => return "HELPER] ",
            "ADMIN" => return "[ADMIN] ",
            "GAME_MASTER" => return "[GM] ",
            _ => {}
        }
    }
    if monthly_package_rank.is_some() {
        match &monthly_package_rank.unwrap()[..] {
            "SUPERSTAR" => return "[MVP++] ",
            _ => {}
        }
    }
    if new_package_rank.is_some() {
        match &new_package_rank.unwrap()[..] {
            "MVP_PLUS" => return "[MVP+] ",
            "MVP" => return "[MVP] ",
            "VIP_PLUS" => return "[VIP+] ",
            "VIP" => return "[VIP] ",
            _ => {}
        }
    }

    ""
}
