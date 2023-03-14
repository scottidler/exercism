use std::collections::HashMap;

const DELIMITERS: [char; 3] = [' ', ',', '\n'];

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for word in words
        .to_lowercase()
        .split(|c: char| DELIMITERS.contains(&c))
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|s| !s.is_empty())
    {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}
