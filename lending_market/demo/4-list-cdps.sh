#!/usr/bin/env sh
source ./baseline.sh 
source ./supply.sh
source ./borrow.sh

# ------------------------------------------------------------------------------------ List CDPs
resim set-current-time 2024-03-01T12:07:00Z

resim set-default-account $OWNER_ADDRESS  $OWNER_PVKEY $OWNER_NONFUNGIBLEGLOBALID

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"lock_fee\"
    Decimal(\"100\");" > tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$USDT_RESOURCE_ADDRESS\") Decimal(\"30\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
resim run tx.rtm

resim set-default-account $LIQUIDATOR_ADDRESS  $LIQUIDATOR_PVKEY $LIQUIDATOR_NONFUNGIBLEGLOBALID

resim set-current-time 2024-03-01T13:00:00Z

echo "CALL_METHOD
    Address(\"$LIQUIDATOR_ADDRESS\")
    \"lock_fee\"
    Decimal(\"100\")
;" > tx.rtm

echo "CALL_METHOD
    Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\")
    \"list_liquidable_cdps\"
;" >> tx.rtm

resim run tx.rtm
