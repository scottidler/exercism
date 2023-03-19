use alphametics::*;

fn puzzle(args: Vec<String>) -> String {
    if args.len() > 1 {
        args[1..].join(" ")
    } else {
        //"I + BB == ILL".to_string()
        "NO + NO + TOO == LATE".to_string()
    }
}


fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let puzzle = puzzle(args);
    if let Some(solution) = solve(&puzzle) {
        println!("{:?} = {:?}", puzzle, solution);
    } else {
        println!("No solution found for {}", puzzle);
    }
}