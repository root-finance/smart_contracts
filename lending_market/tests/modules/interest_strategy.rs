use lending_market::modules::interest_strategy::*;
use scrypto_test::prelude::*;

#[test]
fn test_interest_strategy_check() {
    let breakpoints_with_errors_1 = InterestStrategyBreakPoints{ 
        r0: dec!(0.05),
        r1: dec!(5),
        r2: dec!(3),
    };
    let breakpoints_with_errors_2 = InterestStrategyBreakPoints{ 
        r0: dec!(0.05),
        r1: dec!(3),
        r2: dec!(-5),
    };
    let breakpoints_with_errors_3 = InterestStrategyBreakPoints{ 
        r0: dec!(-0.05),
        r1: dec!(3),
        r2: dec!(5),
    };
        
    let breakpoints_with_errors_4 = InterestStrategyBreakPoints{ 
        r0: dec!(0.05),
        r1: dec!(3),
        r2: dec!(3),
    };
    assert!(InterestStrategy::new()
        .set_breakpoints(breakpoints_with_errors_1)
        .is_err());

    assert!(InterestStrategy::new()
        .set_breakpoints(breakpoints_with_errors_2)
        .is_err());

    assert!(InterestStrategy::new()
        .set_breakpoints(breakpoints_with_errors_3)
        .is_err());

    assert!(InterestStrategy::new()
        .set_breakpoints(breakpoints_with_errors_4)
        .is_err());
}

#[test]
fn test_interest_strategy_check_get_rate() {
    let mut interest_strategy = InterestStrategy::new();

    let breakpoints = InterestStrategyBreakPoints {
        r0: dec!(0),
        r1: dec!(0.04),
        r2: dec!(0.75),
    };

    let result = interest_strategy.set_breakpoints(breakpoints);

    assert!(result.is_ok());

    assert!(interest_strategy.get_interest_rate(dec!(-0.1), dec!(0.5)).is_err());

    assert!(interest_strategy.get_interest_rate(dec!(1.1), dec!(0.9)).is_err());
}

#[test]
fn test_interest_strategy_get_rate_value() {
    let mut interest_strategy = InterestStrategy::new();

    let breakpoints = InterestStrategyBreakPoints {
        r0: dec!(0.01),
        r1: dec!(0.04),
        r2: dec!(0.75),
    };

    let result = interest_strategy.set_breakpoints(breakpoints);

    assert!(result.is_ok());

    assert_eq!(
        interest_strategy.get_interest_rate(dec!(0.6), dec!(0.8)),
        Ok(dec!(0.04))
    );
    assert_eq!(
        interest_strategy.get_interest_rate(dec!(0.9), dec!(0.8)),
        Ok(dec!(0.425))
    );
    assert_eq!(
        interest_strategy.get_interest_rate(dec!(0), dec!(0.8)),
        Ok(dec!(0.01))
    );
    assert_eq!(interest_strategy.get_interest_rate(dec!(1), dec!(0.8)), Ok(dec!(0.8)));

    assert!(interest_strategy.get_interest_rate(dec!(-0.1), dec!(0.500)).is_err());

    assert!(interest_strategy.get_interest_rate(dec!(1.1), dec!(0.500)).is_err());
}
