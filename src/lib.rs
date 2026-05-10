//! indian-numbers
//!
//! A small crate to format numbers in Indian style and convert them to words.

/// Formats a number using Indian numbering system (Lakh, Crore)
/// Example: 1234567 → "12,34,567"
pub fn format(n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let is_negative = n < 0;
    let mut num = n.abs() as u64;
    let mut result = String::new();
    let mut count = 0;

    while num > 0 {
        if count == 3 || (count > 3 && (count - 3) % 2 == 0) {
            result.push(',');
        }
        result.push_str(&(num % 10).to_string());
        num /= 10;
        count += 1;
    }

    let mut formatted = result.chars().rev().collect::<String>();
    if is_negative {
        formatted.insert(0, '-');
    }
    formatted
}

/// Converts number to Indian words
/// Example: 1234567 → "Twelve Lakh Thirty Four Thousand Five Hundred Sixty Seven"
pub fn to_words(n: i64) -> String {
    if n == 0 {
        return "Zero".to_string();
    }

    let is_negative = n < 0;
    let abs_n = n.abs();

    let words = number_to_words(abs_n);

    if is_negative {
        format!("Minus {}", words)
    } else {
        words
    }
}

fn number_to_words(n: i64) -> String {
    const ONES: [&str; 20] = [
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen",
        "Seventeen", "Eighteen", "Nineteen",
    ];

    const TENS: [&str; 10] = ["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];

    if n < 20 {
        return ONES[n as usize].to_string();
    }

    if n < 100 {
        let ten = TENS[(n / 10) as usize];
        let one = if n % 10 != 0 {
            format!(" {}", ONES[(n % 10) as usize])
        } else {
            "".to_string()
        };
        return format!("{}{}", ten, one);
    }

    if n >= 10_000_000 {
        let crore = n / 10_000_000;
        let rem = n % 10_000_000;
        let rem_str = if rem != 0 { format!(" {}", number_to_words(rem)) } else { "".to_string() };
        return format!("{} Crore{}", number_to_words(crore), rem_str);
    }
    if n >= 100_000 {
        let lakh = n / 100_000;
        let rem = n % 100_000;
        let rem_str = if rem != 0 { format!(" {}", number_to_words(rem)) } else { "".to_string() };
        return format!("{} Lakh{}", number_to_words(lakh), rem_str);
    }
    if n >= 1_000 {
        let thousand = n / 1_000;
        let rem = n % 1_000;
        let rem_str = if rem != 0 { format!(" {}", number_to_words(rem)) } else { "".to_string() };
        return format!("{} Thousand{}", number_to_words(thousand), rem_str);
    }
    if n >= 100 {
        let hundred = n / 100;
        let rem = n % 100;
        let rem_str = if rem != 0 { format!(" {}", number_to_words(rem)) } else { "".to_string() };
        return format!("{} Hundred{}", number_to_words(hundred), rem_str);
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(format(1234567), "12,34,567");
        assert_eq!(format(1234567890), "1,23,45,67,890");
        assert_eq!(format(100000), "1,00,000");
        assert_eq!(format(-98765), "-98,765");
        assert_eq!(format(0), "0");
    }

    #[test]
    fn test_to_words() {
        assert_eq!(to_words(1234567), "Twelve Lakh Thirty Four Thousand Five Hundred Sixty Seven");
        assert_eq!(to_words(100000), "One Lakh");
        assert_eq!(to_words(123456789), "Twelve Crore Thirty Four Lakh Fifty Six Thousand Seven Hundred Eighty Nine");
        assert_eq!(to_words(0), "Zero");
        assert_eq!(to_words(-500), "Minus Five Hundred");
    }
}