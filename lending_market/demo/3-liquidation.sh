#!/usr/bin/env sh
source ./baseline.sh 
source ./supply.sh
source ./borrow.sh

# ------------------------------------------------------------------------------------ Liquidation
resim set-current-time 2024-03-01T12:07:00Z

resim set-default-account $OWNER_ADDRESS  $OWNER_PVKEY $OWNER_NONFUNGIBLEGLOBALID

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"lock_fee\"
    Decimal(\"100\");" > tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$USDT_RESOURCE_ADDRESS\") Decimal(\"28\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
resim run tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"lock_fee\"
    Decimal(\"100\");" > tx.rtm
echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"create_proof_of_non_fungibles\"
    Address(\"$LENDING_MARKET_ADMIN_ADDRESS\")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId(\"#1#\"),
        NonFungibleLocalId(\"#2#\"),
        NonFungibleLocalId(\"#3#\"),
        NonFungibleLocalId(\"#4#\")
    );" >> tx.rtm
echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"mint_liquidator_badge\";" >> tx.rtm
echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");"  >> tx.rtm

resim run tx.rtm

resim transfer $LENDING_MARKET_LIQUIDATOR_BADGE:1 $LIQUIDATOR_ADDRESS

resim set-default-account $LIQUIDATOR_ADDRESS  $LIQUIDATOR_PVKEY $LIQUIDATOR_NONFUNGIBLEGLOBALID

resim set-current-time 2024-03-01T13:00:00Z

echo "CALL_METHOD
    Address(\"$LIQUIDATOR_ADDRESS\")
    \"lock_fee\"
    Decimal(\"100\");" > tx.rtm

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"check_cdp_for_liquidation\"
    NonFungibleLocalId(\"#1#\");" >> tx.rtm

echo "CALL_METHOD
    Address(\"$LIQUIDATOR_ADDRESS\")
    \"create_proof_of_non_fungibles\"
    Address(\"$LENDING_MARKET_LIQUIDATOR_BADGE\")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId(\"#1#\")
    );" >> tx.rtm

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"start_liquidation\"
    NonFungibleLocalId(\"#1#\")
    Array<Address>(
        Address(\"$XRD\")
    )
    Enum<1u8>(Decimal(\"1200\"));" >> tx.rtm

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$XRD\")
    Bucket(\"xrd_bucket\");" >> tx.rtm 

echo "CALL_METHOD
    Address(\"$FAUCET_COMPONENT_ADDRESS\")
    \"get_resource\"
    Address(\"$USDT_RESOURCE_ADDRESS\")
    Bucket(\"xrd_bucket\"); " >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$USDT_RESOURCE_ADDRESS\")
    Bucket(\"usdt_bucket\");" >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS\")
    Bucket(\"liquidation_term_bucket\");" >> tx.rtm 

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"end_liquidation\"
    Array<Bucket>(
        Bucket(\"usdt_bucket\")
    )
    Bucket(\"liquidation_term_bucket\");" >> tx.rtm

echo "CALL_METHOD
    Address(\"$LIQUIDATOR_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");"  >> tx.rtm

resim run tx.rtm
