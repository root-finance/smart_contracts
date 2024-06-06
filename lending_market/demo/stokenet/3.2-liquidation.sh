
#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

CDP_ID="#1#";

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"check_cdp_for_liquidation\"
    NonFungibleLocalId(\"$CDP_ID\");" >> tx.rtm

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"start_liquidation\"
    NonFungibleLocalId(\"$CDP_ID\")
    Array<Address>(
        Address(\"$XRD\")
    )
    Enum<0u8>();" >> tx.rtm

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
    Bucket(\"udst_bucket\");" >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS\")
    Bucket(\"liquidation_term_bucket\");" >> tx.rtm 

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"end_liquidation\"
    Array<Bucket>(
        Bucket(\"udst_bucket\")
    )
    Bucket(\"liquidation_term_bucket\");" >> tx.rtm

echo "CALL_METHOD
    Address(\"$LIQUIDATOR_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");"  >> tx.rtm