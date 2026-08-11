pub fn normalize(value: &str) -> Box<str> {
    value.to_lowercase().replace('-', " ").trim().into()
}

pub fn directory_size(path: impl AsRef<std::path::Path>) -> std::io::Result<u64> {
    let mut total = 0;

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_file() {
            total += entry.metadata()?.len();
        } else if entry.file_type()?.is_dir() {
            total += directory_size(path)?;
        }
    }

    Ok(total)
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    let mut value = bytes as f64;
    let mut unit = 0;

    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}
