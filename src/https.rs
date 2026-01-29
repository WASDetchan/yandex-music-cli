use std::sync::LazyLock;

use reqwest::header::{HeaderMap, HeaderValue};

pub const YANDEX_MUSIC_API_URL: &str = "https://api.music.yandex.ru:443";

pub static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::ClientBuilder::new()
        .cookie_store(true)
        .build()
        .unwrap()
});

pub fn make_shared_headers(token: &str) -> HeaderMap<HeaderValue> {
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
