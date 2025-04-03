#!/bin/bash
cargo build
sudo setcap cap_net_raw,cap_net_admin=eip target/debug/sniffer
# wlp2s0 enp6s0
./target/debug/sniffer bypassargs -i enp6s0 -v
