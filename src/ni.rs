use crate::tax_years::TaxYear;

/// Calculates Class 1 employee National Insurance (NI) contributions for a given gross annual income and tax year.
///
/// # Arguments
///
/// * `gross_income` - Gross annual income in pounds.
/// * `year` - Reference to a [`TaxYear`] constant (e.g., `&TAX_YEAR_2025`).
/// * `_personal_allowance` - Unused argument included for signature consistency with `income_tax`. You can pass `None`.
///
/// # Returns
///
/// The NI contributions due, in pounds, rounded to two decimal places.
///
/// # Example
///
/// ```rust
/// let ni = national_insurance(60_000.0, &TAX_YEAR_2025, None);
/// println!("National Insurance: £{:.2}", ni);
/// ```
///
/// [`TaxYear`]: struct.TaxYear.html

pub fn national_insurance(
    gross_income: f64,
    year: &TaxYear,
    _personal_allowance: Option<u32>,
) -> f64 {
    let gross_income_pence = (gross_income * 100.0).round() as u32;
    let p_thold = year.ni_primary_threshold * 100;
    let u_thold = year.ni_upper_earnings_limit * 100;
    let taxable_income = gross_income_pence.saturating_sub(p_thold);

    let mut tax = 0;

    if gross_income_pence <= u_thold {
        tax += (taxable_income as f64 * year.ni_primary_rate).round() as u32;
    } else {
        tax += ((u_thold - p_thold) as f64 * year.ni_primary_rate).round() as u32;
        tax += ((taxable_income - (u_thold - p_thold)) as f64 * year.ni_upper_rate).round() as u32;
    }

    (tax as f64) / 100.0
}
