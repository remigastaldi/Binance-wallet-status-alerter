# Binance-wallet-status-alerter

Send an alert to a telegram channel every time a network deposit/withdrawal status change for the specified coin.

## How to use

1. Download Rust
2. Create a new telegram bot using [@Botfather](https://t.me/botfather) to get a token and a chatId
3. You must set up these different environment variables:

    ```bash
    export TELEGRAM_BOT_TOKEN=<Your token here>
    export TELEGRAM_CHAT_ID=<Your chat id here>
    export BINANCE_API_KEY=<Your token here>
    export BINANCE_SECRET_KEY=<Your token here>
    ```

4. Build and run the program:

    ```bash
    cargo build --release
    ./target/debug/bn-wallet-status-alerter <COIN_TICKET>
    ```

## Usage example

```bash
./target/debug/bn-wallet-status-alerter AVAX
```

Use of --debug or -d will disable telegram messages

## SystemD service

To be sure the bot won't go offline after a reboot or an error, you can use the systemD service, the path to the binary MUST be absolute.  

1. Install the service

    ```bash
    sudo ./install_service.sh <ABSOLUTE_PATH_TO_BN-WALLET-STATUS-ALERTER-BINARY> <COIN_TICKET>
    ```

2. Set the env variables `TELEGRAM_BOT_TOKEN`, `TELEGRAM_CHAT_ID`, `BINANCE_API_KEY` and `BINANCE_SECRET_KEY` to this file: `/etc/binance-wallet-status-alerter/binance-wallet-status-alerter.conf`

3. Run and enable the service

    ```bash
    sudo systemctl start binance-wallet-status-alerter.service
    sudo systemctl enable binance-wallet-status-alerter.service
    ```
