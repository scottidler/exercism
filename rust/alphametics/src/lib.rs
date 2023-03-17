use std::collections::{HashMap, HashSet};
use std::fmt;




//chibbyone solution

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

pub fn solve_working(input: &str) -> Option<HashMap<char, u8>> {
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

// chibbyone's solution





fn format_items(items: &[&str], before: Option<&str>, between: Option<&str>, after: Option<&str>) -> String
where
{
    //if between is not None, then join with between
    //if between is None, then join with ""
    let mut s = between.map_or_else(|| items.join(""), |between| items.join(between));
    //if before is not None, then prepend with before
    if let Some(before) = before {
        s = format!("{before}{s}");
    }
    //if after is not None, then append with after
    if let Some(after) = after {
        s = format!("{s}{after}");
    }
    s
}

#[derive(Debug)]
pub struct Column {
    terms: Vec<char>,
}

impl Column {
    pub fn new(terms: Vec<char>) -> Self {
        Self { terms }
    }
    fn addends(&self, solution: &Solution) -> Option<Vec<u8>> {
        let mut addends = Vec::new();
        for term in self.terms.iter().skip(1) {
            if let Some(value) = solution.get(term) {
                addends.push(*value);
            } else {
                return None;
            }
        }
        Some(addends)
    }
    fn result(&self, solution: &Solution) -> Option<u8> {
        if let Some(result) = self.terms.get(0) {
            solution.get(result).map(|v| *v)
        } else {
            None
        }
    }
    // evaluate that the addends add up to the result
    // if they do return the carry
    pub fn evaluate(&self, carry: u8, solution: &Solution) -> Option<u8> {
        let mut sum = carry;
        let mut carry = 0;
        for term in self.addends(solution)? {
            sum += term;
        }
        if sum > 9 {
            carry = sum / 10;
            sum = sum % 10;
        }
        if sum == self.result(solution)? {
            Some(carry)
        } else {
            None
        }
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();
        for c in self.terms.iter().rev() {
            text = format!("{c}{text}");
        }
        write!(f, "Column({})", text)?;
        Ok(())
    }
}

pub type Columns = Vec<Column>;

pub fn max_permutations(n: usize, m: usize) -> usize {
    // n! / (m - n)!
    (0..n).fold(1, |acc, x| acc * (m - x))
}

pub type Solution = HashMap<char, u8>;
pub type Substitutions = Vec<(char, u8)>;

/*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permutation3 {
    count: usize,
    place: usize,
    subs: Vec<Substitutions>,
}

impl Permutation3 {
    pub fn new() -> Self {
        Self {
            count: 0,
            place: 0,
            subs: vec![],
        }
    }
    pub fn with_new_chars(&mut self, chars: Vec<char>) -> &Self {
        let chars: Vec<char> = chars
            .into_iter()
            .filter(|c| !self.contains_key(*c))
            .collect();
        if !chars.is_empty() {
            let sub = self.chars_to_substitions(chars);
            self.subs.push(sub);
            self.place = self.subs.len() - 1;
        }
        self
    }
    fn max_permutations(&self) -> usize {
        max_permutations(self.keys().len(), 10)
    }
    fn available_numbers(&self) -> Vec<u8> {
        (0..10)
            .filter(|n| !self.values().contains(n))
            .collect()
    }
    fn chars_to_substitions(&self, chars: Vec<char>) -> Substitutions {
        chars
            .into_iter()
            .zip(self.available_numbers().into_iter())
            .collect()
    }
    fn contains_key(&self, c: char) -> bool {
        //check if any of the first members of the tuples in each set contain c
        self.subs
            .iter()
            .any(|s| s.iter()
            .any(|(k, _)| k == &c))
    }
    fn keys(&self) -> Vec<char> {
        if self.place >= self.subs.len() {
            return vec![];
        }
        //get the first member of each tuple in <place> set
        self.subs[self.place]
            .iter()
            .map(|(k, _)| k.to_owned())
            .collect()
    }
    fn values(&self) -> Vec<u8> {
        if self.place >= self.subs.len() {
            return vec![];
        }
        //get the second member of each tuple in <place> set
        self.subs[self.place]
            .iter()
            .map(|(_, v)| v.to_owned())
            .collect()
    }
    fn all_keys(&self) -> Vec<char> {
        //just get the first member of each tuple in each set
        self.subs
            .iter()
            .flat_map(|s| s.iter()
            .map(|(k, _)| k.to_owned()))
            .collect()
    }
    fn all_values(&self) -> Vec<u8> {
        //just get the second member of each tuple in each set
        self.subs
            .iter()
            .flat_map(|s| s.iter()
            .map(|(_, v)| v.to_owned()))
            .collect()
    }
    fn all_items(&self) -> Vec<(char, u8)> {
        //just get the second member of each tuple in each set
        self.subs
            .iter()
            .flat_map(|s| s.iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned())))
            .collect()
    }
    fn get(&self, c: char) -> Option<u8> {
        //find the first tuple in each set that contains c and return the second member
        self.subs
            .iter()
            .find_map(|s| s.iter()
            .find_map(|(k, v)| if k == &c { Some(v) } else { None }))
            .map(|v| v.to_owned())
    }
    fn insert(&mut self, p: usize ,c: char, v: u8) -> () {
        if p == self.subs.len() {
            self.subs.push(Substitutions::new());
            self.place = p;
        }
        self.subs[self.place].push((c, v));
    }
    /*
    fn chars(&mut self, chars: &Vec<char>) -> Vec<(char, u8)> {
        for (n, c) in chars.iter().enumerate() {
            if !self.contains_key(*c) {
                self.insert(self.place + 1, *c, n as u8);
            }
        }
        self.subs[self.place].clone()
    }
    */
    fn find_next_combination(&mut self, index: usize) {
        //if index >= self.current_values.len() {
        if index >= self.subs[self.place].len() {
            return;
        }
        //let mut next_digit = (self.current_values[index] + 1) % 10;
        let mut next_digit = (self.values()[index] + 1) % 10;
        if next_digit == 0 {
            self.find_next_combination(index + 1);
        }
        //while self.current_values[index + 1..].contains(&next_digit) {
        while self.values()[index + 1..].contains(&next_digit) {
            next_digit = (next_digit + 1) % 10;
            if next_digit == 0 {
                self.find_next_combination(index + 1);
            }
        }
        //self.current_values[index] = next_digit;
        //self.subs[self.place].insert((self.keys()[index], next_digit));
        self.insert(self.place, self.keys()[index], next_digit);
    }
    //fn next_hashmap(&mut self) -> HashMap<char, u8> {
    fn next_hashmap(&mut self) -> Solution {
        self.find_next_combination(0);
        //self.letters
        //    .iter()
        //    .copied()
        //    .zip(self.current_values.iter().copied())
        //    .collect()
        self.all_items()
            .iter()
            .copied()
            .collect()

    }
}

impl Iterator for Permutation3 {
    type Item = Solution;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max_permutations() {
            None
        } else if self.count == 0 {
            self.count += 1;
            Some(self.all_items()
                .iter()
                .copied()
                .collect())
        } else {
            self.count += 1;
            Some(self.next_hashmap())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Permutation4 {
    count: usize,
    stack: Vec<Substitutions>,
}

impl Permutation4 {
    pub fn new() -> Self {
        Self {
            count: 0,
            stack: vec![Substitutions::new()],
        }
    }
    pub fn with_new_chars(&mut self, chars: Vec<char>) -> Self {
        let chars: Vec<char> = chars
            .into_iter()
            .filter(|c| !self.contains_key(*c))
            .collect();
        if !chars.is_empty() {
            let sub = self.chars_to_substitions(chars);
            self.stack.push(sub);
        }
        self.clone()
    }
    fn max_permutations(&self) -> usize {
        max_permutations(self.keys().len(), 10)
    }
    fn available_numbers(&self) -> Vec<u8> {
        (0..10)
            .filter(|n| !self.values().contains(n))
            .collect()
    }
    fn chars_to_substitions(&self, chars: Vec<char>) -> Substitutions {
        chars
            .into_iter()
            .zip(self.available_numbers().into_iter())
            .collect()
    }
    fn contains_key(&self, c: char) -> bool {
        //check if any of the first members of the tuples in each set contain c
        self.stack
            .iter()
            .any(|s| s.iter()
            .any(|(k, _)| k == &c))
    }
    fn keys(&self) -> Vec<char> {
        if self.stack.len() == 0 {
            return vec![];
        }
        //get the first member of each tuple in <place> set
        self.stack[0]
            .iter()
            .map(|(k, _)| k.to_owned())
            .collect()
    }
    fn values(&self) -> Vec<u8> {
        if self.stack.len() == 0 {
            return vec![];
        }
        //get the second member of each tuple in <place> set
        self.stack[0]
            .iter()
            .map(|(_, v)| v.to_owned())
            .collect()
    }
    fn all_keys(&self) -> Vec<char> {
        //just get the first member of each tuple in each set
        self.stack
            .iter()
            .flat_map(|s| s.iter()
            .map(|(k, _)| k.to_owned()))
            .collect()
    }
    fn all_values(&self) -> Vec<u8> {
        //just get the second member of each tuple in each set
        self.stack
            .iter()
            .flat_map(|s| s.iter()
            .map(|(_, v)| v.to_owned()))
            .collect()
    }
    fn all_items(&self) -> Vec<(char, u8)> {
        //just get the second member of each tuple in each set
        self.stack
            .iter()
            .flat_map(|s| s.iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned())))
            .collect()
    }
    fn get(&self, c: char) -> Option<u8> {
        //find the first tuple in each set that contains c and return the second member
        self.stack
            .iter()
            .find_map(|s| s.iter()
            .find_map(|(k, v)| if k == &c { Some(v) } else { None }))
            .map(|v| v.to_owned())
    }
    fn insert(&mut self, c: char, v: u8) -> Option<u8> {
        self.stack[0].push((c, v));
        Some(v)
    }
    fn find_next_combination(&mut self, index: usize) {
        //if index >= self.current_values.len() {
        if index >= self.stack[0].len() {
            return;
        }
        //let mut next_digit = (self.current_values[index] + 1) % 10;
        let mut next_digit = (self.values()[index] + 1) % 10;
        if next_digit == 0 {
            self.find_next_combination(index + 1);
        }
        //while self.current_values[index + 1..].contains(&next_digit) {
        while self.values()[index + 1..].contains(&next_digit) {
            next_digit = (next_digit + 1) % 10;
            if next_digit == 0 {
                self.find_next_combination(index + 1);
            }
        }
        //self.current_values[index] = next_digit;
        self.insert(self.keys()[index], next_digit);
    }
    //fn next_hashmap(&mut self) -> HashMap<char, u8> {
    fn next_hashmap(&mut self) -> Solution {
        self.find_next_combination(0);
        //self.letters
        //    .iter()
        //    .copied()
        //    .zip(self.current_values.iter().copied())
        //    .collect()
        self.all_items()
            .iter()
            .copied()
            .collect()

    }
}

