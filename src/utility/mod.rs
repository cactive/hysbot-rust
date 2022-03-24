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
