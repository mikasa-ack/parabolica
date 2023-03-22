#!/bin/sh

echo "Building contract..."
cargo contract build

echo "Uploading contract..."
cargo contract upload --suri //Alice --execute

echo "Instantiate contract..."
contract_address=$(cargo contract instantiate --suri //Alice --args 3 10 --execute --output-json --skip-confirm | jq .contract)
echo "Contract address: $contract_address"
