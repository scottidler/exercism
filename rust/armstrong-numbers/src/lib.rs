
// fn count_digits(num: u32) -> u32 {
//     if num == 0 {
//         return 1;
//     }
//     let mut count = 0;
//     let mut num = num;
//     while num > 0 {
//         num /= 10;
//         count += 1;
//     }
//     count
// }

// return list of digits when given a number
fn get_digits(num: u32) -> Vec<u64> {
    let mut digits = vec![];
    let mut num: u64 = num as u64;
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits
}

fn addition_of_powers_of_digits(num: u32) -> u64 {
    let digits = get_digits(num);
    digits.iter().map(|x| x.pow((digits.len() as u64).try_into().unwrap())).sum()
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_digits(num);
    if num == 0 {
        true
    } else if u64::from(num) == addition_of_powers_of_digits(num) {
        true
    } else {
        false
    }
}
