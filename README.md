
```markdown
# indian-numbers

A small, zero-dependency Rust crate to format numbers in the **Indian numbering system** (Lakh, Crore) and convert them to words.

## Features
- Indian comma formatting: `123456789` → `12,34,56,789`
- Convert numbers to English words
- Supports negative numbers
- Zero dependencies
- Lightweight and fast

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
indian-numbers = "0.1"
```

## Usage

```rust
use indian_numbers::{format, to_words};

fn main() {
    let amount = 123456789;

    println!("Indian Format : {}", format(amount));
    // Output: 12,34,56,789

    println!("In Words      : {}", to_words(amount));
    // Output: Twelve Crore Thirty Four Lakh Fifty Six Thousand Seven Hundred Eighty Nine
}
```

## Examples

| Number     | Format          | Words                                              |
|------------|-----------------|----------------------------------------------------|
| 1234567    | 12,34,567       | Twelve Lakh Thirty Four Thousand Five Hundred Sixty Seven |
| 100000     | 1,00,000        | One Lakh                                           |
| 123456789  | 12,34,56,789    | Twelve Crore Thirty Four Lakh Fifty Six Thousand Seven Hundred Eighty Nine |
| 5000000    | 50,00,000       | Fifty Lakh                                         |
| -25000     | -25,000         | Minus Twenty Five Thousand                         |
| 0          | 0               | Zero                                               |

## API

- `format(n: i64) -> String` — Returns number in Indian comma format
- `to_words(n: i64) -> String` — Returns number in words

## License

MIT OR Apache-2.0

---

**Made with ❤️ for Indian developers**
```

Just copy the entire content above and paste it into your `README.md` file. It uses proper `#` and `##` headers with minimal gaps for clean editing.