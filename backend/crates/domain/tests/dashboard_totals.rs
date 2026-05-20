use chrono::NaiveDate;
use domain::projection::monthly_equivalent_payment;
use domain::types::{LoanCalcInput, LoanStatus, PaymentFrequency, PaymentType};

#[test]
fn household_monthly_normalizes_yearly() {
    assert_eq!(
        monthly_equivalent_payment(12_000_00, PaymentFrequency::Yearly),
        1_000_00
    );
}

#[test]
fn monthly_stays_unchanged() {
    assert_eq!(
        monthly_equivalent_payment(500_00, PaymentFrequency::Monthly),
        500_00
    );
}
