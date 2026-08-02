pub fn normalize(value: &str) -> String {
    value.to_lowercase().replace("-", " ").trim().to_string()
}
