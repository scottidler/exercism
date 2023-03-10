pub fn encode(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        n if n > 20 && n < 30 => "twenty-".to_string() + &encode(n - 20),
        30 => "thirty".to_string(),
        n if n > 30 && n < 40 => "thirty-".to_string() + &encode(n - 30),
        40 => "forty".to_string(),
        n if n > 40 && n < 50 => "forty-".to_string() + &encode(n - 40),
        50 => "fifty".to_string(),
        n if n > 50 && n < 60 => "fifty-".to_string() + &encode(n - 50),
        60 => "sixty".to_string(),
        n if n > 60 && n < 70 => "sixty-".to_string() + &encode(n - 60),
        70 => "seventy".to_string(),    
        n if n > 70 && n < 80 => "seventy-".to_string() + &encode(n - 70),
        80 => "eighty".to_string(),
        n if n > 80 && n < 90 => "eighty-".to_string() + &encode(n - 80),
        90 => "ninety".to_string(),
        n if n > 90 && n < 100 => "ninety-".to_string() + &encode(n - 90),
        n if n > 99 && n < 1000 => {
            let hundreds = n / 100;
            let tens = n % 100;
            let mut result = encode(hundreds) + " hundred";
            if tens > 0 {
                result += " ";
                result += &encode(tens);
            }
            result
        },
        n if n > 999 && n < 1_000_000 => {
            let thousands = n / 1000;
            let hundreds = n % 1000;
            let mut result = encode(thousands) + " thousand";
            if hundreds > 0 {
                result += " ";
                result += &encode(hundreds);
            }
            result
        },
        n if n > 999_999 && n < 1_000_000_000 => {
            let millions = n / 1_000_000;
            let thousands = n % 1_000_000;
            let mut result = encode(millions) + " million";
            if thousands > 0 {
                result += " ";
                result += &encode(thousands);
            }
            result
        },
        n if n > 999_999_999 && n < 1_000_000_000_000 => {
            let billions = n / 1_000_000_000;
            let millions = n % 1_000_000_000;
            let mut result = encode(billions) + " billion";
            if millions > 0 {
                result += " ";
                result += &encode(millions);
            }
            result
        },
        n if n > 999_999_999_999 && n < 1_000_000_000_000_000 => {
            let trillions = n / 1_000_000_000_000;
            let billions = n % 1_000_000_000_000;
            let mut result = encode(trillions) + " trillion";
            if billions > 0 {
                result += " ";
                result += &encode(billions);
            }
            result
        },
        n if n > 999_999_999_999_999 && n < 1_000_000_000_000_000_000 => {
            let quadrillions = n / 1_000_000_000_000_000;
            let trillions = n % 1_000_000_000_000_000;
            let mut result = encode(quadrillions) + " quadrillion";
            if trillions > 0 {
                result += " ";
                result += &encode(trillions);
            }
            result
        },
        n if n > 999_999_999_999_999_999 && n <= u64::MAX => {
            let quintillions = n / 1_000_000_000_000_000_000;
            let quadrillions = n % 1_000_000_000_000_000_000;
            let mut result = encode(quintillions) + " quintillion";
            if quadrillions > 0 {
                result += " ";
                result += &encode(quadrillions);
            }
            result
        },
        _ => panic!("Number too large"),

    }
}
