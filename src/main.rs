mod https;
mod user;
use user::*;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The token for Yandex Music
    #[arg(short, long, value_name = "TOKEN")]
    token: String,
    #[arg(short, long, value_name = "UID")]
    user_id: Option<u64>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let Args { token, user_id } = Args::parse();

    let headers = https::make_shared_headers(&token);

    let user_id = if let Some(uid) = user_id {
        uid
    } else {
        get_user_id(headers.clone()).await?
    };

    println!("{user_id}");

    get_likes(headers, user_id).await?;

    Ok(())
}
