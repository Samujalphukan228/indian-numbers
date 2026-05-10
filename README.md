# indian-numbers

A lightweight, zero-dependency Rust crate for formatting numbers using the Indian numbering system (`Lakh`, `Crore`) and converting numbers into English words.

## Features

- Indian comma formatting  
  `123456789` → `12,34,56,789`
- Number-to-words conversion
- Support for negative numbers
- Zero dependencies
- Lightweight and fast

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
indian-numbers = "0.1"
```

Or install using Cargo:

```bash
cargo add indian-numbers
```

---

## Usage

```rust
use indian_numbers::{format, to_words};

fn main() {
    let number = 123456789;

    println!("{}", format(number));
    // 12,34,56,789

    println!("{}", to_words(number));
    // Twelve Crore Thirty Four Lakh Fifty Six Thousand Seven Hundred Eighty Nine
}
```

---

## Examples

| Number | Formatted | Words |
|---|---|---|
| `1234567` | `12,34,567` | `Twelve Lakh Thirty Four Thousand Five Hundred Sixty Seven` |
| `100000` | `1,00,000` | `One Lakh` |
| `123456789` | `12,34,56,789` | `Twelve Crore Thirty Four Lakh Fifty Six Thousand Seven Hundred Eighty Nine` |
| `5000000` | `50,00,000` | `Fifty Lakh` |
| `-25000` | `-25,000` | `Minus Twenty Five Thousand` |
| `0` | `0` | `Zero` |

---

## API

### `format`

Formats a number using the Indian numbering system.

```rust
use indian_numbers::format;

fn main() {
    assert_eq!(format(12345678), "1,23,45,678");
}
```

### `to_words`

Converts a number into English words.

```rust
use indian_numbers::to_words;

fn main() {
    assert_eq!(
        to_words(2500000),
        "Twenty Five Lakh"
    );
}
```

---

## Supported Units

| Unit | Value |
|---|---|
| Thousand | `1,000` |
| Lakh | `1,00,000` |
| Crore | `1,00,00,000` |

---

## License

Licensed under either of the following:

- MIT License
- Apache License 2.0

at your option.
