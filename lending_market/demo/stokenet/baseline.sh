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


export LENDING_MARKET_COMPONENT_ADDRESS=component_tdx_2_1czaz7w94hyw4tyus9p7lmc68nruruud2e076dglujp8s7hjq7p2a5h
export LENDING_MARKET_ADMIN_ADDRESS=resource_tdx_2_1n27z7emuvlmdc4r9s2ncxplwtm33jhd34tqqr2ql7tezfdnd2dv4cl
export LENDING_MARKET_RESERVE_COLLECTOR_BADGE=resource_tdx_2_1ntyqj3n2p8zwwfemkrtxfhplw52m9z2fjmeqc2m4tyqckr324f3jge
export LENDING_MARKET_CDP_RESOURCE_ADDRESS=resource_tdx_2_1ng4r8j9qsjc34wdpd07adkwdty5za0huzg3k43mjwf2arcz229tjap
export LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS=resource_tdx_2_1nfk0rrefj78vr7um8ghx4zn732akyl4pu5gxmwmemyu0g4j2s84ksu
export LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS=resource_tdx_2_1nfk0rrefj78vr7um8ghx4zn732akyl4pu5gxmwmemyu0g4j2s84ksu

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
        240i64
    )
    Tuple(
        Decimal(\"0.05\"),
        Array<Tuple>(
            Tuple(
                Decimal(\"0\"),
                Decimal(\"0.3\")
            ),
            Tuple(
                Decimal(\"0.4\"),
                Decimal(\"3\")
            )
        )
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
        240i64
    )
    Tuple(
        Decimal(\"0.05\"),
        Array<Tuple>(
            Tuple(
                Decimal(\"0\"),
                Decimal(\"0.5\")
            ),
            Tuple(
                Decimal(\"0.8\"),
                Decimal(\"5\")
            )
        )
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
LP_PROVIDER_ADDRESS=account_tdx_2_12890ptdk2m7298eatk65wkgky4zv5vu6uca3qtvqv6zd2tj8t0rl9p

# ------------------------------------------------------------------------------------ Create borrower account
BORROWER_ADDRESS=account_tdx_2_12xujvlcj5gg2acmwzg8d397kkhek2ly3l0xvtcwwz7n3zpkjutvwdn

# ------------------------------------------------------------------------------------ Create Liquidator account
LIQUIDATOR_ADDRESS=account_tdx_2_12y8g9hxfa8vx4ut85hqsrkdqehyjxvrgu28htdc0cjanzuu6hxxucl
