#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

LP_UNITS_RESOURCE_ADDRESS=resource_tdx_2_1t5uvmkv7ygqrfhxf8a4ca6vmpgwn72zz3p7asdmup46kdxp83k8pw0

echo "CALL_METHOD
    Address(\"$LP_PROVIDER_ADDRESS\")
    \"withdraw\"
    Address(\"$LP_UNITS_RESOURCE_ADDRESS\")
    Decimal(\"200\");" >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$LP_UNITS_RESOURCE_ADDRESS\")
    Bucket(\"rusdt_bucket\");" >> tx.rtm 
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"redeem\"  Bucket(\"rusdt_bucket\");" >> tx.rtm
echo "CALL_METHOD Address(\"$LP_PROVIDER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
