#!/bin/sh

cache_dir=.cache
mkdir -p $cache_dir
WD=$(pwd)
cd racecar

echo "Building contract..."
cargo contract build

echo "Uploading contract..."
cargo contract upload --suri //Alice --execute

echo "Instantiate contract..."
contract_address=$(cargo contract instantiate --suri //Alice --args 420 --execute --output-json --skip-confirm | jq .contract)
echo "Contract address: $contract_address"

cd $WD
# Write contract address to cache
echo $contract_address | tr -d '"' > $cache_dir/contract_address