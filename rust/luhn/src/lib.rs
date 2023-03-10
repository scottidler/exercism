/*
fn remove_whitespace(code: &str) -> String {
    code.chars().filter(|c| !c.is_whitespace()).collect()
}

fn to_digits(code: &str) -> Vec<u32> {
    code.chars().map(|c| c.to_digit(10).unwrap()).collect()
}
*/
fn digitize(code: &str) -> Option<Vec<u32>> {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect()
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if let Some(code) = digitize(code) {
        if code.len() > 1 {
            let mut sum = 0;
            for (i, digit) in code.iter().rev().enumerate() {
                if i % 2 == 1 {
                    sum += if digit * 2 > 9 { digit * 2 - 9 } else { digit * 2 };
                } else {
                    sum += digit;
                }
            }
            return sum % 10 == 0;
        }
    }
    false
}
