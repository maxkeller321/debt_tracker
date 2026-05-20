use domain::amortization::derive_periodic_payment;
use domain::types::PaymentFrequency;

#[test]
fn derived_payment_is_positive() {
    let p = derive_periodic_payment(150_000_00, 400, PaymentFrequency::Monthly);
    assert!(p > 0);
}
