#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

echo "CALL_METHOD
    Address(\"$LP_PROVIDER_ADDRESS\")
    \"withdraw\"
    Address(\"$XRD\")
    Decimal(\"10000\");" >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$XRD\")
    Bucket(\"xrd_bucket\");" >> tx.rtm 

echo "CALL_METHOD
    Address(\"$FAUCET_COMPONENT_ADDRESS\")
    \"get_resource\"
    Address(\"$USDT_RESOURCE_ADDRESS\")
    Bucket(\"xrd_bucket\"); " >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP Address(\"$USDT_RESOURCE_ADDRESS\") Bucket(\"usdt_bucket\");" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"contribute\"  Bucket(\"usdt_bucket\");" >> tx.rtm
echo "CALL_METHOD Address(\"$LP_PROVIDER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm