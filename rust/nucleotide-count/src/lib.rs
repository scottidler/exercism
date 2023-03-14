use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = [
    'A', 
    'C', 
    'G', 
    'T',
];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for c in dna.chars() {
        if !NUCLEOTIDES.contains(&c) {
            return Err(c);
        }
        if c == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for n in NUCLEOTIDES.iter() {
        counts.insert(*n, 0);
    }

    for c in dna.chars() {
        if !NUCLEOTIDES.contains(&c) {
            return Err(c);
        }
        *counts.get_mut(&c).unwrap() += 1;
    }
    Ok(counts)
}
