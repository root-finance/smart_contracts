#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

# ------------------------------------------------------------------------------------ Create CDP
echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"withdraw\"
    Address(\"$XRD\")
    Decimal(\"9500\");"  >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$XRD\")
    Bucket(\"res_bucket_0\")
;" >> tx.rtm 

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"create_cdp\"
    Enum<0u8>()
    Enum<0u8>()
    Enum<0u8>()
    Array<Bucket>(
        Bucket(\"res_bucket_0\")
    )
;" >> tx.rtm


echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\")
;" >> tx.rtm