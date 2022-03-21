pub fn apostrophe(content: String) -> String {
    if content.chars().last().unwrap() == 's' {
        return format!("{}’", content)
    }

    format!("{}’s", content)
}

pub fn rank_prefix(rank: String, monthly_package_rank: String, new_package_rank: String) -> String {
    if rank == "YOUTUBER" {
        return "[YOUTUBE] ".to_owned()
    } if rank == "MODERATOR" {
        return "[MOD] ".to_owned()
    } if rank == "HELPER" {
        return "[HELPER] ".to_owned()
    } if rank == "ADMIN" {
        return "[ADMIN] ".to_owned()
    } if rank == "GAME_MASTER" {
        return "[GM] ".to_owned()
    } if monthly_package_rank == "SUPERSTAR" {
        return "[MVP++] ".to_owned()
    } if new_package_rank == "MVP_PLUS" {
        return "[MVP+] ".to_owned()
    } if new_package_rank == "MVP" {
        return "[MVP] ".to_owned()
    } if new_package_rank == "VIP_PLUS" {
        return "[VIP+] ".to_owned()
    } if new_package_rank == "VIP" {
        return "[VIP] ".to_owned()
    }

    "".to_owned()
}