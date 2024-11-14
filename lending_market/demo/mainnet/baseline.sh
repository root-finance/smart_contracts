#!/usr/bin/env sh

echo "" > tx.rtm;

# ------------------------------------------------------------------------------------ Reinitialize
OWNER_ADDRESS=account_rdx12ykkpf2v0f3hdqtez9yjhyt04u5ct455aqkq5scd5hlecwf20hcvd2
XRD=resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd


# ------------------------------------------------------------------------------------ Instantiate packages

PRICE_FEED_PACKAGE=package_rdx1phwl26ara0qfcjl4f2w2fjl7jpsc432aa382uswqj88sj6jqc6anrh

POOL_PACKAGE=package_rdx1pkhvtjl4m968u3jlxmehnszxwn0kzake49wvfw4x45lu43eqm96c80

LENDING_MARKET_PACKAGE=package_rdx1phwak2lr7nczzl6rxzvtnjwszmvxqycp9h8pckcmy6uwdcucnjeu0p

# ------------------------------------------------------------------------------------ Mint badges

echo "CALL_FUNCTION
    Address(\"$PRICE_FEED_PACKAGE\")
    \"PriceFeed\"
    \"instantiate\";" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
PRICE_FEED_COMPONENT_ADDRESS=component_rdx1cr9alunsmm5c42275fh0rh3kvqfupdhlf84tnkawspq6as7cysqn98
PRICE_FEED_ADMIN_BADGE=resource_rdx1nt72dpj5meu9nemwprkr0w7jqsywy7g9qafcgsvt6s4vnc0lk8lm8h
PRICE_FEED_UPDATER_BADGE=resource_rdx1nf0ktp0jqhn46t9rk3sgsa3dhwjz7asd9g72kx3l5u38d2zapxkrcf

USDT_RESOURCE_ADDRESS=resource_rdx1thrvr3xfs2tarm2dl9emvs26vjqxu6mqvfgvqjne940jv0lnrrg7rw
XWBTC_RESOURCE_ADDRESS=resource_rdx1t580qxc7upat7lww4l2c4jckacafjeudxj5wpjrrct0p3e82sq4y75
XETH_RESOURCE_ADDRESS=resource_rdx1th88qcj5syl9ghka2g9l7tw497vy5x6zaatyvgfkwcfe8n9jt2npww
LSU_RESOURCE_ADDRESS=resource_rdx1thksg5ng70g9mmy9ne7wz0sc7auzrrwy7fmgcxzel2gvp8pj0xxfmf
HUG_RESOURCE_ADDRESS=resource_rdx1t5kmyj54jt85malva7fxdrnpvgfgs623yt7ywdaval25vrdlmnwe97
XUSDC_RESOURCE_ADDRESS=resource_rdx1t4upr78guuapv5ept7d7ptekk9mqhy605zgms33mcszen8l9fac8vf

# ------------------------------------------------------------------------------------ Set prices
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XRD\") Decimal(\"1\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$USDT_RESOURCE_ADDRESS\") Decimal(\"25\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XWBTC_RESOURCE_ADDRESS\") Decimal(\"1300000\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XETH_RESOURCE_ADDRESS\") Decimal(\"72500\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$LSU_RESOURCE_ADDRESS\") Decimal(\"1\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$HUG_RESOURCE_ADDRESS\") Decimal(\"0.001\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XUSDC_RESOURCE_ADDRESS\") Decimal(\"25\");" >> tx.rtm



echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_feed\" Address(\"$USDT_RESOURCE_ADDRESS\") Bytes(\"$SUPRA_PROOF\") Enum<1u8>(276u32, 1u32);" >> tx.rtm


echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

# ------------------------------------------------------------------------------------ Create lending market component
echo "CALL_FUNCTION
    Address(\"$LENDING_MARKET_PACKAGE\")
    \"LendingMarket\"
    \"instantiate\"
    Tuple(
        14u8, 
        Decimal(\"0.432\"),
        Decimal(\"0.5\")
    );" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm


LENDING_MARKET_COMPONENT_ADDRESS=component_rdx1crwusgp2uy9qkzje9cqj6pdpx84y94ss8pe7vehge3dg54evu29wtq
LENDING_MARKET_ADMIN_ADDRESS=resource_rdx1nf7n5vnkqmuja5l0wukkmzqkg73xa9gg0pzm9ykh89yuw4qz6a69d5
LENDING_MARKET_RESERVE_COLLECTOR_BADGE=resource_rdx1ngn0nm64u90m2gd376axlq5nr64cplx2m7r6ut5lejv8s6ywhf47lc
LENDING_MARKET_CDP_RESOURCE_ADDRESS=resource_rdx1ngekvyag42r0xkhy2ds08fcl7f2ncgc0g74yg6wpeeyc4vtj03sa9f
LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS=resource_rdx1ntegfaj82psqes6hd8pk22qdmy3npj0eyjzx47cynfnrd4czkh9ffg
LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS=resource_rdx1ntegfaj82psqes6hd8pk22qdmy3npj0eyjzx47cynfnrd4czkh9ffg
LENDING_MARKET_LIQUIDATOR_BADGE=resource_rdx1n23fjz2v7fm6wz48r6q728h5kqtwf3qswmdgfuzd558zvs9ezfwv9q


# ------------------------------------------------------------------------------------ Create liquidity pools
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\" Address(\"$LENDING_MARKET_ADMIN_ADDRESS\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"),NonFungibleLocalId(\"#2#\"),NonFungibleLocalId(\"#3#\"),NonFungibleLocalId(\"#4#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XRD\")
    Tuple(
        Decimal(\"0.3\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        1i64,
        4i64,
        240i64,
        Decimal(\"0.45\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"3.00\")
    )
    Tuple(
        Enum<1u8>(
            Decimal(\"0.65\")
        ),
        Enum<1u8>(
            Decimal(\"0.65\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.65\")
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
        1i64,
        4i64,
        240i64,
        Decimal(\"0.8\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"0.75\")
    )
    Tuple(
        Enum<1u8>(
            Decimal(\"0.75\")
        ),
        Enum<1u8>(
            Decimal(\"0.75\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.75\")
    );" >> tx.rtm
echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm


echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\" Address(\"$LENDING_MARKET_ADMIN_ADDRESS\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"),NonFungibleLocalId(\"#2#\"),NonFungibleLocalId(\"#3#\"),NonFungibleLocalId(\"#4#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XWBTC_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.25\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        1i64,
        4i64,
        240i64,
        Decimal(\"0.45\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"3.00\")
    )
    Tuple(
        Enum<1u8>(
            Decimal(\"0.75\")
        ),
        Enum<1u8>(
            Decimal(\"0.75\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.75\")
    );" >> tx.rtm

echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XETH_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.25\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        1i64,
        4i64,
        240i64,
        Decimal(\"0.45\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"3.00\")
    )
    Tuple(
        Enum<1u8>(
            Decimal(\"0.75\")
        ),
        Enum<1u8>(
            Decimal(\"0.75\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.75\")
    );" >> tx.rtm

echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$LSU_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.3\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        1i64,
        4i64,
        240i64,
        Decimal(\"0.45\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"3.00\")
    )
    Tuple(
        Enum<1u8>(
            Decimal(\"0.65\")
        ),
        Enum<1u8>(
            Decimal(\"0.65\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.65\")
    );" >> tx.rtm

echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$HUG_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<1u8>(Decimal(\"0\")),
        Enum<0u8>(),
        1i64,
        4i64,
        240i64,
        Decimal(\"0.45\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"3.00\")
    )
    Tuple(
        Enum<1u8>(
            Decimal(\"0.3\")
        ),
        Enum<1u8>(
            Decimal(\"0.3\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.3\")
    );" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XUSDC_RESOURCE_ADDRESS\")
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
        1i64,
        4i64,
        240i64,
        Decimal(\"0.75\")
    )
    Tuple(
        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"0.75\")
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


# ------------------------------------------------------------------------------------ Create liquidity pool provider account
LP_PROVIDER_ADDRESS=account_tdx_2_1285jcu8wkxlz8kpp3euyhwq08fqwhcrahxt273eapvvsl7wsmyals8

# ------------------------------------------------------------------------------------ Create borrower account
BORROWER_ADDRESS=account_tdx_2_12x326aecwaepr3jmxycxfhk3cde3l9lcew5e04rm7h8nq30kefn33a

# ------------------------------------------------------------------------------------ Create Liquidator account
LIQUIDATOR_ADDRESS=account_tdx_2_12y8g9hxfa8vx4ut85hqsrkdqehyjxvrgu28htdc0cjanzuu6hxxucl
