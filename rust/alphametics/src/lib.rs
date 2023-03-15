use std::collections::HashMap;
use std::fmt;
//use std::result::Result;
//use std::error::Error;
//use std::collections::HashSet;

//use itertools::Itertools;

pub type Solution = HashMap<char, u8>;

const NUMBERS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/*
fn permutations(chars: Vec<char>) -> impl Iterator<Item = Vec<(char, u8)>> {
    NUMBERS
        .into_iter()
        .permutations(chars.len())
        .map(move |v| chars.iter().zip(v.iter()).map(|(&c, &n)| (c, n)).collect())
}
*/


/*
fn permutations() -> impl Iterator<Item = Vec<u8>> {
    NUMBERS.into_iter().permutations(10).map(|v| v.iter().map(|&n| n).collect())
}
*/

/*
fn create_solution(chars: &[char], permutation: &[u8]) -> HashMap<char, u8> {
    chars
        .iter()
        .zip(permutation.iter())
        .map(|(&c, &n)| (c, n))
        .collect()
}
*/

/*
fn value(term: &str, solution: &HashMap<char, u8>) -> u32 {
    term.chars()
        .map(|c| solution.get(&c).unwrap())
        .fold(0, |acc, &n| acc * 10 + n as u32)
}
*/

/*
fn value(c: char, solution: &HashMap<char, u8>) -> u32 {
    *solution.get(&c).unwrap() as u32
}
*/
/*
#[derive(Debug)]
struct Alphametic {
    terms: Vec<String>,
    //chars: Vec<char>,
    last: usize,
}

impl Alphametic {
    fn new(equation: &str) -> Self {
        let terms = equation
            .split(|c: char| !c.is_alphabetic())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        /*
        let mut chars = equation
            .chars()impl Iterator<Item = Vec<u8>>
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<char>>();
        chars.sort();
        chars.dedup();
        */
        let last = terms.len() - 1;
        Self { terms, last }
    }
    /*
    fn evaluate(&self, solution: &HashMap<char, u8>) -> bool {            let mut solution = HashMap::new();
            for (c, &n) in self.chars.iter().zip(permutation.iter()) {
                solution.insert(*c, n);
            }
            acc + value(term, solution)
        });
        let rhs = value(&self.terms[self.last], solution);
        lhs == rhs
    }
    pub fn solve(&self) -> Option<HashMap<char, u8>> {
        let mut solution = HashMap::new();
        for permutation in permutations() {
            for (c, &n) in self.chars.iter().zip(permutation.iter()) {
                solution.insert(*c, n);
            }
            if self.evaluate(&solution) {
                return Some(solution);
            }
        }
        None
    }
    */

    pub fn solve(&self) -> Option<HashMap<char, u8>> {
        for p in 0..self.terms[self.last].len() {

        }
    }
}
*/

/*


#[derive(Debug)]
struct Solution {
    map: HashMap<char, u8>,
}

impl Solution {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn reset(&mut self) {
        self.map.clear();
    }
}

impl Iterator for Solution {
    type Item = HashMap<char, u8>;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Debug)]
struct Alphametic {
    terms: Vec<String>,
}

impl Alphametic {
    fn new(equation: &str) -> Self {
        let terms = equation
            .split(|c: char| !c.is_alphabetic())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Self { terms }
    }

    fn addends(&self) -> Vec<String> {
        self.terms[..self.terms.len() - 1].to_vec()
    }
    fn sum(&self) -> String {
        self.terms[self.terms.len() - 1].to_string()
    }
    fn chars(&self, place: usize) -> Vec<char> {
        self.terms
            .iter()
            .filter(|s| s.len() > place)
            .map(|s| s.chars().rev().nth(place).unwrap())
            .collect()
    }

    pub fn solve(&self) -> Option<HashMap<char, u8>> {

        let places = self.sum().len();
        println!("places: {}", places);
        for place in 0..places {
            let chars = self.chars(place);
            println!("place: {}, chars: {:?}", place, chars);
            for permutation in permutations(chars) {
                let mut solution: HashMap<char, u8> = HashMap::from_iter(permutation);
                println!("solution: {:?}", solution);
                let lhs = self.addends().iter().fold(0, |acc, term| {
                    acc + term
                        .chars()
                        .rev()
                        .map(|c| solution.get(&c).unwrap())
                        .fold(0, |acc, &n| acc * 10 + n as u32)
                });
                let rhs = self.sum()
                    .chars()
                    .rev()
                    .map(|c| solution.get(&c).unwrap())
                    .fold(0, |acc, &n| acc * 10 + n as u32);
                println!("lhs: {}, rhs: {}", lhs, rhs);
                if lhs == rhs {
                }
            }
        }
        None
    }
}
*/

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Term {
    pub chars: Vec<char>,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for c in self.chars.iter().rev() {
            output += format!("{}", c).as_str();
        }
        write!(f, "Term({})", output)?;
        Ok(())
    }
}

