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
        let values = available_values(&chars, &accepted);
        let max = max_permutations(&chars, &values);
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
        let values = available_values(&chars, &accepted);
        let max = max_permutations(&chars, &values);
        Self {
            chars: chars.clone(),
            count: 0,
            max: max,
            values: values,
            accepted: accepted.clone(),
        }
    }
    /*
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
    */
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
        self.chars
            .iter()
            .copied()
            .zip(self.values.iter().copied())
            .collect()
    }
}
impl Iterator for Permutation2 {
    type Item = HashMap<char, u8>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == self.max {
            None
        } else if self.count == 0 {
            self.count += 1;
            Some(
                self.chars
                    .iter()
                    .copied()
                    .zip(self.values.iter().copied())
                    .collect(),
            )
        } else {
            self.count += 1;
            Some(self.next_hashmap())
        }
    }
}
impl fmt::Display for Permutation2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Permutation2(chars={:?}, count={}, max={}, values={:?}, accepted={:?})",
            self.chars, self.count, self.max, self.values, self.accepted
        )
    }
}