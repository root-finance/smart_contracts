#!/usr/bin/env sh

# ------------------------------------------------------------------------------------ Reinitialize
set -x
set -e

resim reset

resim set-current-time 2024-03-01T12:00:00Z

XRD=resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3
FAUCET=component_sim1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxhkrefh
SUPRA_PROOF=CAFEBABE

# ------------------------------------------------------------------------------------ Create admin account
echo "Admin account"
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:|NonFungibleGlobalId:/ {print $NF}'`
echo $out
OWNER_ADDRESS=`echo $out | cut -d " " -f1`
OWNER_PUBKEY=`echo $out | cut -d " " -f2`
OWNER_PVKEY=`echo $out | cut -d " " -f3`
OWNER_NONFUNGIBLEGLOBALID=`resim new-simple-badge --name 'OwnerBadge' | awk '/NonFungibleGlobalId:/ {print $NF}'`

# ------------------------------------------------------------------------------------ Instantiate packages
MOCK_SUPRA_ORACLE_PACKAGE=`resim publish ../../mocks/supra_oracle | tee /dev/tty | awk '/Package:/ {print $NF}'`
out=`resim call-function $MOCK_SUPRA_ORACLE_PACKAGE MockSupraOracle instantiate | tee /dev/tty | awk '/Component:|Resource:/ {print $NF}'`
MOCK_SUPRA_ORACLE_ADDRESS=`echo $out | cut -d " " -f1`

POOL_PACKAGE=`resim publish ../../single_resource_pool/ | tee /dev/tty | awk '/Package:/ {print $NF}'`

PRICE_FEED_PACKAGE=`resim publish ../../internal_price_feed | tee /dev/tty | awk '/Package:/ {print $NF}'`

FAUCET_PACKAGE=`resim publish ../../basic_resource_faucet | tee /dev/tty | awk '/Package:/ {print $NF}'`

LENDING_MARKET_PACKAGE=`resim publish ../. | tee /dev/tty | awk '/Package:/ {print $NF}'`

# ------------------------------------------------------------------------------------ Mint badges
out=`resim call-function $PRICE_FEED_PACKAGE PriceFeed instantiate | tee /dev/tty | awk '/Component:|Resource:/ {print $NF}'`
PRICE_FEED_COMPONENT_ADDRESS=`echo $out | cut -d " " -f1`
PRICE_FEED_ADMIN_BADGE=`echo $out | cut -d " " -f2`

out=`resim call-function $FAUCET_PACKAGE Faucet instantiate $PRICE_FEED_COMPONENT_ADDRESS | tee /dev/tty | awk '/Component:|Resource:/ {print $NF}'`
FAUCET_COMPONENT_ADDRESS=`echo $out | cut -d " " -f1`
FAUCET_ADMIN_BADGE=`echo $out | cut -d " " -f2`

# ------------------------------------------------------------------------------------ Instantiate resources
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$FAUCET_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$FAUCET_COMPONENT_ADDRESS\") \"create_resource\" \"USDT\" \"USDT\" \"https://res.cloudinary.com/daisvxhyu/image/upload/v1679440531/825_lkjddk.png\"  Decimal(\"1000000\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

RESULT=$(resim run "tx.rtm")

USDT_RESOURCE_ADDRESS=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')

# ------------------------------------------------------------------------------------ Set prices
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"lock_fee\" Decimal(\"10\");" > tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_feed\" Address(\"$USDT_RESOURCE_ADDRESS\") Bytes(\"$SUPRA_PROOF\") Enum<1u8>(276u32, 1u32);" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XRD\") Decimal(\"1\");" >> tx.rtm
# SUPRA ---------------------> 
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$USDT_RESOURCE_ADDRESS\") Decimal(\"25\");" >> tx.rtm
# <--------------------- SUPRA
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm

