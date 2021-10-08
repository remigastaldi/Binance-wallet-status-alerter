use std::time::Duration;

use crate::coin_wallet::CoinWallet;

use telegram_bot::{Api, CanSendMessage, ChatId};

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
    telegram_chat_id: String,
    api_key: String,
    secret_key: String,
    binance_client: Option<WithdrawalClient>,
    telegram_api: Option<Api>,
    debug: bool
}

impl Alerter {
    pub fn new(telegram_bot_token: String, telegram_chat_id: String, api_key: String, secret_key: String) -> Self {
        Alerter{telegram_bot_token, telegram_chat_id, api_key, secret_key, binance_client: None, telegram_api: None, debug: false}
    }
        
    pub fn init_binance_api(&mut self) -> Result<(), tokio_binance::error::Error> {
        self.binance_client = Some(WithdrawalClient::connect(&self.api_key, &self.secret_key, "https://api.binance.com")?);
        Ok(())
    }
    
    pub fn init_telegram_api(&mut self) {
        if !self.debug {
            self.telegram_api = Some(Api::new(&self.telegram_bot_token));
        }
    }

    async fn get_wallet_status(&self, coin_name: &str) -> Result<CoinWallet, String> {
        if let Some(client) = &self.binance_client {
            match client.get_capital_config().with_recv_window(10000).json::<Vec<Value>>().await {
                Ok(res) => {
                    for coin in & res {
                        if coin["coin"] == coin_name {
                            let mut wallet = CoinWallet::new();
                            
                            return match coin["networkList"].as_array() {
                                Some(networks) => {
                                    for network in networks {
                                        match network["network"].as_str() {
                                            Some(network_name) => 
                                            match network["depositEnable"].as_bool() {
                                                Some(deposit_enable) => match network["depositDesc"].as_str() {
                                                    Some(deposit_desc) => match  network["withdrawEnable"].as_bool() {
                                                        Some(withdraw_enable) => match network["withdrawDesc"].as_str() {
                                                            Some(withdraw_desc) => wallet.add_network(network_name,deposit_enable, deposit_desc, withdraw_enable, withdraw_desc),
                                                            None => return Err(String::from("json: \"withdrawDesc\" is null"))
                                                        }, None => return Err(String::from("json: \"withdrawEnable\" is null"))
                                                    }, None => return Err(String::from("json: \"depositDesc\" is null"))
                                                },
                                                None => return Err(String::from("json: \"depositEnable\" is null"))
                                            },
                                            None => return Err(String::from("json: \"network\" is null"))
                                        }
                                    }
                                    Ok(wallet)
                                }
                                None => Err(String::from("json: \"networkList\" is null")),
                            }
                        }
                    }
                    Err(format!("json: {} not found", coin_name))
                },
                Err(err) => Err(err.to_string())
            }
        } else {
            Err(String::from("Binance client not initialized"))
        }
    }

    pub async fn send_telegram_message(&self, chat: &ChatId, msg: &str) -> Result<(), telegram_bot::Error> {
        if let Some(api) = &self.telegram_api {
            if let Err(err) = api.send(chat.text(msg)).await {
                eprintln!("Error sending telegram msg {}", err);
                return Err(err)
            }
        }
        Ok (())
    }

    pub async fn send_telegram_message_timeout(&self, chat: &ChatId, msg: &str, duration: Duration) -> Result<(), telegram_bot::Error> {
        if let Some(api) = &self.telegram_api {
            if let Err(err) = api.send_timeout(chat.text(msg), duration).await {
                eprintln!("Error sending telegram msg {}", err);
                return Err(err)
            }
        }
        Ok (())
    }

    pub async fn run(&mut self, coin: &str, debug: bool) -> Result<(), Box<dyn std::error::Error>> { //TODO: use a proper error type
        self.debug = debug;
        self.init_binance_api()?;
        self.init_telegram_api();

        let chat = ChatId::new(self.telegram_chat_id.parse::<i64>()?);
        let mut save_status;
        
        match self.get_wallet_status(coin).await {
            Ok(res) => save_status = res,
            Err(err) => return Err(format!("Error getting wallet status: {}", err).into())
        }
        
        let mut msg = add_utc_line(&save_status.formatted_networks_status());
        println!("{}", &msg);
        
        self.send_telegram_message(&chat, &msg).await?;
        
        let mut binance_retry: i32 = 0;
        let mut telegram_retry: i32 = 0;
        
        loop {
            println!("{}", add_utc_line("Send request to binance")); // for debug
            match self.get_wallet_status("AVAX").await {
                Ok(asset_status) => {
                    if save_status != asset_status {
                        msg = add_utc_line(&asset_status.formatted_networks_status());
                        println!("{}",msg);
                        if let Err(err) = self.send_telegram_message_timeout(&chat, &msg, Duration::from_secs(8)).await {
                            println!("Error sending telegram msg {}", err);
                            telegram_retry += 1;
                        } else {
                            save_status = asset_status;
                            telegram_retry = 0;
                        }
                    }
                    binance_retry = 0;
                },
                Err(err) => {
                    eprintln!("Error getting wallet status: {}", err);
                    binance_retry += 1;
                }
            }
            if binance_retry == MAX_API_RETRY_BEFORE_DELAY {
                println!("Too much binance api errors, waiting {} sc", API_RETRY_DELAY);
                sleep(Duration::from_secs(API_RETRY_DELAY)).await;
                self.init_binance_api()?;
            }
            if telegram_retry == MAX_API_RETRY_BEFORE_DELAY {
                println!("Too much telegram api errors, waiting {} sc", API_RETRY_DELAY);
                sleep(Duration::from_secs(API_RETRY_DELAY)).await;
                self.init_telegram_api();
            }
            sleep(Duration::from_secs(REFRESH_RATE)).await;
        }
    }
}