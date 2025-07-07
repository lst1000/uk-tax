use crate::tax_years::TaxYear;

/// Calculates income tax for a given gross income and tax year.
///
/// # Arguments
///
/// * `gross_income` - Gross annual income in pounds.
/// * `year` - Reference to a [`TaxYear`] constant (e.g., `&TAX_YEAR_2025`).
/// * `personal_allowance` - Optional custom personal allowance in pence. If `None`, defaults to the tax year’s standard allowance.
///
/// # Returns
///
/// * `Ok(tax_amount)` — The income tax due, in pounds, rounded to two decimal places.
/// * `Err(...)` — An error if a personal allowance is required (e.g., for incomes over £100,000) but not provided.
///
/// # Example
///
/// ```rust
/// let tax = income_tax(60_000.0, &TAX_YEAR_2025, None).unwrap();
/// println!("Income tax: £{:.2}", tax);
/// ```
///
/// [`TaxYear`]: struct.TaxYear.html

pub fn income_tax(
    gross_income: f64,
    year: &TaxYear,
    personal_allowance: Option<u32>,
) -> Result<f64, &str> {
    if gross_income > 100_000.0 && personal_allowance.is_none() {
        return Err("A personal allowance must be specified for incomes greater than £100,000.");
    }

    let p_allowance = personal_allowance.unwrap_or(year.personal_allowance) * 100;

    let gross_income_pence = (gross_income * 100.0).round() as u32;
    let allowance = p_allowance;
    let taxable_income = gross_income_pence.saturating_sub(allowance);

    let effective_allowance_pounds = p_allowance / 100;
    let basic_band_size = year.higher_rate_threshold - effective_allowance_pounds;
    let higher_band_size = year.additional_rate_threshold - year.higher_rate_threshold;

    let basic_band_pence = basic_band_size * 100;
    let higher_band_pence = higher_band_size * 100;

    let mut tax = 0;

    if taxable_income <= basic_band_pence {
        tax += (taxable_income as f64 * year.basic_rate).round() as u32;
    } else if taxable_income <= (basic_band_pence + higher_band_pence) {
        tax += (basic_band_pence as f64 * year.basic_rate).round() as u32;
        tax += ((taxable_income - basic_band_pence) as f64 * year.higher_rate).round() as u32;
    } else {
        tax += (basic_band_pence as f64 * year.basic_rate).round() as u32;
        tax += (higher_band_pence as f64 * year.higher_rate).round() as u32;
        tax += ((taxable_income - basic_band_pence - higher_band_pence) as f64 * year.additional_rate).round() as u32;
    }

    Ok((tax as f64) / 100.0)
}
