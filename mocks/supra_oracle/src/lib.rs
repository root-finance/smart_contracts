use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone)]
 pub struct PriceData {
    pub pair_index: u32,
    pub price: u128,
    pub timestamp: u64,
    pub round: u64,
    pub decimal: u16
}

impl PriceData {
    pub fn to_decimal_price(&self) -> Decimal {
        Decimal::from(self.price)/self.decimal
    }
}

// Define derive data structure to handle the data pairs that are derived using price updates recieved.
#[derive(ScryptoSbor, Debug)]
pub struct DerivedData {
    pub round_difference : i64,
    pub derived_price : u128,
    pub decimal : u16
}

#[blueprint]
mod price_feed {

    struct MockSupraOracle {}

    impl MockSupraOracle {
        pub fn instantiate() -> Global<MockSupraOracle> {
            Self {}
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }
        pub fn verify_proofs_and_get_data(&self, _data: Vec<u8>) -> Vec<PriceData> {
            vec![
                // XRD/USDT = 0.042
                PriceData { pair_index: 276, price: 42, timestamp: 1710168675, round: 0, decimal: 3 }
            ]
        }
        pub fn get_derived_svalue(&self, _pair_id_1: u32, _pair_id_2: u32, _operation: u32) -> DerivedData {
            DerivedData {
                decimal: 0,
                derived_price: 1,
                round_difference: 0
            }
        }
    }
}