pub fn apostrophe(content: Option<String>) -> String {
    if content.is_some() {
        let content = content.unwrap();

        if content.chars().last().unwrap() == 's' {
            return format!("{}’", content);
        }

        return format!("{}’s", content);
    }
    "N/A".to_owned()
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
            "GAME_MASTER" => return "[GM ]",
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
