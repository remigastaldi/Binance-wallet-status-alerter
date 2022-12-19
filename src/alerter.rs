use std::{time::Duration, convert::TryFrom};

use crate::coin_wallet::{CoinWallet, Network};

use teloxide::{requests::{Request, Requester}, Bot, types::{Recipient, ChatId}};

use tracing::{info, debug, error};
use tokio::time::sleep;
use tokio_binance::WithdrawalClient;
use serde_json::Value;

use chrono::Utc;

const REFRESH_RATE: u64 = 60; // Refresh rate between requests; in seconds
const MAX_API_RETRY_BEFORE_DELAY: i32 = 5; // After too much api errors the program will wait then try to connect again, it's useful in case of an api maintenance per example
const API_RETRY_DELAY: u64 = 3600; // Waiting time before trying to connect again, in seconds

fn add_utc_line(msg: &str) -> String {
    let utc = Utc::now().naive_utc().to_string();
    format!("{}\n{} UTC", msg, utc.split('.').collect::<Vec<&str>>()[0])
}

pub struct Alerter {
    telegram_bot_token: String,
    telegram_chat_id: i64,
    api_key: String,
    secret_key: String,
    binance_client: Option<WithdrawalClient>,
    telegram_api: Option<Bot>,
    debug: bool
}

impl Alerter {
    pub fn new(telegram_bot_token: String, telegram_chat_id: i64, api_key: String, secret_key: String) -> Self {
        Alerter{telegram_bot_token, telegram_chat_id, api_key, secret_key, binance_client: None, telegram_api: None, debug: false}
    }

    fn init_binance_api(&mut self) -> Result<(), tokio_binance::error::Error> {
        self.binance_client = Some(WithdrawalClient::connect(&self.api_key, &self.secret_key, "https://api.binance.com")?);
        Ok(())
    }

    fn init_telegram_api(&mut self) {
        if !self.debug {
            self.telegram_api = Some(teloxide::Bot::new(&self.telegram_bot_token));
        }
    }

    async fn get_wallet_status(&self, coin_name: &str) -> Result<CoinWallet, String> {
        if let Some(client) = &self.binance_client {
            match client.get_capital_config().with_recv_window(10000).json::<Vec<Value>>().await {
                Ok(res) => Ok(res.iter()
                              .find(|item| item["coin"] == coin_name)
                              .and_then(|coin| coin.get("networkList")).ok_or("networkList is null")?
                              .as_array().ok_or("error converting networkList to an array")?
                              .iter()
                              .filter_map(|network| Network::try_from(network).map_err(|err| error!("{err}")).ok())
                              .collect::<Vec<Network>>()
                              .into()),
                Err(err) => Err(err.to_string())
            }
        } else {
            Err(String::from("Binance client not initialized"))
        }
    }
    
    async fn send_telegram_message(&self, msg: &str) -> Result<(), teloxide::RequestError> {
        if let Some(api) = &self.telegram_api {
            api.send_message(Recipient::Id(ChatId(self.telegram_chat_id)), msg).send().await?;
        }
      Ok (())
    }
    
    pub async fn run(&mut self, coin: &str, init: bool, debug: bool) -> Result<(), Box<dyn std::error::Error>> { //TODO: use a proper error type
        self.debug = debug;
        self.init_binance_api()?;
        self.init_telegram_api();
        
        let mut save_status;
        
        match self.get_wallet_status(coin).await {
            Ok(res) => save_status = res,
            Err(err) => return Err(format!("Error getting wallet status: {err}").into())
        }
        
        let mut msg = add_utc_line(&save_status.formatted_networks_status());
        info!("{}", &msg);
        
        if init {
            self.send_telegram_message(&msg).await?;
        }
        
        let mut binance_retry: i32 = 0;
        let mut telegram_retry: i32 = 0;
        
        loop {
            info!("{}", add_utc_line("Send request to binance")); // for debug
            match self.get_wallet_status("AVAX").await {
                Ok(asset_status) => {
                    if save_status != asset_status {
                        msg = add_utc_line(&asset_status.formatted_networks_status());
                        info!("{}",msg);
                        if let Err(err) = self.send_telegram_message(&msg).await {
                            info!("Error sending telegram msg {}", err);
                            telegram_retry += 1;
                        } else {
                            save_status = asset_status;
                            telegram_retry = 0;
                        }
                    }
                    binance_retry = 0;
                },
                Err(err) => {
                    error!("Error getting wallet status: {}", err);
                    binance_retry += 1;
                }
            }
            if binance_retry == MAX_API_RETRY_BEFORE_DELAY {
                info!("Too much binance api errors, waiting {} sc", API_RETRY_DELAY);
                sleep(Duration::from_secs(API_RETRY_DELAY)).await;
                self.init_binance_api()?;
            }
            if telegram_retry == MAX_API_RETRY_BEFORE_DELAY {
                info!("Too much telegram api errors, waiting {} sc", API_RETRY_DELAY);
                sleep(Duration::from_secs(API_RETRY_DELAY)).await;
                self.init_telegram_api();
            }
            sleep(Duration::from_secs(REFRESH_RATE)).await;
        }
    }
}
