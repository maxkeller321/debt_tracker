use chrono::NaiveDate;
use domain::interest::compute_interest_summary;
use domain::types::{
    LoanCalcInput, LoanStatus, PaymentFrequency, PaymentRecord, PaymentType,
};

#[test]
fn sums_interest_paid_from_history() {
    let loan = LoanCalcInput {
        id: "1".into(),
        label: "Test".into(),
        status: LoanStatus::Active,
        remaining_balance_minor: 90_000_00,
        original_principal_minor: Some(100_000_00),
        payment_frequency: PaymentFrequency::Monthly,
        payment_type: PaymentType::Apr,
        fixed_payment_minor: None,
        apr_basis_points: Some(400),
        loan_start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        recurring_extras: vec![],
        scheduled_extras: vec![],
        payments: vec![PaymentRecord {
            paid_at: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            amount_minor: 1_000_00,
            interest_portion_minor: 300_00,
            principal_portion_minor: 700_00,
            event_type: "regular".into(),
        }],
    };
    let summary = compute_interest_summary(&loan, NaiveDate::from_ymd_opt(2025, 5, 1).unwrap());
    assert_eq!(summary.interest_paid_minor, 300_00);
    assert!(summary.computable);
}
