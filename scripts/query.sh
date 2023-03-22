#!/bin/sh

cache_dir=.cache
CONTRACT_ADDR=$(cat $cache_dir/contract_address)

cargo contract call --contract $CONTRACT_ADDR --suri //Alice --message get_track --skip-confirm
cargo contract call --contract $CONTRACT_ADDR --suri //Alice --message get_num_racers --skip-confirm
cargo contract call --contract $CONTRACT_ADDR --suri //Alice --message get_curr_lap --skip-confirm
