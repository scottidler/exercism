pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for r in 0..minefield.len() {
        let mut row = String::new();
        for c in 0..minefield[r].len() {
            let mut count = 0;
            for i in r.saturating_sub(1)..=r + 1 {
                for j in c.saturating_sub(1)..=c + 1 {
                    if i < minefield.len() && j < minefield[i].len() && minefield[i].chars().nth(j) == Some('*') {
                        count += 1;
                    }
                }
            }
            if minefield[r].chars().nth(c) == Some('*') {
                row.push('*');
            } else if count == 0 {
                row.push(' ');
            } else {
                row.push_str(&count.to_string());
            }
        }
        result.push(row);
    }
    result
}
