use super::utils::is_valid_rate;
use scrypto::prelude::*;

/// Slopes of the interest strategy
#[derive(ScryptoSbor, Clone, Debug, Default)]
pub struct InterestStrategyBreakPoints {
    pub r0: Decimal,
    pub r1: Decimal,
    pub r2: Decimal,
}

/// Linear interest strategy allowing several slopes
#[derive(ScryptoSbor, Default, Clone)]
pub struct InterestStrategy {
    break_points: InterestStrategyBreakPoints,
}

impl InterestStrategy {
    /// Constructor
    /// 
    /// *Output*
    /// A new `InterestStrategy` 
    pub fn new() -> Self {
        Self {
            break_points: InterestStrategyBreakPoints::default(),
        }
    }

    /// Setter of the interest strategy breakpoints
    /// 
    /// *Params*
    /// - `interest_strategy_break_points``: The interest strategy breakpoints to set
    /// 
    /// *Error*
    /// - If update of the internal state fails
    pub fn set_breakpoints(
        &mut self,
        interest_strategy_break_points: InterestStrategyBreakPoints,
    ) -> Result<(), String> {
        let input_break_points = vec![interest_strategy_break_points.r0, interest_strategy_break_points.r1, interest_strategy_break_points.r2];
        if input_break_points.is_empty() {
            return Err("The break points vector must contain at least one element".into());
        }

        if input_break_points[0] < dec!(0) {
            return Err("The initial rate must be greater than or equal to 0".into());
        }

        for i in 1..input_break_points.len() {
            if input_break_points[i - 1] >= input_break_points[i] {
                return Err("Slope must be monotonically increasing".into());
            }

            if input_break_points[i] < dec!(0) {
                return Err("Slope must be greater than or equal to 0".into());
            }
        }

        self.break_points = interest_strategy_break_points;

        Ok(())
    }

    /// Getter of the interest rate
    /// 
    /// *Params*
    ///  - `usage`: The pool usage
    ///  - `optimal_usage`: The optimal pool usage
    /// 
    /// *Errors*
    ///  - If parameters are invalid
    pub fn get_interest_rate(&self, usage: Decimal, optimal_usage: Decimal) -> Result<Decimal, String> {
        if !is_valid_rate(usage) {
            return Err(format!("Usage must be between 0 and 1, inclusive, was {usage}"));
        }
        if optimal_usage > dec!(0) && !is_valid_rate(optimal_usage) {
            return Err("Usage must be between 0 exclusive and 1 inclusive".into());
        }

        if usage < optimal_usage {
            return Ok(self.break_points.r0 + usage / optimal_usage * self.break_points.r1);
        } else {
            return Ok(self.break_points.r0 + self.break_points.r1 + (usage - optimal_usage) / (1 - optimal_usage) * self.break_points.r2);
        }
    }
}
