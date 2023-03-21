use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

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
    max: usize,
    count: usize,
    chars: Vec<char>,
    values: Vec<u8>,
}
impl Permutation {
    fn new(chars: &Vec<char>) -> Self {
        fn combinations(num: usize) -> usize {
            let start = 10 - num + 1;
            let mut result = 1;
            for v in start..=10 {
                result *= v;
            }
            result
        }
        Self {
            max: combinations(chars.len()),
            count: 0,
            chars: chars.iter().copied().collect(),
            values: (0..chars.len()).rev().map(|x| x as u8).collect(),
        }
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
        self.chars.iter().copied().zip(self.values.iter().copied()).collect()
    }
}
impl Iterator for Permutation {
    type Item = HashMap<char, u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max {
            None
        } else if self.count == 0 {
            self.count += 1;
            Some(self.chars.iter().copied().zip(self.values.iter().copied()).collect())
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
    let mut chars: Vec<char> = set.iter().copied().collect();
    chars.sort();
    let mut perm = Permutation::new(&chars);
    println!("perm: {:#?}", perm);
    let mut i = 0;
    perm.find(|hashmap| {
        println!("{i}: {:?}", hashmap_to_sorted_vec_of_tuples(&hashmap));
        i += 1;
        is_valid(hashmap, &input, result) && convert_to_numbers_and_check_result(&input, result, hashmap)
    })
}

///////////////////////////
///////////////////////////
///////////////////////////

static COUNTER: AtomicUsize = AtomicUsize::new(0);
const DELIMITERS: [char; 3] = ['+', '=', ' '];

pub type Solution = HashMap<char, u8>;
pub type Column = Vec<char>;
type Term = Vec<char>;
type Value = Vec<u8>;

fn column_to_value(column: &Column, solution: &Solution) -> Value {
    column
        .iter()
        .map(|c| solution.get(c).unwrap())
        .copied()
        .collect::<Value>()
}

#[derive(Debug)]
struct Puzzle {
    terms: Vec<Term>,
    columns: Vec<Column>,
}

impl Puzzle {
    pub fn new(input: &str) -> Option<Self> {
        let terms: Vec<Term> = input
            .split(|c| DELIMITERS.contains(&c))
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.chars().rev().collect())
            .rev()
            .collect();
        if terms.iter().skip(1).any(|term| term.len() > terms[0].len()) {
            return None;
        }
        let mut columns: Vec<Column> = Vec::new();
        for index in 0..terms[0].len() {
            let mut column = Vec::new();
            for term in terms.iter() {
                if let Some(c) = term.iter().nth(index) {
                    column.push(*c);
                }
            }
            columns.push(column);
        }
        Some(Self { terms, columns })
    }
    fn values(&self, solution: &Solution) -> Option<Vec<Value>> {
        self.terms
            .iter()
            .map(|term| {
                term.iter()
                    .map(|c| solution.get(c).map(|v| *v))
                    .collect::<Option<Value>>()
            })
            .collect()
    }
    fn column(&self, index: usize, unique: bool) -> Option<Column> {
        let mut column: Column = self.columns.get(index)?.iter().copied().collect();
        if unique {
            column.dedup();
            column.sort();
        }
        Some(column)
    }
    fn any_term_with_leading_zero(&self, solution: &Solution) -> bool {
        self.terms
            .iter()
            .any(|term| solution.get(term.last().unwrap()).unwrap() == &0)
    }
    fn is_valid(&self, solution: &Solution) -> bool {
        !self.any_term_with_leading_zero(solution)
    }
    pub fn evaluate_column(&self, index: usize, carry: u64, solution: &Solution) -> Option<u64> {
        let column = self.column(index, false)?;
        let value: Vec<u8> = column_to_value(&column, solution);
        let mut sum = carry;
        let mut carry = 0;
        for digit in value[1..].iter() {
            sum += *digit as u64;
        }
        if sum > 9 {
            carry = sum / 10;
            sum = sum % 10;
        }
        if sum == value[0].into() {
            Some(carry)
        } else {
            None
        }
    }
    pub fn evaluate_columns(&self, index: usize, carry: u64, solution: &Solution) -> Option<Solution> {
        if index >= self.columns.len() {
            if carry == 0 {
                Some(solution.clone())
            } else {
                None
            }
        } else {
            let chars = self.column(index, true)?;
            let permutor = Permutor::new(&chars, solution);
            for (_id, solution) in permutor {
                if let Some(carry) = self.evaluate_column(index, carry, &solution) {
                    if let Some(solution) = self.evaluate_columns(index + 1, carry, &solution) {
                        if self.is_valid(&solution) {
                            return Some(solution);
                        } else {
                            return None;
                        }
                    }
                }
            }
            None
        }
    }
    pub fn solve(&self) -> Option<Solution> {
        self.evaluate_columns(0, 0, &HashMap::new())
    }
}
impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Puzzle {{\n\
            \x20   terms: {:?}\n\
            \x20   columns: {:?}\n\
        }}",
            self.terms, self.columns
        )
    }
}

