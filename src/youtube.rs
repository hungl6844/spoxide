use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub type YoutubeSearchResults = Vec<SearchResult>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub title: String,
    pub video_id: String,
}

pub async fn search(
    client: &Client,
    query: &str,
    sort_by: &str,
    res_type: &str,
) -> Result<YoutubeSearchResults, Box<dyn Error>> {
    Ok(serde_json::from_str(
        client
            .get(format!(
                "https://iv.ggtyler.dev/api/v1/search?q={query}&sort_by={sort_by}&type={res_type}"
            ))
            .send()
            .await?
            .text()
            .await?
            .as_str(),
    )?)
}
