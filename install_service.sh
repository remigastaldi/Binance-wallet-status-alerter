#!/bin/bash

# $1 - path to executable

mkdir -p /etc/binance-wallet-status-alerter
touch /etc/binance-wallet-status-alerter/binance-wallet-status-alerter.conf

echo "[Unit]
Description=Binance wallet status alerter service
Requires=network-online.target
After=network-online.target

[Service]
EnvironmentFile=/etc/binance-wallet-status-alerter/binance-wallet-status-alerter.conf
ExecStart=$1 $2
RestartSec=60s
Restart=always
TimeoutStopSec=10s

[Install]
WantedBy=multi-user.target" > /etc/systemd/system/binance-wallet-status-alerter.service

exit 0