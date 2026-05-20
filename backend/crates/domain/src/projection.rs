use chrono::{Datelike, NaiveDate};

use crate::amortization::{effective_periodic_payment, periodic_rate};
use crate::types::{LoanCalcInput, PaymentFrequency, PaymentType};

const MAX_PERIODS: u32 = 600;

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionResult {
    pub projected_payoff_date: Option<NaiveDate>,
    pub periodic_payment_minor: i64,
}

/// Forward-simulate from `as_of` until balance <= 0.
pub fn project_payoff(loan: &LoanCalcInput, as_of: NaiveDate) -> ProjectionResult {
    let mut balance = loan.remaining_balance_minor as f64;
    let periodic_payment = effective_periodic_payment(
        loan.remaining_balance_minor,
        loan.apr_basis_points,
        loan.fixed_payment_minor,
        loan.payment_frequency,
    );
    let r = match loan.payment_type {
        PaymentType::Apr => loan
            .apr_basis_points
            .map(|a| periodic_rate(a, loan.payment_frequency))
            .unwrap_or(0.0),
        PaymentType::Fixed => loan
            .apr_basis_points
            .map(|a| periodic_rate(a, loan.payment_frequency))
            .unwrap_or(0.0),
    };

    let mut date = as_of;
    let mut payoff: Option<NaiveDate> = None;

    for _ in 0..MAX_PERIODS {
        if balance <= 0.0 {
            payoff = Some(date);
            break;
        }

        let interest = balance * r;
        let mut payment = periodic_payment as f64 / 100.0;
        if payment > balance + interest {
            payment = balance + interest;
        }
        let principal = (payment - interest).max(0.0);
        balance -= principal;

        balance -= extras_for_date(loan, date) as f64 / 100.0;
        balance -= pending_scheduled_extras(loan, date) as f64 / 100.0;

        if balance <= 0.0 {
            payoff = Some(date);
            break;
        }

        date = advance_period(date, loan.payment_frequency);
    }

    ProjectionResult {
        projected_payoff_date: payoff,
        periodic_payment_minor: periodic_payment,
    }
}

fn advance_period(date: NaiveDate, freq: PaymentFrequency) -> NaiveDate {
    match freq {
        PaymentFrequency::Monthly => {
            let (y, m) = if date.month() == 12 {
                (date.year() + 1, 1)
            } else {
                (date.year(), date.month() + 1)
            };
            NaiveDate::from_ymd_opt(y, m, 1).unwrap_or(date)
        }
        PaymentFrequency::Yearly => NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap_or(date),
    }
}

fn extras_for_date(loan: &LoanCalcInput, date: NaiveDate) -> i64 {
    loan.recurring_extras
        .iter()
        .filter(|e| {
            e.enabled
                && u32::from(e.month) == date.month()
                && u32::from(e.day) == date.day()
        })
        .map(|e| e.amount_minor)
        .sum()
}

fn pending_scheduled_extras(loan: &LoanCalcInput, date: NaiveDate) -> i64 {
    loan.scheduled_extras
        .iter()
        .filter(|s| s.status == "pending" && s.due_date == date)
        .map(|s| s.amount_minor)
        .sum()
}

/// Monthly-normalized obligation for dashboard totals.
pub fn monthly_equivalent_payment(periodic_minor: i64, frequency: PaymentFrequency) -> i64 {
    match frequency {
        PaymentFrequency::Monthly => periodic_minor,
        PaymentFrequency::Yearly => (periodic_minor as f64 / 12.0).round() as i64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LoanStatus, PaymentType};

    fn sample_loan() -> LoanCalcInput {
        LoanCalcInput {
            id: "1".into(),
            label: "Test".into(),
            status: LoanStatus::Active,
            remaining_balance_minor: 100_000_00,
            original_principal_minor: Some(120_000_00),
            payment_frequency: PaymentFrequency::Monthly,
            payment_type: PaymentType::Fixed,
            fixed_payment_minor: Some(1_000_00),
            apr_basis_points: Some(300),
            loan_start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            recurring_extras: vec![],
            scheduled_extras: vec![],
            payments: vec![],
        }
    }

    #[test]
    fn yearly_normalization() {
        assert_eq!(
            monthly_equivalent_payment(12_000_00, PaymentFrequency::Yearly),
            1_000_00
        );
    }

    #[test]
    fn projection_computes_periodic_payment() {
        let loan = sample_loan();
        let result = project_payoff(&loan, NaiveDate::from_ymd_opt(2025, 5, 1).unwrap());
        assert!(result.periodic_payment_minor > 0);
    }
}
