#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"list_liquidable_cdps\"
;" >> tx.rtm