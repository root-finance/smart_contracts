#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

export CDP_ID="#4#";

# ------------------------------------------------------------------------------------ Borrow
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
    \"borrow\"
    Proof(\"cdp_proof\")
    Array<Tuple>(
        Tuple(
            Address(\"$USDT_RESOURCE_ADDRESS\"),
            Decimal(\"265\")
        )
    )
;" >> tx.rtm

echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\")
;" >> tx.rtm