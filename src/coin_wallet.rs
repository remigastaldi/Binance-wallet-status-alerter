use core::fmt;
use std::convert::TryFrom;

#[derive(PartialEq)]
pub struct Network {
    network: String,
    deposit: bool,
    deposit_desc: String,
    withdraw: bool,
    withdraw_desc: String,
}

impl Network {
    pub fn new(network: &str, deposit: bool, deposit_desc: &str, withdraw: bool, withdraw_desc: &str) -> Self {
        Network{ network: network.to_string(), deposit, deposit_desc: deposit_desc.to_string(), withdraw, withdraw_desc: withdraw_desc.to_string() }
    }
}

impl TryFrom<&serde_json::Value> for Network {
    type Error = String;

    fn try_from(item: &serde_json::Value) -> Result<Self, String> {
        Ok(Network { network: item["network"].as_str().ok_or("network field is null")?.to_string(),
                    deposit: item["depositEnable"].as_bool().ok_or("depositEnable field is null")?,
                    deposit_desc: item["depositDesc"].as_str().ok_or("depositDesc field is null")?.to_string(),
                    withdraw: item["withdrawEnable"].as_bool().ok_or("withdrawEnable field is null")?,
                    withdraw_desc: item["withdrawDesc"].as_str().ok_or("withdrawDesc field is null")?.to_string()})
            
    }
}

#[derive(PartialEq)]
pub struct CoinWallet {
    networks: Vec<Network>
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { //TODO: remove unwrap
        writeln!(f, "Network: {}", self.network).unwrap();

        if self.deposit {
            writeln!(f, "Deposit available").unwrap();
        } else {
            writeln!(f, "{}", self.deposit_desc).unwrap();
        }
        if self.withdraw {
            writeln!(f, "Withdrawal available").unwrap();
        } else {
            writeln!(f, "{}", self.withdraw_desc).unwrap();
        }
        Ok(())
    }
}

impl CoinWallet {
    pub fn new() -> Self {
        CoinWallet{networks: Vec::new()}
    }

    pub fn formatted_networks_status(&self) -> String {
        let mut msg = String::new();
        for (i, network) in self.networks.iter().enumerate() {
            msg += &format!("{}{}", if i == 0 {""} else {"\n"}, network);
        }
        msg
    }

    pub fn add_network(&mut self, network: Network) {
        self.networks.push(network);
    }
}

impl From<Vec<Network>> for CoinWallet {
    fn from(item: Vec<Network>) -> Self {
        CoinWallet{ networks: item }
    }
}
