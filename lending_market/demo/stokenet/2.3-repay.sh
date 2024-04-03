#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

export CDP_ID="#4#";

# ------------------------------------------------------------------------------------ Repay

echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"withdraw\"
    Address(\"$USDT_RESOURCE_ADDRESS\")
    Decimal(\"100\");"  >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$USDT_RESOURCE_ADDRESS\")
    Bucket(\"res_bucket_0\")
;" >> tx.rtm 

echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"create_proof_of_non_fungibles\"
    Address(\"$LENDING_MARKET_CDP_RESOURCE_ADDRESS\")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId(\"$CDP_ID\")
    )
;" >> tx.rtm

echo "POP_FROM_AUTH_ZONE
    Proof(\"cdp_proof\")
;" >>  tx.rtm

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"repay\"
    Proof(\"cdp_proof\")
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