use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);
const DELIMITERS: [char; 3] = ['+', '=', ' '];

pub type Solution = HashMap<char, u8>;
pub type Column = Vec<char>;
type Term = Vec<char>;
type Value = Vec<u8>;

pub fn max_permutations(chars: &Vec<char>, values: &Vec<u8>) -> usize {
    // n! / (m - n)!
    let n = chars.len();
    let m = values.len();
    (0..n).fold(1, |acc, x| acc * (m - x))
}

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

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let puzzle = Puzzle::new(input)?;
    puzzle.solve()
}
