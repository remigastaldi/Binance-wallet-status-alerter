use core::fmt;

#[derive(PartialEq)]
pub  struct Network {
    network: String,
    deposit: bool,
    deposit_desc: String,
    withdraw: bool,
    withdraw_desc: String,
}

#[derive(PartialEq)]
pub struct CoinWallet {
    networks: Vec<Network>
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { //TODO: remove unwrap
        writeln!(f, "Network: {}", self.network).unwrap();

        match self.deposit {
            true => {
                writeln!(f, "Deposit available").unwrap();
            }
            false => {
                writeln!(f, "{}", self.deposit_desc).unwrap();
            }
        }
        match self.withdraw {
            true => {
                writeln!(f, "Withdrawal available").unwrap();
            }
            false => {
                writeln!(f, "{}", self.withdraw_desc).unwrap();
            }
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
            msg += &format!("{}{}", if i != 0 {"\n"} else {""}, network);
        }
        msg
    }

    pub fn add_network(&mut self, network: &str, deposit: bool, deposit_desc: &str, withdraw: bool, withdraw_desc: &str) {
        self.networks.push(Network{network: network.to_string(), deposit, deposit_desc: deposit_desc.to_string(), withdraw, withdraw_desc: withdraw_desc.to_string()});
    }
}