#!/bin/bash
cargo build
sudo setcap cap_net_raw,cap_net_admin=eip target/debug/sniffer
./target/debug/sniffer bypassargs -i wlp2s0
