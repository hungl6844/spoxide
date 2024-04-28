use std::error::Error;
use std::path::Path;

use clap::{arg, Parser};
use futures_util::StreamExt;
use regex::Regex;
use serde_json::{json, Value};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use spotify::SpotifyCredentials;

mod spotify;
mod youtube;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    client_id: String,
    #[arg(long)]
    client_secret: String,
    #[arg(short, long)]
    playlist_id: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let client = reqwest::Client::new();

    let credentials: SpotifyCredentials = serde_json::from_str(
        spotify::gen_token(
            &client,
            args.client_id.as_str(),
            args.client_secret.as_str(),
        )
        .await?
        .as_str(),
    )?;

    let mut i = 0;
    let mut tracks = spotify::get_items(
        &client,
        credentials.access_token.as_str(),
        i,
        "items(track(name,artists(name))),total",
        args.playlist_id.as_str(),
    )
    .await?;

    while i < tracks.total.unwrap() {
        'trackloop: for item in tracks.items {
            if let Some(track) = item.track {
                let search = format!(
                    "{} - {}",
                    track
                        .artists
                        .unwrap()
                        .iter()
                        .map(|a| { a.to_owned().name.unwrap() })
                        .collect::<Vec<String>>()
                        .join(", "),
                    track.name.clone().unwrap()
                );

                let name = format!("{:.36}.m4a", track.name.unwrap());
                let sanitize = Regex::new(r#"[/?<>\\:*|"\x00-\x1f\x80-\x9f]"#)?;
                let sanitized_path = sanitize.replace_all(name.as_str(), "_").to_string();
                let path = Path::new(&sanitized_path);
                if path.exists() {
                    println!("{name} already exists, skipping");
                    i += 1;
                    continue 'trackloop;
                } // skip to next track

                let response = client
                    .post(
                        "https://co.wuk.sh/api/json"
                    )
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .body(json!({
                        "url": format!("https://youtu.be/{}", youtube::search(&client, search.as_str(), "relevance", "video").await?.first().unwrap().video_id.as_str()),
                        "isAudioOnly": "true"
                    }).to_string())
                    .send()
                    .await?;

                let mut file = File::options()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .await?;

                println!("downloading {}", search);
                let pre_time = std::time::SystemTime::now();

                let mut stream = client
                    .get(
                        serde_json::from_str::<Value>(response.text().await?.as_str())?["url"]
                            .as_str()
                            .unwrap(),
                    )
                    .send()
                    .await?
                    .bytes_stream();

                while let Some(chunk) = stream.next().await {
                    file.write_all(chunk?.to_vec().as_slice()).await?;
                }

                println!(
                    "downloaded {} in {}s",
                    path.to_str().unwrap(),
                    std::time::SystemTime::now()
                        .duration_since(pre_time)?
                        .as_secs_f32()
                );

                i += 1;
            }
        }

        if i == tracks.total.unwrap() {
            break;
        }
        tracks = spotify::get_items(
            &client,
            credentials.access_token.as_str(),
            i,
            "items(track(name,artists(name))),total",
            args.playlist_id.as_str(),
        )
        .await?;
    }

    Ok(())
}
