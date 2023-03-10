use std::collections::HashMap;
use std::thread;

type Frequency = HashMap<char, usize>;

fn count_letters(string: &str) -> Frequency {
    let mut frequency = Frequency::new();
    for c in string.chars().filter(|c| c.is_alphabetic()) {
        *frequency.entry(c.to_ascii_lowercase()).or_default() += 1;
    }
    frequency
}

fn merge(f1: Frequency, f2: Frequency) -> Frequency {
    let mut result = f1;
    for (key, value) in f2 {
        let counter = result.entry(key).or_insert(0);
        *counter += value;
    }
    result
}

pub fn frequency(input: &[&str], worker_count: usize) -> Frequency {
    let mut result = Frequency::new();
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles = vec![];
    for chunk in chunks {
        let string = chunk.join("");
        let handle = thread::spawn(move || {
            count_letters(&string)
        });
        handles.push(handle);
    }
    for handle in handles {
        let frequency = handle.join().unwrap();
        result = merge(result, frequency);
    }
    result
}
