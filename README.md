# Binance-wallet-status-alerter

Send an alert to a telegram channel everytime a network deposit/withdrawal status change for the specified coin.

## How to use
1. Download Rust
2. Create a new telegram bot using [@Botfather](https://t.me/botfather) to get a token and a chatId
3. You must set up those different environment variables:
```bash
export TELEGRAM_BOT_TOKEN=<Your token here>
export TELEGRAM_CHAT_ID=<Your chat id here>
export BINANCE_API_KEY=<Your token here>
export BINANCE_SECRET_KEY=<Your token here>
```
4. Build and run the program:
```bash
cargo build --release
./target/debug/bn-wallet-status-alerter <Coin ticket>
```

# Usage exemple
```bash
./target/debug/bn-wallet-status-alerter AVAX
```
Use of --debug or -d will disable telegram messages