# ------------------------------------------------------------------------------------ Create lending market component
echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"lock_fee\"
    Decimal(\"100\");" > tx.rtm

echo "CALL_FUNCTION
    Address(\"$LENDING_MARKET_PACKAGE\")
    \"LendingMarket\"
    \"instantiate\"
    Tuple(
        10u8, 
        Decimal(\"0.4\")
        Decimal(\"0.99\")
    )
;" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\")
;" >> tx.rtm

RESULT=$(resim run "tx.rtm")

LENDING_MARKET_COMPONENT_ADDRESS=$(echo "$RESULT" | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p" | sed '1!d')
LENDING_MARKET_ADMIN_ADDRESS=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '1!d')
LENDING_MARKET_RESERVE_COLLECTOR_BADGE=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '2!d')
LENDING_MARKET_CDP_RESOURCE_ADDRESS=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '3!d')
LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '4!d')
LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '4!d')
LENDING_MARKET_LIQUIDATOR_BADGE=$(echo "$RESULT" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '5!d')

echo  "LENDING_MARKET_COMPONENT_ADDRESS $LENDING_MARKET_COMPONENT_ADDRESS"
echo  "LENDING_MARKET_ADMIN_ADDRESS $LENDING_MARKET_ADMIN_ADDRESS"
echo  "LENDING_MARKET_RESERVE_COLLECTOR_BADGE $LENDING_MARKET_RESERVE_COLLECTOR_BADGE"
echo  "LENDING_MARKET_CDP_RESOURCE_ADDRESS $LENDING_MARKET_CDP_RESOURCE_ADDRESS"
echo  "LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS $LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS"
echo  "LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS $LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS"
echo  "LENDING_LIQUIDATION_PEEK_RESOURCE_ADDRESS $LENDING_LIQUIDATION_PEEK_RESOURCE_ADDRESS"
echo  "LENDING_MARKET_LIQUIDATOR_BADGE $LENDING_MARKET_LIQUIDATOR_BADGE"

# ------------------------------------------------------------------------------------ Create liquidity pools
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\")\"lock_fee\" Decimal(\"5000\");" > tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\" Address(\"$LENDING_MARKET_ADMIN_ADDRESS\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"),NonFungibleLocalId(\"#2#\"),NonFungibleLocalId(\"#3#\"),NonFungibleLocalId(\"#4#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XRD\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        5i64,
        15i64,
        240i64,
        Decimal(\"0.8\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"0.75\")
    )
    Tuple(
        Enum<1u8>(
            Decimal(\"0.7\")
        ),
        Enum<1u8>(
            Decimal(\"0.7\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.7\")
    );" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$USDT_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        1u8,
        Decimal(\"0\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        5i64,
        15i64,
        240i64,
        Decimal(\"0.45\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"3.00\")
    )
    Tuple(
        Enum<1u8>(
            Decimal(\"0.8\")
        ),
        Enum<1u8>(
            Decimal(\"0.8\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.8\")
    );" >> tx.rtm
echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm

# ------------------------------------------------------------------------------------ Create liquidity pool provider account
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:/ {print $NF}'`
LP_PROVIDER_ADDRESS=`echo $out | cut -d " " -f1`
LP_PROVIDER_PUBKEY=`echo $out | cut -d " " -f2`
LP_PROVIDER_PVKEY=`echo $out | cut -d " " -f3`
LP_PROVIDER_NONFUNGIBLEGLOBALID=`resim new-simple-badge --name 'OwnerBadge' | awk '/NonFungibleGlobalId:/ {print $NF}'`

# ------------------------------------------------------------------------------------ Create borrower account
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:/ {print $NF}'`
BORROWER_ADDRESS=`echo $out | cut -d " " -f1`
BORROWER_PUBKEY=`echo $out | cut -d " " -f2`
BORROWER_PVKEY=`echo $out | cut -d " " -f3`
BORROWER_NONFUNGIBLEGLOBALID=`resim new-simple-badge --name 'OwnerBadge' | awk '/NonFungibleGlobalId:/ {print $NF}'`

# ------------------------------------------------------------------------------------ Create Liquidator account
out=`resim new-account | tee /dev/tty | awk '/Account component address:|Public key:|Private key:/ {print $NF}'`
LIQUIDATOR_ADDRESS=`echo $out | cut -d " " -f1`
LIQUIDATOR_PUBKEY=`echo $out | cut -d " " -f2`
LIQUIDATOR_PVKEY=`echo $out | cut -d " " -f3`
LIQUIDATOR_NONFUNGIBLEGLOBALID=`resim new-simple-badge --name 'OwnerBadge' | awk '/NonFungibleGlobalId:/ {print $NF}'`
