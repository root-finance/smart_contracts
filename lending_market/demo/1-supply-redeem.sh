source ./baseline.sh 
# ------------------------------------------------------------------------------------ Supply
source ./supply.sh 

# ------------------------------------------------------------------------------------ Redeem
LP_UNITS_RESOURCE_ADDRESS=`resim show $LP_PROVIDER_ADDRESS | grep "rtUSDT" | cut -d " " -f2 | cut -d ":" -f1`

echo "CALL_METHOD Address(\"$LP_PROVIDER_ADDRESS\") \"lock_fee\" Decimal(\"100\");" > tx.rtm 

echo "CALL_METHOD
    Address(\"$LP_PROVIDER_ADDRESS\")
    \"withdraw\"
    Address(\"$LP_UNITS_RESOURCE_ADDRESS\")
    Decimal(\"200\");" >> tx.rtm 

echo "TAKE_ALL_FROM_WORKTOP
    Address(\"$LP_UNITS_RESOURCE_ADDRESS\")
    Bucket(\"rusdt_bucket\");" >> tx.rtm 
echo "CALL_METHOD Address(\"$LENDING_MARKET_COMPONENT_ADDRESS\") \"redeem\"  Bucket(\"rusdt_bucket\");" >> tx.rtm
echo "CALL_METHOD Address(\"$LP_PROVIDER_ADDRESS\") \"deposit_batch\" Expression(\"ENTIRE_WORKTOP\");" >> tx.rtm

resim run tx.rtm
resim show $LP_PROVIDER_ADDRESS
