pub fn normalize(value: &str) -> Box<str> {
    value.to_lowercase().replace('-', " ").trim().into()
}
