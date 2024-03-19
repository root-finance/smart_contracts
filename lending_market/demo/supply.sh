#!/usr/bin/env sh

# ------------------------------------------------------------------------------------ Supply
resim set-default-account $LP_PROVIDER_ADDRESS  $LP_PROVIDER_PVKEY $LP_PROVIDER_NONFUNGIBLEGLOBALID

out=`resim call-method $FAUCET free`
echo $out
out=`resim call-method $FAUCET free`
echo $out
out=`resim call-method $FAUCET free`
echo $out

echo "CALL_METHOD Address(\"$LP_PROVIDER_ADDRESS\") \"lock_fee\" Decimal(\"100\");" > tx.rtm 

echo "CALL_METHOD
    Address(\"$LP_PROVIDER_ADDRESS\")
    \"withdraw\"
    Address(\"$XRD\")
    Decimal(\"10000\");" >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$XRD\")
    Bucket(\"xrd_bucket\");" >> tx.rtm 

echo "CALL_METHOD
    Address(\"$FAUCET_COMPONENT_ADDRESS\")
    \"get_resource\"
    Address(\"$USDT_RESOURCE_ADDRESS\")
    Bucket(\"xrd_bucket\"); " >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP Address(\"$USDT_RESOURCE_ADDRESS\") Bucket(\"usdt_bucket\");" >> tx.rtm
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"contribute\"  Bucket(\"usdt_bucket\");" >> tx.rtm
echo "CALL_METHOD Address(\"$LP_PROVIDER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm
