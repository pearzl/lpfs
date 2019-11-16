use crate::Result;

pub fn ip_tables_names() -> Result<String> {
    let content = std::fs::read_to_string("/proc/net/ip_tables_names")?;
    Ok(content
        .trim()
        .split_ascii_whitespace()
        .map(String::from)
        .collect())
}
