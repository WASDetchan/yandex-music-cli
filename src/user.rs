use anyhow::Context;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::https::{CLIENT, YANDEX_MUSIC_API_URL};

pub async fn get_user_id(headers: HeaderMap<HeaderValue>) -> anyhow::Result<u64> {
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

struct LikesResponse {}

pub async fn get_likes(headers: HeaderMap<HeaderValue>, user_id: u64) -> anyhow::Result<()> {
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

    log::trace!("get_likes response: {response:#?}");

    Ok(())
}
