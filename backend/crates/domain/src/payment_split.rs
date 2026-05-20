use crate::amortization::periodic_rate;
use crate::types::LoanCalcInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentSplit {
    pub interest_portion_minor: i64,
    pub principal_portion_minor: i64,
    pub balance_after_minor: i64,
}

pub fn split_payment(
    loan: &LoanCalcInput,
    amount_minor: i64,
    current_balance_minor: i64,
) -> PaymentSplit {
    let r = loan
        .apr_basis_points
        .map(|a| periodic_rate(a, loan.payment_frequency))
        .unwrap_or(0.0);
    let balance = current_balance_minor as f64 / 100.0;
    let amount = amount_minor as f64 / 100.0;
    let interest = (balance * r).min(amount);
    let principal = (amount - interest).max(0.0);
    let balance_after = ((balance - principal) * 100.0).max(0.0).round() as i64;
    PaymentSplit {
        interest_portion_minor: (interest * 100.0).round() as i64,
        principal_portion_minor: (principal * 100.0).round() as i64,
        balance_after_minor: balance_after,
    }
}
