use crate::types::PaymentFrequency;

/// Periodic interest rate from annual APR basis points.
pub fn periodic_rate(apr_basis_points: i32, frequency: PaymentFrequency) -> f64 {
    let annual = apr_basis_points as f64 / 10_000.0;
    match frequency {
        PaymentFrequency::Monthly => annual / 12.0,
        PaymentFrequency::Yearly => annual,
    }
}

/// Derive periodic payment from balance, APR, and implicit term (months).
/// Uses standard annuity formula; default term 240 periods (20y monthly / 20y yearly).
pub fn derive_periodic_payment(
    balance_minor: i64,
    apr_basis_points: i32,
    frequency: PaymentFrequency,
) -> i64 {
    let balance = balance_minor as f64 / 100.0;
    let r = periodic_rate(apr_basis_points, frequency);
    if r <= 0.0 {
        let periods = default_periods(frequency);
        return (balance / periods as f64 * 100.0).round() as i64;
    }
    let n = default_periods(frequency) as f64;
    let payment = if r > 0.0 {
        balance * (r * (1.0 + r).powf(n)) / ((1.0 + r).powf(n) - 1.0)
    } else {
        balance / n
    };
    (payment * 100.0).round() as i64
}

fn default_periods(frequency: PaymentFrequency) -> u32 {
    match frequency {
        PaymentFrequency::Monthly => 240,
        PaymentFrequency::Yearly => 20,
    }
}

pub fn effective_periodic_payment(
    balance_minor: i64,
    apr_basis_points: Option<i32>,
    fixed_payment_minor: Option<i64>,
    frequency: PaymentFrequency,
) -> i64 {
    if let Some(fixed) = fixed_payment_minor {
        return fixed;
    }
    if let Some(apr) = apr_basis_points {
        return derive_periodic_payment(balance_minor, apr, frequency);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apr_payment_positive() {
        let p = derive_periodic_payment(200_000_00, 375, PaymentFrequency::Monthly);
        assert!(p > 0);
    }
}
