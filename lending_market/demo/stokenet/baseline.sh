#!/usr/bin/env sh

echo "" > tx.rtm;

# ------------------------------------------------------------------------------------ Reinitialize
OWNER_ADDRESS=account_tdx_2_1290qzud9wh40gp6ckcmcgu60hj7g08v2ftw7hk5crm59p6tua93jdk
XRD=resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc
SUPRA_PROOF=5c2101202101040a000000000000000020072097005e53b1a210741ce728e51ee02345267f7c429086d199b67f870b3f1799ee2007608ae6587fbd751aced028d0e5314e6a5dd606b760aea03a077e0b6f4ac2efdd42c3f559284e305dade21569980761f8a9138edd0a2423fc07348457f3564350795b0e00ee8ec713d818099e57b6907e3a4c673fe076bc7931169225314b839c2420210102210509140100000b0080a4a61971d60000000000000000000a9016e8e08e0100000812000a5015e8e08e0100002020090720dd5490e8c99e44e441c86b514cf4f7c1d3b0b1e82d6a2efcefbc3c3c236b6a520720d9503c6895245d1c69b261d083687abb08a4a21ffe33f0405b8ea16c42f9647e07208fb25af57390a0555ed7d04b1ece82c00df85fd5b5cf4985bcbb160cc8fa318807205ff7d20a6d4b8e63d9feeae85564195f40e70cac0a10a39ed286451efa0fd96007205ca0c5d8981c5bebf80eb759450add92176e6b04a8c1e8062be3a31a790a17dd0720daff9a68fbb4762377d825c2ab4c84fee793a818d82fe7e49fd77dd852547fc7072063ec35bd32b820452f8e5dd329484d6189daca59f38bd318f717fda4677cccd307200a6eaeb7e0927c73e734d29363f833b6d39dcaec225fabb11e0aa28dc16a919107200b6e5c5d2ad0b3e32eb90343b3a71e1aeeeeb25d2a4540d9fb1627e830bee6b0


# ------------------------------------------------------------------------------------ Instantiate packages

POOL_PACKAGE=package_tdx_2_1p4twa63f87utfvqatt0u9ucvea4ykhjnv67pum8x807m459w04lgfd

PRICE_FEED_PACKAGE=package_tdx_2_1pkfj7uu9229ws7pzkcfw6gqujw8zuf26slrudwg5eh5zg823nrueu0

LENDING_MARKET_PACKAGE=package_tdx_2_1pkt5leaa0kpmxuukard8khrewa0vmwqd0m9w4kqf9pjzt73rdcrxx0

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
echo "CALL_METHOD Address(\"$FAUCET_COMPONENT_ADDRESS\") \"create_resource\" \"xUSDT\" \"xUSDT\" \"https://res.cloudinary.com/daisvxhyu/image/upload/v1679440531/825_lkjddk.png\"  Decimal(\"1000000\");" >> tx.rtm

echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$FAUCET_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$FAUCET_COMPONENT_ADDRESS\") \"create_resource\" \"xWBTC\" \"xWBTC\" \"https://res.cloudinary.com/daisvxhyu/image/upload/v1679440531/825_lkjddk.png\"  Decimal(\"1000000\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$FAUCET_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$FAUCET_COMPONENT_ADDRESS\") \"create_resource\" \"xETH\" \"xETH\" \"https://res.cloudinary.com/daisvxhyu/image/upload/v1679440531/825_lkjddk.png\"  Decimal(\"1000000\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$FAUCET_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$FAUCET_COMPONENT_ADDRESS\") \"create_resource\" \"LSU\" \"LSU\" \"https://res.cloudinary.com/daisvxhyu/image/upload/v1679440531/825_lkjddk.png\"  Decimal(\"1000000\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$FAUCET_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$FAUCET_COMPONENT_ADDRESS\") \"create_resource\" \"HUG\" \"HUG\" \"https://res.cloudinary.com/daisvxhyu/image/upload/v1679440531/825_lkjddk.png\"  Decimal(\"1000000\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$FAUCET_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$FAUCET_COMPONENT_ADDRESS\") \"create_resource\" \"xUSDC\" \"xUSDC\" \"https://res.cloudinary.com/daisvxhyu/image/upload/v1679440531/825_lkjddk.png\"  Decimal(\"1000000\");" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

