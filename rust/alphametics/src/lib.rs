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
pub fn chibbyone_solve(input: &str) -> Option<HashMap<char, u8>> {
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

///////////////////////////
///////////////////////////
///////////////////////////

#[derive(Debug)]
pub struct Permutation2 {
    max: usize,
    count: usize,
    chars: Vec<char>,
    values: Vec<u8>,
    accepted: Vec<(char, u8)>,
}
impl Permutation2 {
    pub fn new(chars: &Vec<char>) -> Self {
        let accepted = vec![];
        let numbers = available_numbers(&accepted);
        let max = max_permutations(&chars, &numbers);
        let values = starting_values(&chars, &numbers);
        Self {
            max: max,
            count: 0,
            chars: chars.clone(),
            values: values,
            accepted: accepted,
        }
    }
    pub fn push(&self, chars: &Vec<char>, solution: &Solution) -> Self {
        let mut accepted = self.accepted.clone();
        accepted.extend(hashmap_to_sorted_vec_of_tuples(&solution));
        let numbers = available_numbers(&accepted);
        let max = max_permutations(&chars, &numbers);
        let values = starting_values(&chars, &numbers);
        Self {
            chars: chars.clone(),
            count: 0,
            max: max,
            values: values,
            accepted: accepted.clone(),
        }
    }
    fn build_hashmap(&self) -> HashMap<char, u8> {
        let hashmap = self.accepted.clone();
        hashmap
            .into_iter()
            .chain(self.chars.iter().copied().zip(self.values.iter().copied()))
            .collect()
    }
    fn find_next_combination(&mut self, index: usize) {
        if index >= self.values.len() {
            return;
        }
        let mut next_digit = (self.values[index] + 1) % 10;
        if next_digit == 0 {
            self.find_next_combination(index + 1);
        }
        while self.values[index + 1..].contains(&next_digit) {
            next_digit = (next_digit + 1) % 10;
            if next_digit == 0 {
                self.find_next_combination(index + 1);
            }
        }
        self.values[index] = next_digit;
    }
    fn next_hashmap(&mut self) -> HashMap<char, u8> {
        self.find_next_combination(0);
        self.build_hashmap()
    }
}
impl Iterator for Permutation2 {
    type Item = HashMap<char, u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max {
            None
        } else if self.count == 0 {
            self.count += 1;
            Some(self.build_hashmap())
        } else {
            self.count += 1;
            Some(self.next_hashmap())
        }
    }
}
use std::fmt;
impl fmt::Display for Permutation2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Permutation2(chars={:?}, count={}, max={}, values={:?}, accepted={:?})",
            self.chars, self.count, self.max, self.values, self.accepted
        )
    }
}

// const list of delimeters
const DELIMITERS: [char; 3] = ['+', '=', ' '];

pub type Solution = HashMap<char, u8>;
pub type Column = Vec<char>;

fn input_to_terms(input: &str) -> Option<Vec<String>> {
    let terms: Vec<String> = input
        .split(|c| DELIMITERS.contains(&c))
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .rev()
        .collect();
    // test if any term 1..N is longer than term 0
    if terms[1..].iter().any(|t| t.len() > terms[0].len()) {
        return None;
    }
    Some(terms)
}
//no duplicate chars; sorted
fn terms_to_unique_chars(terms: &Vec<String>) -> Vec<char> {
    let mut unique = terms
        .iter()
        .flat_map(|s| s.chars())
        .collect::<Vec<char>>();
    unique.dedup();
    unique.sort();
    unique
}

fn terms_to_columns(terms: &Vec<String>) -> Vec<Vec<char>> {
    let mut columns = Vec::new();
    for index in 0..terms[0].len() {
        let mut column = Vec::new();
        for term in terms.iter() {
            if let Some(c) = term.chars().rev().nth(index) {
                column.push(c);
            }
        }
        columns.push(column);
    }
    columns
}

fn evaluate_column(column: &Vec<char>, carry: u8, map: &HashMap<char, u8>) -> Option<u8> {
    let terms = column.iter().map(|c| map.get(c).unwrap()).copied().collect::<Vec<u8>>();
    let mut sum = carry;
    let mut carry = 0;
    for term in terms[1..].iter() {
        sum += term;
    }
    if sum > 9 {
        carry = sum / 10;
        sum = sum % 10;
    }
    if sum == terms[0] {
        Some(carry)
    } else {
        None
    }
}

fn evaluate_columns(columns: &Vec<Vec<char>>, map: &HashMap<char, u8>) -> bool {
    let mut carry = 0;
    for column in columns {
        if let Some(c) = evaluate_column(column, carry, map) {
            carry = c;
        } else {
            return false;
        }
    }
    if carry == 0 {
        true
    } else {
        false
    }
}

pub fn max_permutations(chars: &Vec<char>, values: &Vec<u8>) -> usize {
    // n! / (m - n)!
    let n = chars.len();
    let m = values.len();
    (0..n).fold(1, |acc, x| acc * (m - x))
}

fn hashmap_to_sorted_vec_of_tuples(hashmap: &HashMap<char, u8>) -> Vec<(char, u8)> {
    let mut vec: Vec<(char, u8)> = hashmap.iter().map(|(k, v)| (*k, *v)).collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec
}

fn chars_to_values(terms: &Vec<String>, map: &HashMap<char, u8>) -> Vec<String> {
    terms
        .iter()
        .map(|s| s.chars().map(|c| map.get(&c).unwrap().to_string()).collect::<String>())
        .collect::<Vec<String>>()
}

fn available_numbers(accepted: &Vec<(char, u8)>) -> Vec<u8> {
    (0..10).filter(|n| !accepted.iter().any(|(_, v)| *v == *n)).collect()
}

fn starting_values(chars: &Vec<char>, numbers: &Vec<u8>) -> Vec<u8> {
    numbers.iter().take(chars.len()).rev().map(|v| *v).collect::<Vec<u8>>()
}

pub fn escote_solve(input: &str) -> Option<HashMap<char, u8>> {
    let terms = input_to_terms(input)?;
    let chars = terms_to_unique_chars(&terms);
    let chars2 = chars.clone().into_iter().collect::<HashSet<char>>();
    let columns = terms_to_columns(&terms);
    let p = Permutation::new(&chars2);
    //let p =Permutation2::new(&chars);
    println!("p: {:#?}", p);
    for (i, solution) in p.enumerate() {
        println!("{i}: {:?}", hashmap_to_sorted_vec_of_tuples(&solution));
        if evaluate_columns(&columns, &solution) {
            if chars_to_values(&terms, &solution).iter().any(|s| s.starts_with('0')) {
                return None;
            }
            return Some(solution);
        }
    }
    None
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mine = false;
    if mine {
        escote_solve(input)
    } else {
        chibbyone_solve(input)
    }
}
