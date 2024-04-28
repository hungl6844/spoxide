use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistItemsObject {
    pub href: Option<String>,
    pub limit: Option<i64>,
    pub next: Option<String>,
    pub offset: Option<i64>,
    pub previous: Option<String>,
    pub total: Option<u16>,
    pub items: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub added_at: Option<String>,
    pub added_by: Option<AddedBy>,
    pub is_local: Option<bool>,
    pub track: Option<Track>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpotifyCredentials {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddedBy {
    #[serde(rename = "external_urls")]
    pub external_urls: Option<ExternalUrls>,
    pub followers: Option<Followers>,
    pub href: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalUrls {
    pub spotify: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Followers {
    pub href: Option<String>,
    pub total: Option<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub album: Option<Album>,
    pub artists: Option<Vec<Artist>>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: Option<u8>,
    pub duration_ms: Option<u32>,
    pub explicit: Option<bool>,
    pub external_ids: Option<ExternalIds>,
    pub external_urls: Option<ExternalUrls>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub is_playable: Option<bool>,
    pub linked_from: Option<LinkedFrom>,
    pub restrictions: Option<Restrictions>,
    pub name: Option<String>,
    pub popularity: Option<u8>,
    pub preview_url: Option<String>,
    pub track_number: Option<u16>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub uri: Option<String>,
    pub is_local: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    pub album_type: Option<String>,
    pub total_tracks: Option<u16>,
    pub available_markets: Option<Vec<String>>,
    pub external_urls: Option<ExternalUrls>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub images: Option<Vec<Image>>,
    pub name: Option<String>,
    pub release_date: Option<String>,
    pub release_date_precision: Option<String>,
    pub restrictions: Option<Restrictions>,
    pub type_field: Option<String>,
    pub uri: Option<String>,
    pub artists: Option<Vec<Artist>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    pub url: Option<String>,
    pub height: Option<u16>,
    pub width: Option<u16>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Restrictions {
    pub reason: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    pub external_urls: Option<ExternalUrls>,
    pub followers: Option<Followers>,
    pub genres: Option<Vec<String>>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub images: Option<Vec<Image>>,
    pub name: Option<String>,
    pub popularity: Option<u8>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalIds {
    pub isrc: Option<String>,
    pub ean: Option<String>,
    pub upc: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkedFrom {
    external_urls: Option<ExternalUrls>,
}

pub async fn gen_token(
    client: &Client,
    client_id: &str,
    client_secret: &str,
) -> Result<String, Box<dyn Error>> {
    Ok(client
        .post("https://accounts.spotify.com/api/token")
        .body(format!(
            "grant_type=client_credentials&client_id={}&client_secret={}",
            client_id, client_secret
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?
        .text()
        .await?)
}

pub async fn get_items(
    client: &Client,
    access_token: &str,
    offset: u16,
    filter: &str,
    playlist_id: &str,
) -> Result<PlaylistItemsObject, Box<dyn Error>> {
    Ok(serde_json::from_str(
        client
            .get(format!(
                "https://api.spotify.com/v1/playlists/{}/tracks?offset={}&filter={}",
                playlist_id, offset, filter
            ))
            .header("Authorization", "Bearer ".to_string() + access_token)
            .send()
            .await?
            .text()
            .await?
            .as_str(),
    )?)
}
