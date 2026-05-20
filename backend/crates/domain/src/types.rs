use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentFrequency {
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentType {
    Fixed,
    Apr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoanStatus {
    Active,
    Archived,
}

impl LoanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoanStatus::Active => "active",
            LoanStatus::Archived => "archived",
        }
    }

}

impl std::str::FromStr for LoanStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(LoanStatus::Active),
            "archived" => Ok(LoanStatus::Archived),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecurringExtra {
    pub amount_minor: i64,
    pub month: u8,
    pub day: u8,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ScheduledExtra {
    pub amount_minor: i64,
    pub due_date: NaiveDate,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct PaymentRecord {
    pub paid_at: NaiveDate,
    pub amount_minor: i64,
    pub interest_portion_minor: i64,
    pub principal_portion_minor: i64,
    pub event_type: String,
}

/// Loan data needed for calculations (DB-agnostic).
#[derive(Debug, Clone)]
pub struct LoanCalcInput {
    pub id: String,
    pub label: String,
    pub status: LoanStatus,
    pub remaining_balance_minor: i64,
    pub original_principal_minor: Option<i64>,
    pub payment_frequency: PaymentFrequency,
    pub payment_type: PaymentType,
    pub fixed_payment_minor: Option<i64>,
    pub apr_basis_points: Option<i32>,
    pub loan_start_date: NaiveDate,
    pub recurring_extras: Vec<RecurringExtra>,
    pub scheduled_extras: Vec<ScheduledExtra>,
    pub payments: Vec<PaymentRecord>,
}
