#!/usr/bin/env sh

echo "" > tx.rtm;

# ------------------------------------------------------------------------------------ Reinitialize
OWNER_ADDRESS=account_tdx_2_12y5yhfvsd7ya0jsvv5p8v5m9jvmudkfqjknwy2j84uduh90kpvk8ch
XRD=resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc
SUPRA_PROOF=5c2101202101040a000000000000000020072097005e53b1a210741ce728e51ee02345267f7c429086d199b67f870b3f1799ee2007608ae6587fbd751aced028d0e5314e6a5dd606b760aea03a077e0b6f4ac2efdd42c3f559284e305dade21569980761f8a9138edd0a2423fc07348457f3564350795b0e00ee8ec713d818099e57b6907e3a4c673fe076bc7931169225314b839c2420210102210509140100000b0080a4a61971d60000000000000000000a9016e8e08e0100000812000a5015e8e08e0100002020090720dd5490e8c99e44e441c86b514cf4f7c1d3b0b1e82d6a2efcefbc3c3c236b6a520720d9503c6895245d1c69b261d083687abb08a4a21ffe33f0405b8ea16c42f9647e07208fb25af57390a0555ed7d04b1ece82c00df85fd5b5cf4985bcbb160cc8fa318807205ff7d20a6d4b8e63d9feeae85564195f40e70cac0a10a39ed286451efa0fd96007205ca0c5d8981c5bebf80eb759450add92176e6b04a8c1e8062be3a31a790a17dd0720daff9a68fbb4762377d825c2ab4c84fee793a818d82fe7e49fd77dd852547fc7072063ec35bd32b820452f8e5dd329484d6189daca59f38bd318f717fda4677cccd307200a6eaeb7e0927c73e734d29363f833b6d39dcaec225fabb11e0aa28dc16a919107200b6e5c5d2ad0b3e32eb90343b3a71e1aeeeeb25d2a4540d9fb1627e830bee6b0


# ------------------------------------------------------------------------------------ Instantiate packages

PRICE_FEED_PACKAGE=package_tdx_2_1p5lknmvchhuwwdecrler9edjlvguf2fzdck3jm4aq2vre7v27r9sjl

POOL_PACKAGE=package_tdx_2_1p57jp8na4jhnep6acjerk9thz0q3y87u6d5k30ajm9m8w6y0fdpajy

LENDING_MARKET_PACKAGE=package_tdx_2_1phgcn4t8zkcrugxs9k7dhah2g6s4mcktg3fu7txh0yqsxn8vg8nfx3

FAUCET_PACKAGE=package_tdx_2_1ph0c86sfdzeeyur58rz3gev8yd2ed45q8jxn3a50d807k5kqdeq7aj

# ------------------------------------------------------------------------------------ Mint badges

