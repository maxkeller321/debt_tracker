use chrono::{Datelike, Months, NaiveDate};

use crate::types::PaymentFrequency;

/// Calendar due dates for regular installments from `loan_start` through `through`.
/// Skips dates on or before `last_regular` (already recorded).
pub fn due_regular_payment_dates(
    loan_start: NaiveDate,
    frequency: PaymentFrequency,
    last_regular: Option<NaiveDate>,
    through: NaiveDate,
) -> Vec<NaiveDate> {
    let mut dates = Vec::new();
    let mut due = match last_regular {
        Some(last) => advance_due(last, frequency),
        None => loan_start,
    };
    while due <= through {
        dates.push(due);
        due = advance_due(due, frequency);
    }
    dates
}

pub fn advance_due(date: NaiveDate, frequency: PaymentFrequency) -> NaiveDate {
    match frequency {
        PaymentFrequency::Monthly => date
            .checked_add_months(Months::new(1))
            .unwrap_or_else(|| end_of_month(date.year(), date.month())),
        PaymentFrequency::Yearly => date
            .checked_add_months(Months::new(12))
            .unwrap_or(date),
    }
}

fn end_of_month(year: i32, month: u32) -> NaiveDate {
    let (y, m) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    NaiveDate::from_ymd_opt(y, m, 1)
        .unwrap()
        .pred_opt()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_monthly_dates_from_start() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let through = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let dates = due_regular_payment_dates(start, PaymentFrequency::Monthly, None, through);
        assert_eq!(dates.len(), 3);
        assert_eq!(dates[0], start);
        assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 2, 15).unwrap());
        assert_eq!(dates[2], NaiveDate::from_ymd_opt(2024, 3, 15).unwrap());
    }

    #[test]
    fn resumes_after_last_regular() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let last = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
        let through = NaiveDate::from_ymd_opt(2024, 4, 1).unwrap();
        let dates = due_regular_payment_dates(start, PaymentFrequency::Monthly, Some(last), through);
        assert_eq!(dates[0], NaiveDate::from_ymd_opt(2024, 3, 1).unwrap());
        assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 4, 1).unwrap());
    }
}
