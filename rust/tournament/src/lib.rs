
use std::collections::HashMap;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Debug, Clone, PartialEq, Eq)]
enum MatchResult {
    Win,
    Draw,
    Loss,
}

impl MatchResult {
    fn new(result: &str) -> Self {
        match result {
            "win" => MatchResult::Win,
            "draw" => MatchResult::Draw,
            "loss" => MatchResult::Loss,
            _ => panic!("Invalid match result"),
        }
    }
    fn opposite(&self) -> MatchResult {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Draw => MatchResult::Draw,
            MatchResult::Loss => MatchResult::Win,
        }
    }
    fn points(&self) -> u32 {
        match self {
            MatchResult::Win => 3,
            MatchResult::Draw => 1,
            MatchResult::Loss => 0,
        }
    }
}

pub fn tally(results: &str) -> String {
    let matches = results
        .lines()
        .map(|line| line.split(';').collect::<Vec<&str>>())
        .map(|line| (line[0].to_string(), line[1].to_string(), MatchResult::new(&line[2])))
        .collect::<Vec<(String, String, MatchResult)>>();
    let mut results = HashMap::<String, Vec<MatchResult>>::new();
    for (team1, team2, result) in matches {
        results.entry(team1.clone()).or_insert(vec![]).push(result.clone());
        results.entry(team2.clone()).or_insert(vec![]).push(result.opposite());
    }
    let mut output: Vec<String> = vec![HEADER.to_string()];
    let mut data: Vec<(String, u32, u32, u32, u32, u32)> = vec![];
    for (team, result) in results {
        data.push((
            team,
            result.len() as u32,
            result.iter().filter(|r| **r == MatchResult::Win).count() as u32,
            result.iter().filter(|r| **r == MatchResult::Draw).count() as u32,
            result.iter().filter(|r| **r == MatchResult::Loss).count() as u32,
            result.iter().map(|r| r.points()).sum::<u32>()));
    }
    data.sort_by(|a, b| b.5.cmp(&a.5).then(a.0.cmp(&b.0)));
    for (team, mp, w, d, l, p) in data {
        output.push(format!("{:30} | {:2} | {:2} | {:2} | {:2} | {:2}", team, mp, w, d, l, p));
    }
    let text = output.join("\n");
    println!("{}", text);
    text
}
