use std::sync::LazyLock;

use anyhow::Context;
use clap::Parser;
use reqwest::Client;

const YANDEX_MUSIC_API_URL: &str = "https://api.music.yandex.ru:443";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The token for Yandex Music
    #[arg(short, long, value_name = "TOKEN")]
    token: String,
}

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap()
});

async fn get_user_id(token: &str) -> anyhow::Result<String> {
    CLIENT
        .get(format!("{YANDEX_MUSIC_API_URL}/account/status"))
        .header("x-Yandex-Music-Client", "YandexMusicAndroid/24024312")
        .header("User-Agent", "okhttp/4.12.0")
        // .header("Content-Type", "x-www-form-urlencoded")
        .header("accept", "application/json")
        .header("Authorization", format!("OAuth {token}"))
        .send()
        .await
        .context("Getting user id")?
        .text()
        .await
        .context("Getting user id")
}

async fn get_likes(token: &str, user_id: &str) -> anyhow::Result<String> {
    CLIENT
        .get(format!(
            "{YANDEX_MUSIC_API_URL}/users/{user_id}/likes/tracks"
        ))
        .header("x-Yandex-Music-Client", "YandexMusicAndroid/24024312")
        .header("User-Agent", "okhttp/4.12.0")
        // .header("Content-Type", "x-www-form-urlencoded")
        .header("accept", "application/json")
        .header("Authorization", format!("OAuth {token}"))
        .send()
        .await
        .context("Getting likes")?
        .text()
        .await
        .context("Getting likes")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { token } = Args::parse();

    // let user_id = get_user_id(&token).await?;

    // println!("{user_id}");

    let user_id = String::from("000000000");

    let likes = get_likes(&token, &user_id).await?;

    println!("{likes}");

    Ok(())
}
