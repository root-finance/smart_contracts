#!/usr/bin/env sh

resim set-default-account $BORROWER_ADDRESS  $BORROWER_PVKEY $BORROWER_NONFUNGIBLEGLOBALID

out=`resim call-method $FAUCET free`
echo $out
out=`resim call-method $FAUCET free`
echo $out
out=`resim call-method $FAUCET free`
echo $out

# ------------------------------------------------------------------------------------ Create CDP
echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"lock_fee\"
    Decimal(\"100\");" > tx.rtm

echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"withdraw\"
    Address(\"$XRD\")
    Decimal(\"3000\");"  >> tx.rtm 

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

resim run tx.rtm

# ------------------------------------------------------------------------------------ Borrow

resim set-current-time 2024-03-01T12:05:00Z

echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"lock_fee\"
    Decimal(\"100\");" > tx.rtm

echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"create_proof_of_non_fungibles\"
    Address(\"$LENDING_MARKET_CDP_RESOURCE_ADDRESS\")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId(\"#1#\")
    )
;" >> tx.rtm

echo "POP_FROM_AUTH_ZONE
    Proof(\"cdp_proof\")
;" >>  tx.rtm

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"borrow\"
    Proof(\"cdp_proof\")
    Array<Tuple>(
        Tuple(
            Address(\"$USDT_RESOURCE_ADDRESS\"),
            Decimal(\"76\")
        )
    )
;" >> tx.rtm

echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\")
;" >> tx.rtm

resim run tx.rtm
