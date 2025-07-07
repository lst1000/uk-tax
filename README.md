# uk-tax

A Rust library that calculates UK PAYE Income Tax and Class 1 National Insurance contributions with historical tax year support.  

Supports historical tax years (2011/12 to 2025/26) and allows custom personal allowances to be specified, if needed.

## Features

- Calculate annual Income Tax (`income_tax`)
- Calculate annual Class 1 employee NI (`national_insurance`)
- Tax year constants from 2011/12 to 2025/26
- Flexible personal allowance support
- MIT licenced

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
uk-tax = "0.1.2"
```

## Usage

```rust
use uk_tax::{income_tax, national_insurance, TAX_YEAR_2025};

fn main() {
    let gross_income = 60_000.0;

    // Calculate income tax
    let tax = income_tax(gross_income, &TAX_YEAR_2025, None).unwrap();
    println!("Income Tax: £{:.2}", tax);

    // Calculate National Insurance
    let ni = national_insurance(gross_income, &TAX_YEAR_2025, None);
    println!("NI: £{:.2}", ni);
}
```

## Future developments

Planned or potential improvements:

- Automatic personal allowance tapering above £100,000 per annum
- Default to the current tax year automatically
- Add support for Scottish and Welsh income tax bands
- Add other NI classes (e.g., Class 2, Class 4)

## Licence

MIT Licence.  
See [LICENSE](LICENSE) for details.

## Contributing

Pull requests and issues are welcome!

## Author

Written by [Laurence Stock-Tully](https://github.com/lst1000)