impl Term {
    pub fn new(term: &str) -> Self {
        Self {
            chars: term
                .chars()
                .rev()
                .collect(),
        }
    }
    pub fn places(&self) -> usize {
        self.chars.len()
    }
    pub fn char(&self, place: usize) -> Option<char> {
        self.chars.get(place).map(|&c| c)
    }
    pub fn evaluate(&self, place: usize, solution: &Solution) -> Option<u32> {
        self.chars
            .iter()
            .rev()
            .nth(place)
            .and_then(|&c| solution.get(&c).map(|&n| n as u32))
    }
    pub fn value(&self, solution: &Solution) -> Option<u32> {
        self.chars
            .iter()
            .rev()
            .map(|&c| solution.get(&c).map(|&n| n as u32))
            .fold(Some(0), |acc, n| {
                acc.and_then(|acc| n.map(|n| acc * 10 + n))
            })
    }
}

#[derive(Debug)]
pub struct Alphametic {
    pub terms: Vec<Term>,
    pub solution: Solution,
}

impl fmt::Display for Alphametic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut terms = String::new();
        for term in self.terms.iter() {
            if let Some(value) = term.value(&self.solution) {
                terms += format!("{}=[{}] ", term, value).as_str();
            } else {
                terms += format!("{} ", term).as_str();
            }
        }
        write!(f, "Alphametic(terms=[{}] solution={:?})", terms, self.solution)?;
        Ok(())
    }
}

impl Alphametic {
    pub fn new(equation: &str) -> Self {
        let terms = equation
            .split(|c: char| !c.is_alphabetic())
            .filter(|s| !s.is_empty())
            .map(|s| Term::new(s))
            .collect::<Vec<Term>>();
        let solution = Solution::new();
        Self { terms, solution }
    }
    pub fn addends(&self) -> Vec<Term> {
        self.terms[..self.terms.len() - 1].to_vec()
    }
    pub fn sum(&self) -> Term {
        self.terms[self.terms.len() - 1].clone()
    }
    pub fn addends2(&self) -> Vec<&Term> {
        self.terms[..self.terms.len() - 1].iter().collect()
    }
    pub fn sum2(&self) -> &Term {
        &self.terms[self.terms.len() - 1]
    }

    // return char at place in terms, or skip it
    pub fn chars(&self, place: usize) -> Vec<char> {
        self.terms
            .iter()
            .filter(|t| t.places() > place)
            .map(|t| t.char(place).unwrap())
            .collect()
    }
    pub fn insert(&mut self, permutation: &Vec<(char, u8)>) {
        for (c, n) in permutation {
            self.solution.insert(*c, *n);
        }
    }
    pub fn remove(&mut self, permutation: &Vec<(char, u8)>) {
        for (c, _) in permutation {
            self.solution.remove(&c);
        }
    }
    pub fn reset(&mut self) {
        self.solution.clear();
    }
    pub fn numbers(&self) -> Vec<u8> {
        NUMBERS
            .iter()
            .filter(|&&n| !self.solution.values()
            .any(|&x| x == n))
            .cloned()
            .collect()
    }
    pub fn permutations(&mut self, chars: Vec<char>) -> impl Iterator<Item = Vec<(char, u8)>> {
        let mut permutations = Vec::new();
        for n in self.numbers() {
            self.solution.insert(chars[0], n);
            if chars.len() == 1 {
                permutations.push(self.solution.iter().map(|(&c, &n)| (c, n)).collect());
            } else {
                permutations.extend(self.permutations(chars[1..].to_vec()));
            }
        }
        self.solution.remove(&chars[0]);
        permutations.into_iter()
    }
    pub fn evaulate(&self, place: usize, terms: Vec<&Term>) -> u32 {
        terms.iter().fold(0, |acc, term| {
            acc + term
                .evaluate(place, &self.solution)
                .unwrap_or(0)
        })
    }
    pub fn solve(&mut self) -> Option<HashMap<char, u8>> {
        let places = self.sum().places();
        let mut lhs = 0;
        let mut rhs = 0;
        let mut carry = 0;
        for place in 0..places {
            let mut chars = self.chars(place);
            println!("place={} chars={:?}", place, chars);
            chars = chars
                .iter()
                .filter(|&&c| !self.solution.contains_key(&c))
                .cloned()
                .collect();
            for permutation in self.permutations(chars) {
                println!("  permutation={:?}", permutation);
                self.insert(&permutation);
                lhs = self.evaulate(place, self.addends2()) + carry;
                rhs = self.evaulate(place, vec![self.sum2()]);
                println!("  lhs={}, rhs={}", lhs, rhs);

                //calculate carry
                carry = lhs / 10;
                lhs = lhs % 10;
                println!("  carry={}, lhs={}", carry, lhs);

                if lhs == rhs {
                    println!("  break; lhs={} == rhs={}", lhs, rhs);
                    break;
                } else {
                    println!("  reset");
                    self.remove(&permutation);
                }
            }
        }
        if lhs == rhs {
            Some(self.solution.clone())
        } else {
            None
        }
    }
}


pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut alphametic = Alphametic::new(input);
    alphametic.solve()
}
