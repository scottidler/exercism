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
        // sorted vec of chars
        let sorted = {
            let mut v: Vec<char> = s.iter().copied().collect();
            v.sort();
            v
        };
        Self {
            //letters: s.iter().copied().collect(),
            letters: sorted,
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