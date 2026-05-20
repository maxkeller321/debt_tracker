/// Format minor units as decimal string for API (e.g. 12345 -> "123.45")
pub fn minor_to_decimal(amount_minor: i64) -> String {
    let euros = amount_minor / 100;
    let cents = amount_minor.abs() % 100;
    if amount_minor < 0 {
        format!("-{}.{:02}", euros.abs(), cents)
    } else {
        format!("{}.{:02}", euros, cents)
    }
}

pub fn apr_basis_to_percent(basis_points: i32) -> f64 {
    basis_points as f64 / 100.0
}
