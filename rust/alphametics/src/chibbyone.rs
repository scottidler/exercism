use std::collections::{HashMap, HashSet};

fn insert_char_if_not_seen(s: &str, set: &mut HashSet<char>) {
    for c in s.chars() {
        set.insert(c);
    }
}
fn get_number_representation(s: &str, hashmap: &HashMap<char, u8>) -> u64 {
    let mut number: u64 = 0;
    let chars = s.chars();
    let mut pos = s.len();
    for c in chars {
        let val = *hashmap.get(&c).unwrap() as u64;
        number += val * 10_u64.pow(pos as u32);
        pos -= 1;
    }
    number
}
fn convert_to_numbers_and_check_result(input: &[&str], result: &str, hashmap: &HashMap<char, u8>) -> bool {
    // Convert inputs to number
    let val: u64 = input.iter().map(|s| get_number_representation(s, hashmap)).sum();
    // Convert result to number
    let result_as_number = get_number_representation(result, hashmap);
    val == result_as_number
}
fn is_valid(map: &HashMap<char, u8>, inputs: &Vec<&str>, result: &str) -> bool {
    for input in inputs {
        if *map.get(&input.chars().next().unwrap()).unwrap() == 0 {
            return false;
        }
    }
    if *map.get(&result.chars().next().unwrap()).unwrap() == 0 {
        return false;
    }
    true
}
#[derive(Debug)]
struct Permutation {
    letters: Vec<char>,
    count: usize,
    max: usize,
    current_values: Vec<u8>,
}
impl Permutation {
    fn new(s: &HashSet<char>) -> Self {
        fn combinations(num: usize) -> usize {
            let start = 10 - num + 1;
            let mut result = 1;
            for v in start..=10 {
                result *= v;
            }
            result
        }
        Self {
            letters: s.iter().copied().collect(),
            count: 0,
            max: combinations(s.len()),
            current_values: (0..s.len()).rev().map(|x| x as u8).collect(),
        }
    }
    fn find_next_combination(&mut self, index: usize) {
        if index >= self.current_values.len() {
            return;
        }
        let mut next_digit = (self.current_values[index] + 1) % 10;
        if next_digit == 0 {
            self.find_next_combination(index + 1);
        }
        while self.current_values[index + 1..].contains(&next_digit) {
            next_digit = (next_digit + 1) % 10;
            if next_digit == 0 {
                self.find_next_combination(index + 1);
            }
        }
        self.current_values[index] = next_digit;
    }
    fn next_hashmap(&mut self) -> HashMap<char, u8> {
        self.find_next_combination(0);
        self.letters
            .iter()
            .copied()
            .zip(self.current_values.iter().copied())
            .collect()
    }
}
impl Iterator for Permutation {
    type Item = HashMap<char, u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max {
            None
        } else if self.count == 0 {
            self.count += 1;
            Some(
                self.letters
                    .iter()
                    .copied()
                    .zip(self.current_values.iter().copied())
                    .collect(),
            )
        } else {
            self.count += 1;
            Some(self.next_hashmap())
        }
    }
}
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // The HashMap must be mutable
    let mut set: HashSet<char> = HashSet::new();
    // Get all the unique letters to create the hashmap
    let input_and_result: Vec<&str> = input.split("==").collect();
    // If any of the input or result is missing then there can be no solution and therefore we
    // return None
    // The input is split into a vector of &str since there could be > 1 inputs
    let input: Vec<&str> = match input_and_result.first() {
        Some(v) => v.split('+').map(|v| v.trim()).collect(),
        None => {
            return None;
        }
    };
    // The result is handled as a &str since it will only be 1 result
    let result = match input_and_result.last() {
        Some(v) => v.trim(),
        None => {
            return None;
        }
    };
    // There can be at most 10 entries to the hashmap since there can only be ten kinds of digits.
    insert_char_if_not_seen(result, &mut set);
    for s in &input {
        insert_char_if_not_seen(s, &mut set);
    }
    // We iterate over each kind of
    let mut perm = Permutation::new(&set);
    perm.find(|hashmap| {
        is_valid(hashmap, &input, result) && convert_to_numbers_and_check_result(&input, result, hashmap)
    })
}