echo "CALL_FUNCTION
    Address(\"$PRICE_FEED_PACKAGE\")
    \"PriceFeed\"
    \"instantiate\";" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
PRICE_FEED_COMPONENT_ADDRESS=component_tdx_2_1cq4nv43cdntjn6wcas7wlf7fa7eaq0p9y7e5ekytlg54k6rm6v6hv5
PRICE_FEED_ADMIN_BADGE=resource_tdx_2_1ntc5r00pcamge975rfnl5zu9tc9e303vdegephxpx4ex0ud4wv4gq0
PRICE_FEED_UPDATER_BADGE=resource_tdx_2_1ngpcsxhyc2q0a05hsycp2yrase0whd6hwkexrdtt8qnpc94ulc2455

echo "CALL_FUNCTION
    Address(\"$FAUCET_PACKAGE\")
    \"Faucet\"
    \"instantiate\"
    Address(\"$PRICE_FEED_COMPONENT_ADDRESS\");" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
FAUCET_COMPONENT_ADDRESS=component_tdx_2_1cqzu85x8sa34sgxn05dz03jq492c3f5q5k9vusjm6pl0ca6ths4ymd
FAUCET_ADMIN_BADGE=resource_tdx_2_1nt6l07kdv4w3ssaf7k9n5zvvyntwgtw4927vz5hjg9x56pq50qpnmk

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

USDT_RESOURCE_ADDRESS=resource_tdx_2_1t4mnpjclyk5w994fgercpyfpprqt839wwjywqnxunq96pkdkssx80r
XWBTC_RESOURCE_ADDRESS=resource_tdx_2_1thc05pngjmyrvjnwultf3ajtx8z4z9q8pn4qnhrknwpax7yeld0h8m
XETH_RESOURCE_ADDRESS=resource_tdx_2_1t4m9d5493yz9uux0jc95gz0kwu6zhxa3cwwycnuy4tul4ess6apdyp
LSU_RESOURCE_ADDRESS=resource_tdx_2_1t5q69gymymlleyxx2y8glsjvs6x30hs9dknhce6ten3v7zhrd6lwrd
HUG_RESOURCE_ADDRESS=resource_tdx_2_1thvu4phuexzp794xa6p0ng0p4768pv6lgszx62lsxxk49jd0p2kgyy
XUSDC_RESOURCE_ADDRESS=resource_tdx_2_1t5u0vxy0f95dyhk5frz50z4qae0hc760lr4gr6e63jaray7xl03cx4

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
        Decimal(\"0.4\"),
        Decimal(\"0.99\")
    );" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm


LENDING_MARKET_COMPONENT_ADDRESS=component_tdx_2_1cz06urzruueelc34m6mnajx0z7g3xq96krt43gcvwsn0c4p0q4pfqr
LENDING_MARKET_ADMIN_ADDRESS=resource_tdx_2_1ntrrvmz5tqgh5fgwzg450pnmp50r6pnea5zkfq7nxdeqlxtaqr6zhz
LENDING_MARKET_RESERVE_COLLECTOR_BADGE=resource_tdx_2_1nfj8mru6xljzuhrrx4vuunn8zn8hc3p6jpgvyhvnd79uayx3aprdk3
LENDING_MARKET_CDP_RESOURCE_ADDRESS=resource_tdx_2_1n2gw33zggksf60yn0s3crf2gr049jpgquzh09jexnshlxf5yg9ph6t
LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS=resource_tdx_2_1ntxcx6s7mjwr84j60w8c7er64mcah05ms5m3utgharvq5xd0che3gc
LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS=resource_tdx_2_1ntxcx6s7mjwr84j60w8c7er64mcah05ms5m3utgharvq5xd0che3gc
LENDING_MARKET_LIQUIDATOR_BADGE=resource_tdx_2_1nfd7vegrkvlp9uvgdqhveqy855pnhqmhu2jwnr0ys42cxd256tw7pt


# ------------------------------------------------------------------------------------ Create liquidity pools
echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\" Address(\"$LENDING_MARKET_ADMIN_ADDRESS\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"),NonFungibleLocalId(\"#2#\"),NonFungibleLocalId(\"#3#\"),NonFungibleLocalId(\"#4#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XRD\")
    Tuple(
        Decimal(\"0.35\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.1\"),
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

echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$USDT_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        1u8,
        Decimal(\"0.1\"),
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


echo "CALL_METHOD Address(\"$OWNER_ADDRESS\") \"create_proof_of_non_fungibles\" Address(\"$LENDING_MARKET_ADMIN_ADDRESS\") Array<NonFungibleLocalId>(NonFungibleLocalId(\"#1#\"),NonFungibleLocalId(\"#2#\"),NonFungibleLocalId(\"#3#\"),NonFungibleLocalId(\"#4#\"));" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$XWBTC_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.25\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.1\"),
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
        Decimal(\"0.1\"),
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

#echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$LSU_RESOURCE_ADDRESS\")
#    Tuple(
#        Decimal(\"0.35\"),
#        Decimal(\"0.15\"),
#        Decimal(\"0.08\"),
#        Decimal(\"0.001\"),
#        0u8,
#        Decimal(\"0.1\"),
#        Decimal(\"1\"),
#        Enum<0u8>(),
#        Enum<0u8>(),
#        Enum<0u8>(),
#        1i64,
#        4i64,
#        240i64,
#        Decimal(\"0.45\")
#    )
#    Tuple(
#        Decimal(\"0\"), Decimal(\"0.04\"), Decimal(\"3.00\")
#    )
#    Tuple(
#        Enum<1u8>(
#            Decimal(\"0.75\")
#        ),
#        Enum<1u8>(
#            Decimal(\"0.75\")
#        ),
#        Map<Address, Decimal>(),
#        Map<U8, Decimal>(),
#        Decimal(\"0.75\")
#    );" >> tx.rtm

echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"create_lending_pool\" Address(\"$PRICE_FEED_COMPONENT_ADDRESS\") Address(\"$HUG_RESOURCE_ADDRESS\")
    Tuple(
        Decimal(\"0.2\"),
        Decimal(\"0.15\"),
        Decimal(\"0.08\"),
        Decimal(\"0.001\"),
        0u8,
        Decimal(\"0.1\"),
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
        Decimal(\"0.1\"),
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
            Decimal(\"0.85\")
        ),
        Enum<1u8>(
            Decimal(\"0.85\")
        ),
        Map<Address, Decimal>(),
        Map<U8, Decimal>(),
        Decimal(\"0.85\")
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