#[derive(Debug)]
pub struct Permutor {
    id: usize,
    max: usize,
    count: usize,
    chars: Vec<char>,
    indices: Vec<usize>,
    available: Vec<u8>,
    accepted: Vec<(char, u8)>,
}
impl Permutor {
    pub fn new(chars: &Vec<char>, solution: &Solution) -> Self {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        let count = 0;
        let mut chars = chars
            .iter()
            .copied()
            .filter(|c| !solution.contains_key(c))
            .collect::<Vec<char>>();
        chars.dedup();
        chars.sort();
        let indices = (0..chars.len()).rev().collect();
        let mut accepted = solution.iter().map(|(c, v)| (*c, *v)).collect::<Vec<(char, u8)>>();
        accepted.sort();
        let available = (0..10)
            .filter(|i| !accepted.iter().any(|(_, v)| v == i))
            .collect::<Vec<u8>>();
        let max = max_permutations(&chars, &available);
        Self {
            id,
            max,
            count,
            chars,
            indices,
            available,
            accepted,
        }
    }
    pub fn solution(&self) -> Solution {
        let solution = self.accepted.clone();
        // zip the chars and values together
        // values are the indices of the available numbers
        let values = self.indices.iter().map(|i| self.available[*i]).collect::<Vec<u8>>();
        solution
            .into_iter()
            .chain(self.chars.iter().copied().zip(values.iter().copied()))
            .collect()
    }
    fn find_next_combination(&mut self, index: usize) {
        if index >= self.indices.len() {
            return;
        }
        let mut next_digit = (self.indices[index] + 1) % self.available.len();
        if next_digit == 0 {
            self.find_next_combination(index + 1);
        }
        while self.indices[index + 1..].contains(&next_digit) {
            next_digit = (next_digit + 1) % self.available.len();
            if next_digit == 0 {
                self.find_next_combination(index + 1);
            }
        }
        self.indices[index] = next_digit;
    }
    fn next_solution(&mut self) -> Solution {
        self.find_next_combination(0);
        self.solution()
    }
}
impl Iterator for Permutor {
    type Item = (usize, Solution);
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max {
            None
        } else if self.count == 0 {
            self.count += 1;
            Some((self.id, self.solution()))
        } else {
            self.count += 1;
            Some((self.id, self.next_solution()))
        }
    }
}
impl fmt::Display for Permutor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Permutor {{\n\
            \x20   id: {:?}\n\
            \x20   max: {:?}\n\
            \x20   chars: {:?}\n\
            \x20   indices: {:?}\n\
            \x20   count: {:?}\n\
            \x20   available: {:?}\n\
            \x20   accepted: {:?}\n\
        }}",
            self.id, self.max, self.chars, self.indices, self.count, self.available, self.accepted
        )
    }
}

pub fn escote_solve(input: &str) -> Option<Solution> {
    let puzzle = Puzzle::new(input)?;
    puzzle.solve()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mine = true;
    if mine {
        escote_solve(input)
    } else {
        chibbyone_solve(input)
    }
}
