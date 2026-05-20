use crate::types::{PaymentFrequency, PaymentType};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ValidationError {
    #[error("loan label is required")]
    MissingLabel,
    #[error("remaining balance must be positive")]
    NonPositiveBalance,
    #[error("fixed payment must be provided for fixed-payment loans")]
    NoPaymentMethod,
    #[error("interest rate (APR) is required")]
    MissingApr,
    #[error("fixed payment must be positive")]
    InvalidFixedPayment,
    #[error("APR cannot be negative")]
    InvalidApr,
}

#[derive(Debug, Clone)]
pub struct CreateLoanValidation {
    pub label: String,
    pub remaining_balance_minor: i64,
    pub payment_frequency: PaymentFrequency,
    pub payment_type: PaymentType,
    pub fixed_payment_minor: Option<i64>,
    pub apr_basis_points: Option<i32>,
}

pub fn validate_create_loan(input: &CreateLoanValidation) -> Result<(), ValidationError> {
    if input.label.trim().is_empty() {
        return Err(ValidationError::MissingLabel);
    }
    if input.remaining_balance_minor <= 0 {
        return Err(ValidationError::NonPositiveBalance);
    }
    match input.apr_basis_points {
        None => return Err(ValidationError::MissingApr),
        Some(a) if a < 0 => return Err(ValidationError::InvalidApr),
        Some(_) => {}
    }
    if input.payment_type == PaymentType::Fixed {
        match input.fixed_payment_minor {
            None => return Err(ValidationError::NoPaymentMethod),
            Some(f) if f <= 0 => return Err(ValidationError::InvalidFixedPayment),
            Some(_) => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_input() -> CreateLoanValidation {
        CreateLoanValidation {
            label: "Test".into(),
            remaining_balance_minor: 100_000,
            payment_frequency: PaymentFrequency::Monthly,
            payment_type: PaymentType::Fixed,
            fixed_payment_minor: Some(500),
            apr_basis_points: Some(350),
        }
    }

    #[test]
    fn accepts_fixed_payment_with_apr() {
        assert!(validate_create_loan(&base_input()).is_ok());
    }

    #[test]
    fn rejects_missing_apr() {
        let mut input = base_input();
        input.apr_basis_points = None;
        assert_eq!(
            validate_create_loan(&input),
            Err(ValidationError::MissingApr)
        );
    }

    #[test]
    fn rejects_fixed_without_payment_amount() {
        let mut input = base_input();
        input.fixed_payment_minor = None;
        assert_eq!(
            validate_create_loan(&input),
            Err(ValidationError::NoPaymentMethod)
        );
    }
}
