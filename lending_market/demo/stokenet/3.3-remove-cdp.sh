
#!/usr/bin/env sh
source ./baseline.sh 

echo "" > tx.rtm;

CDP_ID="#4#";

# ------------------------------------------------------------------------------------ rEMOVE cOLLATERAL

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
    \"remove_collateral\"
    Proof(\"cdp_proof\")
    Enum<0u8>()
    Array<Tuple>(
        Tuple(
            ResourceAddress(),
            Decimal(),
            false
        )
    )
;" >> tx.rtm

echo "CALL_METHOD
    Address(\"$BORROWER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\")
;" >> tx.rtm