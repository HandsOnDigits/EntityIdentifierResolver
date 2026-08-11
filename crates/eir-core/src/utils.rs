pub fn normalize(value: &str) -> Box<str> {
    value.to_lowercase().replace('-', " ").trim().into()
}

pub fn directory_size(path: impl AsRef<std::path::Path>) -> std::io::Result<u64> {
    let mut total = 0;

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            total += entry.metadata()?.len();
        }
    }

    Ok(total)
}
