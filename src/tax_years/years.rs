/// This struct contains the rates and thresholds for Income Tax and National Insurance for a single tax year in the United Kingdom.
pub struct TaxYear {
    /// Annual personal allowance (£).
    pub personal_allowance: u32,
    /// Upper limit for basic rate (£).
    pub higher_rate_threshold: u32,
    /// Threshold for additional rate (£).
    pub additional_rate_threshold: u32,
    /// Additional rate (e.g., 0.45 for 45%).
    pub additional_rate: f64,
    /// Basic rate (e.g., 0.20 for 20%).
    pub basic_rate: f64,
    /// Higher rate (e.g., 0.40 for 40%).
    pub higher_rate: f64,
    /// National Insurance primary threshold (£).
    pub ni_primary_threshold: u32,
    /// National Insurance upper earnings limit (£).
    pub ni_upper_earnings_limit: u32,
    /// National Insurance primary rate (e.g., 0.12 for 12%).
    pub ni_primary_rate: f64,
    /// National Insurance upper rate (e.g., 0.02 for 2%).
    pub ni_upper_rate: f64,
}

/// Tax year 2025/26.
pub const TAX_YEAR_2025: TaxYear = TaxYear {
    personal_allowance: 12_570,
    higher_rate_threshold: 50_270,
    additional_rate_threshold: 125_140,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 12_570,
    ni_upper_earnings_limit: 50_270,
    ni_primary_rate: 0.08,
    ni_upper_rate: 0.02,
};

/// Tax year 2024/25.
pub const TAX_YEAR_2024: TaxYear = TaxYear {
    personal_allowance: 12_570,
    higher_rate_threshold: 50_270,
    additional_rate_threshold: 125_140,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 12_570,
    ni_upper_earnings_limit: 50_270,
    ni_primary_rate: 0.08,
    ni_upper_rate: 0.02,
};

/// Tax year 2023/24.
pub const TAX_YEAR_2023: TaxYear = TaxYear {
    personal_allowance: 12_570,
    higher_rate_threshold: 50_270,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 11_908,
    ni_upper_earnings_limit: 50_270,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2022/23.
pub const TAX_YEAR_2022: TaxYear = TaxYear {
    personal_allowance: 12_570,
    higher_rate_threshold: 50_270,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 9_568,
    ni_upper_earnings_limit: 50_270,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2021/22.
pub const TAX_YEAR_2021: TaxYear = TaxYear {
    personal_allowance: 12_500,
    higher_rate_threshold: 50_000,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 9_500,
    ni_upper_earnings_limit: 50_000,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2020/21.
pub const TAX_YEAR_2020: TaxYear = TaxYear {
    personal_allowance: 12_500,
    higher_rate_threshold: 50_000,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 8_632,
    ni_upper_earnings_limit: 50_000,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2019/20.
pub const TAX_YEAR_2019: TaxYear = TaxYear {
    personal_allowance: 11_850,
    higher_rate_threshold: 46_350,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 8_424,
    ni_upper_earnings_limit: 46_350,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2018/19.
pub const TAX_YEAR_2018: TaxYear = TaxYear {
    personal_allowance: 11_500,
    higher_rate_threshold: 45_000,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 8_164,
    ni_upper_earnings_limit: 45_000,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2017/18.
pub const TAX_YEAR_2017: TaxYear = TaxYear {
    personal_allowance: 11_000,
    higher_rate_threshold: 43_000,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 8_060,
    ni_upper_earnings_limit: 43_000,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2016/17.
pub const TAX_YEAR_2016: TaxYear = TaxYear {
    personal_allowance: 10_600,
    higher_rate_threshold: 42_385,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 8_060,
    ni_upper_earnings_limit: 42_385,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2015/16.
pub const TAX_YEAR_2015: TaxYear = TaxYear {
    personal_allowance: 10_000,
    higher_rate_threshold: 41_865,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 7_956,
    ni_upper_earnings_limit: 41_865,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2014/15.
pub const TAX_YEAR_2014: TaxYear = TaxYear {
    personal_allowance: 9_440,
    higher_rate_threshold: 41_450,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.45,
    ni_primary_threshold: 7_755,
    ni_upper_earnings_limit: 41_450,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2013/14.
pub const TAX_YEAR_2013: TaxYear = TaxYear {
    personal_allowance: 8_105,
    higher_rate_threshold: 42_475,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.50,
    ni_primary_threshold: 7_605,
    ni_upper_earnings_limit: 42_475,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2012/13.
pub const TAX_YEAR_2012: TaxYear = TaxYear {
    personal_allowance: 7_475,
    higher_rate_threshold: 42_475,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.50,
    ni_primary_threshold: 7_225,
    ni_upper_earnings_limit: 42_475,
    ni_primary_rate: 0.12,
    ni_upper_rate: 0.02,
};

/// Tax year 2011/12.
pub const TAX_YEAR_2011: TaxYear = TaxYear {
    personal_allowance: 6_475,
    higher_rate_threshold: 43_875,
    additional_rate_threshold: 150_000,
    basic_rate: 0.20,
    higher_rate: 0.40,
    additional_rate: 0.50,
    ni_primary_threshold: 5_715,
    ni_upper_earnings_limit: 43_875,
    ni_primary_rate: 0.11,
    ni_upper_rate: 0.01,
};
