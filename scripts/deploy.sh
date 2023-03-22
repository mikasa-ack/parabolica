#!/bin/sh

cache_dir=.cache
mkdir -p $cache_dir

echo "Building contract..."
cargo contract build

echo "Uploading contract..."
cargo contract upload --suri //Alice --execute

echo "Instantiate contract..."
contract_address=$(cargo contract instantiate --suri //Alice --args 3 10 --execute --output-json --skip-confirm | jq .contract)
echo "Contract address: $contract_address"
# Write contract address to cache
echo $contract_address | tr -d '"' > $cache_dir/contract_address

echo "lap: 0x76cb0e8d"
echo "should_run: 0xb4d74178"
echo "should_kill: 0x44e091c7"
