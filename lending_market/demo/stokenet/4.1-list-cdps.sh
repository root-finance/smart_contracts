#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"list_liquidable_cdps\"
;" >> tx.rtm

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$LENDING_LIQUIDATION_PEEK_RESOURCE_ADDRESS\")
    Bucket(\"liqidation_peek_bucket\")
;" >> tx.rtm 

echo "BURN_RESOURCE
    Bucket(\"liqidation_peek_bucket\")
;" >> tx.rtm

