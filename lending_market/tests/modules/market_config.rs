use lending_market::modules::market_config::*;
use scrypto_test::prelude::*;

#[test]
fn test_check_valid_max_cdp_position() {
    let market_config = MarketConfig {
        max_cdp_position: 10,
        max_liquidable_value: dec!(0.4),
        liquidation_dex_swap_rate: dec!(1)
    };

    assert_eq!(market_config.check(), Ok(()));
}

#[test]
fn test_check_invalid_max_cdp_position() {
    let market_config = MarketConfig {
        max_cdp_position: 0,
        max_liquidable_value: dec!(0.4),
        liquidation_dex_swap_rate: dec!(1)
    };

    assert_eq!(
        market_config.check(),
        Err("Max CDP position must be greater than 0".into())
    );
}

#[test]
fn test_update_max_cdp_position_valid() {
    let mut market_config = MarketConfig {
        max_cdp_position: 10,
        max_liquidable_value: dec!(0.4),
        liquidation_dex_swap_rate: dec!(1)
    };

    assert_eq!(
        market_config.update(UpdateMarketConfigInput::MaxCDPPosition(20)),
        Ok(())
    );

    assert_eq!(market_config.max_cdp_position, 20);
}

#[test]
fn test_update_max_cdp_position_invalid() {
    let mut market_config = MarketConfig {
        max_cdp_position: 10,
        max_liquidable_value: dec!(0.4),
        liquidation_dex_swap_rate: dec!(1)
    };

    assert_eq!(
        market_config.update(UpdateMarketConfigInput::MaxCDPPosition(0)),
        Err("Max CDP position must be greater than 0".into())
    );
}


#[test]
fn test_check_valid_max_liquidable_value() {
    let market_config = MarketConfig {
        max_cdp_position: 10,
        max_liquidable_value: dec!(0.4),
        liquidation_dex_swap_rate: dec!(1)
    };

    assert_eq!(market_config.check(), Ok(()));
}

#[test]
fn test_check_invalid_max_liquidable_value() {
    let market_config = MarketConfig {
        max_cdp_position: 10,
        max_liquidable_value: dec!(100),
        liquidation_dex_swap_rate: dec!(1)
    };

    assert_eq!(
        market_config.check(),
        Err("Max liquidable value must be in range 0..1".into())
    );
}

#[test]
fn test_update_max_liquidable_value() {
    let mut market_config = MarketConfig {
        max_cdp_position: 10,
        max_liquidable_value: dec!(0.4),
        liquidation_dex_swap_rate: dec!(1)
    };

    assert_eq!(
        market_config.update(UpdateMarketConfigInput::MaxLiquidableValue(dec!(0.3))),
        Ok(())
    );

    assert_eq!(market_config.max_liquidable_value, dec!(0.3));
}

#[test]
fn test_update_max_liquidable_value_invalid() {
    let mut market_config = MarketConfig {
        max_cdp_position: 10,
        max_liquidable_value: dec!(0.4),
        liquidation_dex_swap_rate: dec!(1)
    };

    assert_eq!(
        market_config.update(UpdateMarketConfigInput::MaxLiquidableValue(dec!(-0.3))),
        Err("Max liquidable value must be in range 0..1".into())
    );
}
