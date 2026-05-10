# indian-numbers

A lightweight, zero-dependency Rust library for formatting numbers in the **Indian numbering system** (Lakh & Crore), converting them to words, and handling Rupee currency formatting.

## Features

- Indian-style comma formatting (`12,34,56,789`)
- Number to English words with proper "and"
- `to_rupees()` for clean invoice-ready output
- Full support for negative numbers
- Zero dependencies
- Fast and minimal

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
indian-numbers = "0.2"
```

## Usage

```rust
use indian_numbers::{format, to_words, to_rupees};

fn main() {
    let amount = 123456789;

    println!("Formatted : {}", format(amount));
    println!("In Words  : {}", to_words(amount));
    println!("Rupees    : {}", to_rupees(amount));
}
```

## API

| Function | Description | Example Output |
|---|---|---|
| `format(n: i64)` | Indian comma formatting | `"12,34,56,789"` |
| `to_words(n: i64)` | Convert number to words | `"Twelve Crore Thirty Four Lakh..."` |
| `to_rupees(n: i64)` | Invoice-ready Rupee formatting | `"₹12,34,56,789 only"` |

## Examples

```rust
let n = 1234567;

format(n);      // "12,34,567"
to_words(n);    // "Twelve Lakh Thirty Four Thousand Five Hundred and Sixty Seven"
to_rupees(n);   // "₹12,34,567 only"
```

## Example Table

| Number | Format | Words | Rupees |
|---|---|---|---|
| 1234567 | 12,34,567 | Twelve Lakh Thirty Four Thousand Five Hundred and Sixty Seven | ₹12,34,567 only |
| 123456789 | 12,34,56,789 | Twelve Crore Thirty Four Lakh Fifty Six Thousand Seven Hundred and Eighty Nine | ₹12,34,56,789 only |
| 100000 | 1,00,000 | One Lakh | ₹1,00,000 only |
| 123 | 123 | One Hundred and Twenty Three | ₹123 only |
| -25000 | -25,000 | Minus Twenty Five Thousand | ₹-25,000 only |
| 0 | 0 | Zero | ₹0 only |

## License

Licensed under either of:

- MIT License
- Apache License 2.0

at your option.

---

Made for developers working with Indian number formatting in Rust.