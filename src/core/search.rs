use anyhow::{Result, bail};

pub fn run_search(query: String) -> Result<String> {
    if query.trim().is_empty() {
        bail!("empty query not allowed");
    }

    Ok(format!("Results for {query}"))
}
