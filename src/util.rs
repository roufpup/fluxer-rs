pub fn get_emoji(emoji: &str) -> String {
    if emoji.starts_with(":") && emoji.ends_with(":") {
        let unicode = emojis::get_by_shortcode(emoji.trim_matches(':')).unwrap();
        return unicode.as_str().into();
    }

    emoji.into()
}
