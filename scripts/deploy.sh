#!/bin/sh

cache_dir=.cache
mkdir -p $cache_dir

echo "Building contract..."
cargo contract build

echo "Uploading contract..."
cargo contract upload --suri //Alice --execute

echo "Instantiate contract..."
contract_address=$(cargo contract instantiate --suri //Alice --args 3 1000 --execute --output-json --skip-confirm | jq .contract)
echo "Contract address: $contract_address"
# Write contract address to cache
echo $contract_address | tr -d '"' > $cache_dir/contract_address

echo "lap: 0x76cb0e8d"
echo "should_run: 0xb4d74178"
echo "should_kill: 0x44e091c7"

# Build racer contract
cargo contract build --manifest-path racecar/Cargo.toml
racer_hash=$(cargo contract upload --manifest-path racecar/Cargo.toml --suri //Alice --execute | grep -i "Code hash" | awk '{print $3}') 
echo $racer_hash | tr -d '"' > $cache_dir/racer_hash
TRIMMER_RACER_HASH=$(cat $cache_dir/racer_hash)

# # Deploy racer 1
# echo "Deploying racer 1..."
# racer1_address=$(cargo contract instantiate --manifest-path racecar/Cargo.toml --suri //Alice --args 1 --execute --output-json --skip-confirm | jq .contract)
# echo "Racer 1 address: $racer1_address"
# # Write racer 1 address to cache
# echo $racer1_address | tr -d '"' > $cache_dir/racer1_address

# # Deploy racer 2
# echo "Deploying racer 2..."
# racer2_address=$(cargo contract instantiate --manifest-path racecar/Cargo.toml --suri //Alice --args 2 --execute --output-json --skip-confirm | jq .contract)
# echo "Racer 2 address: $racer2_address"
# # Write racer 2 address to cache
# echo $racer2_address | tr -d '"' > $cache_dir/racer2_address

# # Deploy racer 1
# echo "Deploying racer 3..."
# racer3_address=$(cargo contract instantiate --manifest-path racecar/Cargo.toml --suri //Alice --args 3 --execute --output-json --skip-confirm | jq .contract)
# echo "Racer 3 address: $racer3_address"
# # Write racer 3 address to cache
# echo $racer3_address | tr -d '"' > $cache_dir/racer3_address

# Register racers
echo "Registering racers..."
main_contract_address=$(cat $cache_dir/contract_address)

# Racer 1
echo "Registering racer 1..."
cargo contract call --suri //Alice --contract $main_contract_address --message register_racer --args $TRIMMER_RACER_HASH --execute --skip-confirm

# Racer 2
echo "Registering racer 2..."
cargo contract call --suri //Alice --contract $main_contract_address --message register_racer --args $TRIMMER_RACER_HASH --execute --skip-confirm

# Racer 3
echo "Registering racer 3..."
cargo contract call --suri //Alice --contract $main_contract_address --message register_racer --args $TRIMMER_RACER_HASH --execute --skip-confirm

echo "Querying number of registered racers..."
cargo contract call --contract $main_contract_address --suri //Alice --message get_num_racers --output-json | jq .data.Tuple.values