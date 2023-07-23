#!/bin/bash
cargo b --release
ext=$?
if [[ $ext -ne 0 ]]; then
	exit $ext
fi 
sudo setcap cap_net_admin=eip /home/joe/Documents/Programming/tcprust/target/release/tcprust
/home/joe/Documents/Programming/tcprust/target/release/tcprust & 
pid=$!


sudo ip addr add 192.168.0.128/28 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid