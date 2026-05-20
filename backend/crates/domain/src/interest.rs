use chrono::Datelike;

use crate::projection::project_payoff;
use crate::types::LoanCalcInput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterestSummary {
    pub interest_paid_minor: i64,
    pub interest_remaining_minor: i64,
    pub computable: bool,
    pub message: Option<String>,
}

pub fn compute_interest_summary(loan: &LoanCalcInput, as_of: chrono::NaiveDate) -> InterestSummary {
    let paid: i64 = loan
        .payments
        .iter()
        .map(|p| p.interest_portion_minor)
        .sum();

    if loan.apr_basis_points.is_none() {
        return InterestSummary {
            interest_paid_minor: paid,
            interest_remaining_minor: 0,
            computable: false,
            message: Some("missing_apr".into()),
        };
    }

    let remaining = estimate_remaining_interest(loan, as_of, 0);
    InterestSummary {
        interest_paid_minor: paid,
        interest_remaining_minor: remaining,
        computable: true,
        message: None,
    }
}

fn estimate_remaining_interest(
    loan: &LoanCalcInput,
    as_of: chrono::NaiveDate,
    _periodic: i64,
) -> i64 {
    let projection = project_payoff(loan, as_of);
    let r = loan
        .apr_basis_points
        .map(|a| crate::amortization::periodic_rate(a, loan.payment_frequency))
        .unwrap_or(0.0);
    let mut balance = loan.remaining_balance_minor as f64 / 100.0;
    let mut total_interest = 0.0;
    let periodic = crate::amortization::effective_periodic_payment(
        loan.remaining_balance_minor,
        loan.apr_basis_points,
        loan.fixed_payment_minor,
        loan.payment_frequency,
    ) as f64
        / 100.0;
    let mut date = as_of;
    for _ in 0..600 {
        if balance <= 0.0 {
            break;
        }
        let interest = balance * r;
        total_interest += interest;
        let payment = periodic.min(balance + interest);
        let principal = (payment - interest).max(0.0);
        balance -= principal;
        date = advance_date(date, loan.payment_frequency);
        if projection
            .projected_payoff_date
            .map(|p| date > p)
            .unwrap_or(false)
        {
            break;
        }
    }
    (total_interest * 100.0).round() as i64
}

fn advance_date(date: chrono::NaiveDate, freq: crate::types::PaymentFrequency) -> chrono::NaiveDate {
    match freq {
        crate::types::PaymentFrequency::Monthly => {
            if date.month() == 12 {
                chrono::NaiveDate::from_ymd_opt(date.year() + 1, 1, date.day()).unwrap_or(date)
            } else {
                chrono::NaiveDate::from_ymd_opt(date.year(), date.month() + 1, date.day())
                    .unwrap_or(date)
            }
        }
        crate::types::PaymentFrequency::Yearly => {
            chrono::NaiveDate::from_ymd_opt(date.year() + 1, date.month(), date.day()).unwrap_or(date)
        }
    }
}