impl Iterator for Permutation4 {
    type Item = Solution;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max_permutations() {
            None
        } else if self.count == 0 {
            self.count += 1;
            Some(self.all_items()
                .iter()
                .copied()
                .collect())
        } else {
            self.count += 1;
            Some(self.next_hashmap())
        }
    }
}
*/

/*
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
*/

pub fn permutations(chars: Vec<char>, values: Vec<u8>) -> Vec<Substitutions> {
    vec![]
}

#[derive(Debug)]
pub struct Alphametic {
    chars: Vec<char>,
    terms: Vec<String>,
    columns: Columns,
}

const DELIMITERS: [char; 3] = ['+', '=', ' '];

impl Alphametic {
    pub fn new(input: &str) -> Self {
        let terms: Vec<String> = input
            .split(|c| DELIMITERS.contains(&c) )
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .rev()
            .collect();
        //unique list of chars in terms
        let chars: Vec<char> = terms
            .iter()
            .flat_map(|s| s.chars())
            .collect::<HashSet<char>>()
            .into_iter()
            .collect();
        let mut columns = Vec::new();
        for index in 0..terms[0].len() {
            let mut column = Vec::new();
            for term in terms.iter() {
                if let Some(c) = term.chars().rev().nth(index) {
                    column.push(c);
                }
            }
            columns.push(Column::new(column));
        }
        Self {
            chars,
            terms,
            columns,
        }
    }
    pub fn addends(&self) -> &[String] {
        &self.terms[1..]
    }
    pub fn result(&self) -> &String {
        &self.terms[0]
    }
    pub fn terms(&self) -> &[String] {
        &self.terms
    }
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }
    pub fn solve(&self) -> Option<Solution> {
        let chars: HashSet<char> = self.chars.iter().copied().collect();
        let permutation = Permutation::new(&chars);
        let mut soltuion = Solution::new();
        let mut carry = 0;
        for c in self.columns.iter() {
            for p in permutation {
                if let Some(carry) = c.evaluate(carry, &p) {
                    soltuion = p;
                } else {
                    break;
                }
            }
            
        }
        None
    }
}

impl fmt::Display for Alphametic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();
        let terms: &[&str] = &self
            .terms
            .iter()
            .map(|v| v.as_str())
            .collect::<Vec<&str>>();
        let columns: Vec<String> = self.columns
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>();
        let columns: &[&str] = &columns
            .iter()
            .map(|v| v.as_str())
            .collect::<Vec<&str>>();
        let terms = format_items(terms, Some("["), Some(", "), Some("]"));
        let columns = format_items(columns, Some("["), Some(", "), Some("]"));
        write!(f, "Alphametic(terms={terms}, columns={columns})")?;
        Ok(())
    }
}

pub fn solve(input: &str) -> Option<Solution> {
    let alphametic = Alphametic::new(input);
    alphametic.solve()
}

#[derive(Debug)]
pub struct Apple<'a> {
    banana: &'a Banana,
}

impl<'a> Apple<'a> {
    pub fn new(banana: &'a Banana) -> Self {
        Self { banana }
    }
    pub fn integer(&self) -> i32 {
        self.banana.integer
    }
}

#[derive(Debug)]
pub struct Banana {
    pub integer: i32,
}

impl Banana {
    pub fn new(integer: i32) -> Self {
        Self { integer }
    }
}

