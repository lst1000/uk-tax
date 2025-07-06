//! A Rust library that calculates UK PAYE Income Tax and Class 1 National Insurance contributions with historical tax year support.  
//!
//! Supports historical tax years (2011/12 to 2025/26) and allows custom personal allowances to be specified, if needed.

#![deny(missing_docs)]

/// Income Tax calculation function.
pub mod it;

/// National Insurance calculation function.
pub mod ni;

/// Definitions of tax years and thresholds.
pub mod tax_years;
