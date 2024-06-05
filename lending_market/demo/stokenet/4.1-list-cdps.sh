#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"list_liquidable_cdps\"
    0u64
    10u64
;" >> tx.rtm