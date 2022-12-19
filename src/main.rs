mod alerter;
mod coin_wallet;

use clap::{Arg, App};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Binance Wallet Status Alerter")
        .version("1.0")
        .author("Rémi G. <remi.gataldi@protonmail.com>")
        .about("Alert when the wallet networks status change on Binance")
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .help("Print additional logs and disable telegram messages"))
        .arg(Arg::new("init")
            .short('i')
            .long("init")
            .help("Start the bot by sending a message with the actual status of the network"))
        .arg(Arg::new("")
            .value_name("TOKEN NAME")
            .required(true)
            .takes_value(true)
            .help("Token name to monitor"))
        .get_matches();

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting");

    let debug = matches.is_present("debug");
    if debug {
        info!("Running in debug mod - telegram messages disabled");
    }

    let telegram_bot_token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let telegram_chat_id = env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set");
    let api_key = env::var("BINANCE_API_KEY").expect("BINANCE_API_KEY not set");
    let secret_key = env::var("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY not set");
   
    alerter::Alerter::new(telegram_bot_token, telegram_chat_id.parse::<i64>()?, api_key, secret_key)?
        .run(matches.value_of("").unwrap(), matches.is_present("init"), debug).await?;

    Ok(())
}