export USDT_RESOURCE_ADDRESS=resource_tdx_2_1tkak2k6lycwcakgqdgkqe7whja3rr24j25j6ncdef0lcp3yut6lv4p
export XWBTC_RESOURCE_ADDRESS=resource_tdx_2_1tkatgs43legl0rjetydl3a96593rzgvwpxzt6uyg8hc4sztpkss752
export XETH_RESOURCE_ADDRESS=resource_tdx_2_1t59ttjcqftthpy59qug635fw2qrv4nq02ht4ywp0sdtqzelerx6dwq
export LSU_RESOURCE_ADDRESS=resource_tdx_2_1thset9xxx5lt00g2xf5fw3jqeh98ptn7750295zufk83qlvxnmn76j
export HUG_RESOURCE_ADDRESS=resource_tdx_2_1t4u3tqp5hl4y22urfh2yjr44v85wq2sp27p5y7sjaearkg2qljpdmz
export XUSDC_RESOURCE_ADDRESS=resource_tdx_2_1t5ahd2zd55u775l8fnf7at32sj9wunvrgjp267qdzst8mgul90kalm

# ------------------------------------------------------------------------------------ Set prices
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_feed\" Address(\"$USDT_RESOURCE_ADDRESS\") Bytes(\"$SUPRA_PROOF\") Enum<1u8>(276u32, 1u32);" >> tx.rtm
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XRD\") Decimal(\"1\");" >> tx.rtm

echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\"  Address(\"$PRICE_FEED_ADMIN_BADGE\")  Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"));" >> tx.rtm

echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XWBTC_RESOURCE_ADDRESS\") Decimal(\"1\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XETH_RESOURCE_ADDRESS\") Decimal(\"1\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$LSU_RESOURCE_ADDRESS\") Decimal(\"1\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$HUG_RESOURCE_ADDRESS\") Decimal(\"1\");" >> tx.rtm
echo "CALL_METHOD Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") \"admin_update_price\" Address(\"$XUSDC_RESOURCE_ADDRESS\") Decimal(\"1\");" >> tx.rtm

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


export LENDING_MARKET_COMPONENT_ADDRESS=component_tdx_2_1czae9ze5r7kjy6mj7gkjt55udjpmd3n6lymrldnt68agtq3qgfrecc
export LENDING_MARKET_ADMIN_ADDRESS=resource_tdx_2_1nguq2xgf60ks0p0njwsrymz8qqmpu2gtpcxvt5xhnngvhuls0ra3s6
export LENDING_MARKET_RESERVE_COLLECTOR_BADGE=resource_tdx_2_1ngfcvau0h3p8x35caqepeyakd3ulyrf3nug8jgdng0ulvrnxly3krx
export LENDING_MARKET_CDP_RESOURCE_ADDRESS=resource_tdx_2_1n2x5kadrlh0femw9up2qdvgx2mf7ta6ajewj9rpj2mvwel60qa9cgx
export LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS=resource_tdx_2_1nf4dfy0h5x9gug8mf564k2lxfsed6tnznl5eqzz0l54f4nyf6ar9jw
export LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS=resource_tdx_2_1nf4dfy0h5x9gug8mf564k2lxfsed6tnznl5eqzz0l54f4nyf6ar9jw

# ------------------------------------------------------------------------------------ Create liquidity pools
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\" Address(\"$LENDING_MARKET_ADMIN_ADDRESS\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"),NonFungibleLocalId(\"#2#\"),NonFungibleLocalId(\"#3#\"),NonFungibleLocalId(\"#4#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XRD\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.4\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.08\"),
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
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.4\"),
        Decimal(\"0.001\"),
        1u8,
        Decimal(\"0.08\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        5i64,
        15i64,
        240i64,
        Decimal(\"0.8\"),
        Decimal(\"0.75\")
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


echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\" Address(\"$LENDING_MARKET_ADMIN_ADDRESS\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"),NonFungibleLocalId(\"#2#\"),NonFungibleLocalId(\"#3#\"),NonFungibleLocalId(\"#4#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XWBTC_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.4\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.08\"),
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

echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XETH_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.4\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.08\"),
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

echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$LSU_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.4\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.08\"),
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

echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$HUG_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.4\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.08\"),
        Decimal(\"1\"),
        Enum<0u8>(),
        Enum<0u8>(),
        Enum<0u8>(),
        5i64,
        15i64,
        240i64,
        Decimal(\"0.45\"),
        Decimal(\"0.25\")
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
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XUSDC_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.4\"),
        Decimal(\"0.001\"),
        1u8,
        Decimal(\"0.08\"),
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
