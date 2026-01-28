use std::sync::LazyLock;

use anyhow::Context;
use clap::Parser;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};

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

fn make_shared_headers(token: &str) -> HeaderMap<HeaderValue> {
    let mut map = HeaderMap::new();
    map.insert(
        "x-Yandex-Music-Client",
        HeaderValue::from_static("YandexMusicAndroid/24024312"),
    );
    map.insert("User-Agent", HeaderValue::from_static("okhttp/4.12.0"));
    map.insert(
        "Authorization",
        HeaderValue::from_str(format!("OAuth {token}").as_str()).unwrap(),
    );
    map.insert("accept", HeaderValue::from_static("application/json"));

    map
}

// struct AccountStatus {
//     result
// }

async fn get_user_id(headers: HeaderMap<HeaderValue>) -> anyhow::Result<u64> {
    let response: serde_json::Value = CLIENT
        .get(format!("{YANDEX_MUSIC_API_URL}/account/status"))
        .headers(headers)
        .send()
        .await
        .context("Getting user id")?
        .json()
        .await
        .context("Getting user id. Possibly got asked to do a CAPTHCA?")?;

    log::trace!("get_user_id response: {response:?}");
    let user_id: Option<u64> = (|| {
        Some(
            response
                .get("result")?
                .get("account")?
                .get("uid")?
                .as_number()?
                .as_u64()?
                .to_owned(),
        )
    })();
    user_id.ok_or(anyhow::anyhow!("The response did not contain the user id"))
}

async fn get_likes(headers: HeaderMap<HeaderValue>, user_id: u64) -> anyhow::Result<()> {
    let response: serde_json::Value = CLIENT
        .get(format!(
            "{YANDEX_MUSIC_API_URL}/users/{user_id}/likes/tracks"
        ))
        .headers(headers)
        .send()
        .await
        .context("Getting likes")?
        .json()
        .await
        .context("Getting likes")?;

    log::trace!("get_likes response: {response:?}");

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let Args { token } = Args::parse();

    let headers = make_shared_headers(&token);

    let user_id = get_user_id(headers.clone()).await?;

    println!("{user_id}");

    get_likes(headers, user_id).await?;
    //
    // let user_id = String::from("000000000");
    //
    // let likes = get_likes(&token, &user_id).await?;
    //
    // println!("{likes}");

    Ok(())
}
