#!/usr/bin/env sh

echo "" > tx.rtm;

# ------------------------------------------------------------------------------------ Reinitialize
OWNER_ADDRESS=account_tdx_2_1290qzud9wh40gp6ckcmcgu60hj7g08v2ftw7hk5crm59p6tua93jdk
XRD=resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc
SUPRA_PROOF=CAFEBABE # TODO


# ------------------------------------------------------------------------------------ Instantiate packages

POOL_PACKAGE=package_tdx_2_1phqmc3pcvggna0xtprl6lvdhvrgps4kmkcdlgcp7lamxnna8q440d9

PRICE_FEED_PACKAGE=package_tdx_2_1pkfj7uu9229ws7pzkcfw6gqujw8zuf26slrudwg5eh5zg823nrueu0

LENDING_MARKET_PACKAGE=package_tdx_2_1pkdd82qryv2s7ly5kdtdsa43rjlmc9q3kd0n29cetu9ffspqzpspwe

FAUCET_PACKAGE=package_tdx_2_1p57eldur7nakguzepj2yhn0hz6rquxf6xaqfwqtmvz8sek0kvv2rmq

# ------------------------------------------------------------------------------------ Mint badges

echo "CALL_FUNCTION
    Address(\"$PRICE_FEED_PACKAGE\")
    \"PriceFeed\"
    \"instantiate\";" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
PRICE_FEED_COMPONENT_ADDRESS=component_tdx_2_1cpzas48thjfqjdqgvk9lk87werqeyj9zj9xvgsmgd6fylakrlzvj3t
PRICE_FEED_ADMIN_BADGE=resource_tdx_2_1nf8qj6nn4m92g4j9v74jg957agtdzcl6sp6ypmax57zcyn65kqxnxs

echo "CALL_FUNCTION
    Address(\"$FAUCET_PACKAGE\")
    \"Faucet\"
    \"instantiate\"
    Address(\"$PRICE_FEED_COMPONENT_ADDRESS\");" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
FAUCET_COMPONENT_ADDRESS=component_tdx_2_1crchxcqvas2tjzesqp8rrw8lvdtlhw4k5pcaeqjzqvvr08vwm8z3ms
FAUCET_ADMIN_BADGE=resource_tdx_2_1nfh62hky7df7kwlwkq4pr3uxkjetrth6nq37szmqwvyy2y8dcg0cgf

# ------------------------------------------------------------------------------------ Instantiate resources
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$FAUCET_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$FAUCET_COMPONENT_ADDRESS\") \"create_resource\" \"USDT\" \"USDT\" \"https://res.cloudinary.com/daisvxhyu/image/upload/v1679440531/825_lkjddk.png\"  Decimal(\"1000000\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
export USDT_RESOURCE_ADDRESS=resource_tdx_2_1tkak2k6lycwcakgqdgkqe7whja3rr24j25j6ncdef0lcp3yut6lv4p


# ------------------------------------------------------------------------------------ Set prices
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_feed\" Address(\"$USDT_RESOURCE_ADDRESS\") Bytes(\"$SUPRA_PROOF\") Enum<1u8>(276u32, 1u32);" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XRD\") Decimal(\"1\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

# ------------------------------------------------------------------------------------ Create lending market component
echo "CALL_FUNCTION
    Address(\"$LENDING_MARKET_PACKAGE\")
    \"LendingMarket\"
    \"instantiate\"
    Tuple(10u8,);" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm


export LENDING_MARKET_COMPONENT_ADDRESS=component_tdx_2_1czyar79nj556nafg4m4duxzrf3t8f27wu8cdd903kg7rx8wpgrjzmu
export LENDING_MARKET_ADMIN_ADDRESS=resource_tdx_2_1ngdatt9haxlr8c68kfpm4pdkm67nda5g67664zg83zwejq8ue850pk
export LENDING_MARKET_RESERVE_COLLECTOR_BADGE=resource_tdx_2_1nf0x3eg6zk2k2a6z4k3nd9gt562wnl2u0upgt5gq4uxp3k8pnwtmrn
export LENDING_MARKET_CDP_RESOURCE_ADDRESS=resource_tdx_2_1n2nt0tmjd39kc8gysdrntl76tyylqt5rkeqeawfppds5s2tzszaw38
export LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS=resource_tdx_2_1ntpyl3zdes5ma30r4jd72vy8taxkjv92pe8zaw09ke9h3je70nhc5m
export LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS=resource_tdx_2_1ntpyl3zdes5ma30r4jd72vy8taxkjv92pe8zaw09ke9h3je70nhc5m

# ------------------------------------------------------------------------------------ Create liquidity pools
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\" Address(\"$LENDING_MARKET_ADMIN_ADDRESS\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"),NonFungibleLocalId(\"#2#\"),NonFungibleLocalId(\"#3#\"),NonFungibleLocalId(\"#4#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XRD\")
    Tuple(
        Decimal(\"0.15\"),
        Decimal(\"0.15\"),
        Decimal(\"0.15\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.05\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        5i64,
        15i64,
        240i64,
        Decimal(\"0.45\"),
        Decimal(\"0.7\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"3.00\")
    )
    Tuple(
        Enum<0u8>(),
        Enum<1u8>(
            Decimal(\"0.8\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.7\")
    );" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$USDT_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.15\"),
        Decimal(\"0.15\"),
        Decimal(\"0.15\"),
        Decimal(\"0.001\"),
        1u8,
        Decimal(\"0.05\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        5i64,
        15i64,
        240i64,
        Decimal(\"0.8\"),
        Decimal(\"0.8\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"0.75\")
    )
    Tuple(
        Enum<0u8>(),
        Enum<1u8>(
            Decimal(\"0.8\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0\")
    );" >> tx.rtm
echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

# ------------------------------------------------------------------------------------ Create liquidity pool provider account
LP_PROVIDER_ADDRESS=account_tdx_2_1285jcu8wkxlz8kpp3euyhwq08fqwhcrahxt273eapvvsl7wsmyals8

# ------------------------------------------------------------------------------------ Create borrower account
BORROWER_ADDRESS=account_tdx_2_12x326aecwaepr3jmxycxfhk3cde3l9lcew5e04rm7h8nq30kefn33a

# ------------------------------------------------------------------------------------ Create Liquidator account
LIQUIDATOR_ADDRESS=account_tdx_2_12y8g9hxfa8vx4ut85hqsrkdqehyjxvrgu28htdc0cjanzuu6hxxucl
