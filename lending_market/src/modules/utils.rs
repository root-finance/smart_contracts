use scrypto::prelude::*;

// CONSTANTS

pub const SECOND_PER_MINUTE: i64 = 60;
pub const MINUTE_PER_YEAR: i64 = 60 * 24 * 365;

/// Check if the given rate is between 0 and 1
/// 
/// *Params*
/// - `rate`: rate to check
/// 
/// *Output*
/// True if the rate is valid, false otherwise
pub fn is_valid_rate(rate: Decimal) -> bool {
    rate >= dec!(0) && rate <= dec!(1)
}

/// Type of withdraw
#[derive(ScryptoSbor, PartialEq)]
pub enum WithdrawType {
    /// Used for flash loans
    TemporaryUse,
    /// Standard liquidity withdrawal
    LiquidityWithdrawal,
}


/// Type of deposit
#[derive(ScryptoSbor, PartialEq)]
pub enum DepositType {
    /// Used for flash loans
    FromTemporaryUse,
    /// Standard liquidity supply
    LiquiditySupply,
}

/// Price info coming from the oracle
#[derive(ScryptoSbor, Clone)]
pub struct PriceInfo {
    /// Timestamp of retrieval
    pub timestamp: i64,
    /// Price
    pub price: Decimal,
}

/// Get the price from the oracle component
/// 
/// *Params*
/// - `price_feed`: The global component address of the price oracle
/// - `res_address`: The resource address of the asset to price
/// 
/// *Output*
/// The `PriceInfo` from the oracle
/// 
/// *Errors*
/// - If the call to the oracle component fails
pub fn get_price(
    price_feed: Global<AnyComponent>,
    res_address: ResourceAddress,
) -> Result<PriceInfo, String> {
    // Bypass price feed for XRD, return 1 as XRD is the base asset
    if res_address == XRD {
        return Ok(PriceInfo {
            timestamp: Clock::current_time(TimePrecision::Second).seconds_since_unix_epoch,
            price: dec!(1),
        });
    }

    match price_feed.call_raw::<Option<PriceInfo>>("get_price", scrypto_args!(res_address)) {
        Some(price_info) => Ok(price_info),
        None => Err("Price not found".to_string()),
    }
}
