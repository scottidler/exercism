use std::collections::{HashMap, HashSet};

type Num = u64;

fn is_valid(input: &str, mapping: &[Num]) -> bool {
    let mut current: Num = 0;
    let mut total: Num = 0;
    for byte in input.bytes() {
        match byte {
            b'+' | b'=' | b' ' => {
                total += current;
                current = 0;
            }
            b'A'..=b'Z' => {
                current *= 10;
                current += mapping[(byte - b'A') as usize] as Num;
            }
            _ => unreachable!(),
        }
    }
    current == total
}
fn attempt(
    input: &str,
    leading: &[u8],
    chars: &[u8],
    used: &mut [bool],
    mapping: &mut [Num],
) -> Option<HashMap<char, u8>> {
    if chars.is_empty() {
        if is_valid(input, &mapping) {
            let mut result = HashMap::new();
            for (offset, &value) in mapping.iter().enumerate() {
                if value < 10 {
                    result.insert((b'A' + offset as u8) as char, value as u8);
                }
            }
            return Some(result);
        } else {
            return None;
        }
    } else {
        let char = chars[0];
        for i in 0..10 {
            if used[i] || i == 0 && leading.contains(&char) {
                continue;
            }
            mapping[(char - b'A') as usize] = i as Num;
            used[i] = true;
            let result = attempt(input, leading, &chars[1..], used, mapping);
            if result != None {
                return result;
            }
            used[i] = false;
        }
        mapping[(char - b'A') as usize] = 10;
    }
    None
}
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let leading: Vec<u8> = input
        .split(|c| c == ' ' || c == '+' || c == '=')
        .filter_map(|w| w.bytes().nth(0))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let letters: Vec<u8> = input
        .bytes()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let mut used = [false; 10];
    let mut mapping = [10; 26];
    attempt(input, &leading, &letters, &mut used, &mut mapping)
}
