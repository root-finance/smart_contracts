#!/usr/bin/env sh

echo "" > tx.rtm;

# ------------------------------------------------------------------------------------ Reinitialize
OWNER_ADDRESS=account_tdx_2_12ymq466ag96y7ksx2ypuj9perjxdts2g43kxfuywu0tq27zzw6hsah
XRD=resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc
SUPRA_PROOF=5c2101202101040a000000000000000020072097005e53b1a210741ce728e51ee02345267f7c429086d199b67f870b3f1799ee2007608ae6587fbd751aced028d0e5314e6a5dd606b760aea03a077e0b6f4ac2efdd42c3f559284e305dade21569980761f8a9138edd0a2423fc07348457f3564350795b0e00ee8ec713d818099e57b6907e3a4c673fe076bc7931169225314b839c2420210102210509140100000b0080a4a61971d60000000000000000000a9016e8e08e0100000812000a5015e8e08e0100002020090720dd5490e8c99e44e441c86b514cf4f7c1d3b0b1e82d6a2efcefbc3c3c236b6a520720d9503c6895245d1c69b261d083687abb08a4a21ffe33f0405b8ea16c42f9647e07208fb25af57390a0555ed7d04b1ece82c00df85fd5b5cf4985bcbb160cc8fa318807205ff7d20a6d4b8e63d9feeae85564195f40e70cac0a10a39ed286451efa0fd96007205ca0c5d8981c5bebf80eb759450add92176e6b04a8c1e8062be3a31a790a17dd0720daff9a68fbb4762377d825c2ab4c84fee793a818d82fe7e49fd77dd852547fc7072063ec35bd32b820452f8e5dd329484d6189daca59f38bd318f717fda4677cccd307200a6eaeb7e0927c73e734d29363f833b6d39dcaec225fabb11e0aa28dc16a919107200b6e5c5d2ad0b3e32eb90343b3a71e1aeeeeb25d2a4540d9fb1627e830bee6b0


# ------------------------------------------------------------------------------------ Instantiate packages

PRICE_FEED_PACKAGE=package_tdx_2_1p5psu0kzcu7jrm7pavq85q2qn34s26mx6pxwedkramd7f5qa7ed4v6

POOL_PACKAGE=package_tdx_2_1phpu7g3hh7n0ffv2rwmngk5r725a0k767m0adg82z29rn83y5n9p3d

LENDING_MARKET_PACKAGE=package_tdx_2_1p4glualhs5twrmg74utqsy9e9dsdr2jrh22xrpkg6w85nr6zhg4a2j

FAUCET_PACKAGE=package_tdx_2_1pkwgr3xz6a52uda3hl3j83ug29255s5a3r29k43ml92x2298r3pyeu

# ------------------------------------------------------------------------------------ Mint badges

echo "CALL_FUNCTION
    Address(\"$PRICE_FEED_PACKAGE\")
    \"PriceFeed\"
    \"instantiate\";" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
PRICE_FEED_COMPONENT_ADDRESS=component_tdx_2_1crsme2ssqcy4guecmv69dgkanlr4fkg0u6w5p3rk8pw4u9rmayz0jv
PRICE_FEED_ADMIN_BADGE=resource_tdx_2_1nt5edh8g96cnrf4wcwer3kks5vytvtce7hdqmkt79ftyjf0t95mclp
PRICE_FEED_UPDATER_BADGE=resource_tdx_2_1nt37ht06cdcyw0rd830eknw4s7azwk4750mu38ud9nu2e55ka576f2

echo "CALL_FUNCTION
    Address(\"$FAUCET_PACKAGE\")
    \"Faucet\"
    \"instantiate\"
    Address(\"$PRICE_FEED_COMPONENT_ADDRESS\");" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm
FAUCET_COMPONENT_ADDRESS=component_tdx_2_1cpze7z68uq5k84gj8jkyue3xesceq0mhtzpwwxpf7hjs6ht4swrhzp
FAUCET_ADMIN_BADGE=resource_tdx_2_1nfmt2qf6ggu5d8p6lr2269qdmv43r086kz3gull3fc7zc8clrhpy6p

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

USDT_RESOURCE_ADDRESS=resource_tdx_2_1t4nwv6aw2n6zt2nx80vxa2h6ynssz75xtjeac4f0hktmapdvh5ve3z
XWBTC_RESOURCE_ADDRESS=resource_tdx_2_1t48fsfmh9kdfhdvge8x5dxee7u38wqcrjwesghjlks8lzmst725ccg
XETH_RESOURCE_ADDRESS=resource_tdx_2_1tkjmydgvva5rl8x0lt9vn5lzpz2xh2d23klzjhv884hm9gg770l720
LSU_RESOURCE_ADDRESS=resource_tdx_2_1t48r0hhzy6nv9kmga7u2fdmexy0j08lxhx9jewdap7uv4j5r64jpgw
HUG_RESOURCE_ADDRESS=resource_tdx_2_1tkna28k99gnj24ngqxvrcl7dh7lrqyr85496guk2zgprjhg8nkvs5h
XUSDC_RESOURCE_ADDRESS=resource_tdx_2_1t5e5q2jsn9eqe5ma0gqtpfjzqcmchjze28rfyttzunu3pr6y6t06t7

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
    Tuple(10u8,);" >> tx.rtm

echo "CALL_METHOD
    Address(\"$OWNER_ADDRESS\")
    \"deposit_batch\"
    Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm


LENDING_MARKET_COMPONENT_ADDRESS=component_tdx_2_1crdm3u3xhsatvpftm9xwf8nwzty65kxj6wryjs8m9rrt527ahl087c
LENDING_MARKET_ADMIN_ADDRESS=resource_tdx_2_1ntjw0j86z23uhqke68j5rjk9e9ny90ks6v9wlrzvfccjrdkzwu3t96
LENDING_MARKET_RESERVE_COLLECTOR_BADGE=resource_tdx_2_1n2tzfw86l7xksr33m2r8vtlacxnfhql982x7rmv5k92cgapvqvf585
LENDING_MARKET_CDP_RESOURCE_ADDRESS=resource_tdx_2_1n28waz5k6kqs06r0ay8cqph22xtxj9sj8cwp5kqk7k0enk6s08gj03
LENDING_BATCH_FLASH_LOAN_RESOURCE_ADDRESS=resource_tdx_2_1nt0rsect89tz26kqcqec86y2yv02p2lgv3rm8tdrjaassrtam0jny0
LENDING_LIQUIDATION_TERM_RESOURCE_ADDRESS=resource_tdx_2_1nt0rsect89tz26kqcqec86y2yv02p2lgv3rm8tdrjaassrtam0jny0

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
