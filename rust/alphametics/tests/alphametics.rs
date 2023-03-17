use std::collections::{HashMap, HashSet};

use alphametics::*;
/*
#[test]
fn test_term() {
    let input = "ABC";
    let t = Term::new("ABC");
    let s = Solution::from_iter(vec![('A', 1), ('B', 2), ('C', 3)]);
    for (i, c) in input.chars().enumerate() {
        let v = t.evaluate(i, &s);
        assert_eq!(v, Some(3 - i as u8));
    }
}

#[test]
fn test_term2() {
    let t1 = Term::new("I");
    let t2 = Term::new("BB");
    let t3 = Term::new("ILL");
    let terms = vec![t1, t2, t3];
    let s = Solution::from_iter(vec![('I', 1), ('B', 9), ('L', 0)]);
    let (sum, carry) = Term::add(0, &terms[..2], 0, &s);
    assert_eq!(sum, 0);
    assert_eq!(carry, 1);
}

#[test]
#[ignore]
fn test_term3() {
    let t1 = Term::new("I");
    let t2 = Term::new("BB");
    let t3 = Term::new("ILL");
    let terms = vec![t1, t2, t3];
    let s = Solution::from_iter(vec![('I', 1), ('B', 9), ('L', 0)]);
    let mut sum = 0;
    let mut total = 0;
    let mut carry = 0;
    for p in 0..2 {
        (sum, carry) = Term::add(1, &terms[..2], carry, &s);
        total += sum * 10u32.pow(p as u32);
    }
    assert_eq!(carry, 1);
    assert_eq!(total, 100);
    if carry > 0 {
        total += carry;
    }
    assert_eq!(total, 100);
}

#[test]
fn test_term4() {
    let addends = vec![
        Term::new("I"),
        Term::new("BB"),
    ];
    let result = Term::new("ILL");
    let solution = Solution::from_iter(vec![('I', 1), ('B', 9), ('L', 0)]);
    for place in 0..result.places() {
        println!("place: {}", place);
        if let Some(carry) = Term::validate(place, &addends, &result, &solution) {
            assert_eq!(carry, 1);
        } else {
            assert!(false);
        }
    }
}
*/

#[test]
fn test_apple_banana() {
    let banana = Banana::new(13);
    let apple = Apple::new(&banana);
    assert_eq!(apple.integer(), 13);
}

#[test]
fn test_alphametic() {
    let a = Alphametic::new("I + BB == ILL");
    assert_eq!(a.columns.len(), 3);
}

#[test]
fn test_column() {
    let c1 = Column::new(vec!['L', 'B', 'I']);
    let s = Solution::from_iter(vec![('I', 1), ('B', 9), ('L', 0)]);
    if let Some(carry) = c1.evaluate(0, &s) {
        assert_eq!(carry, 1);
    } else {
        assert!(false);
    }
    assert!(true);
}

#[test]
fn test_alphametic_Permutation3() {
    //let chars = vec!['A', 'B', 'C'];
    //let chars = chars.into_iter().map(|c| (c, 0)).collect();
    let chars = vec!['A', 'B', 'C'];
    let mut p = Permutation3::new(chars);
    let mut pairs: Option<Vec<(char, u8)>> = p.next()
        .map(|v| v.into_iter().map(|(c, i)| (c, i as u8)).collect());
    if let Some(ref mut p) = pairs {
        p.sort();
        assert_eq!(p, &vec![('A', 0), ('B', 1), ('C', 2)]);
    } else {
        assert!(false);
    }
}
/*

#[test]
#[ignore]
fn test_alphametic_new() {
    let a = Alphametic::new("I + BB == ILL");
    let expected = vec![Term::new("I"), Term::new("BB"), Term::new("ILL")];
    assert_eq!(a.terms, expected);
}

#[test]
#[ignore]
fn test_alphametic_chars1() {
    let a = Alphametic::new("I + BB == ILL");
    let expected = vec!['I', 'B', 'L'];
    assert_eq!(a.chars(0), expected);
}

#[test]
#[ignore]
fn test_alphametic_chars2() {
    let a = Alphametic::new("I + BB == ILL");
    let expected = vec!['B', 'L'];
    assert_eq!(a.chars(1), expected);
}

#[test]
#[ignore]
fn test_alphametic_chars3() {
    let a = Alphametic::new("I + BB == ILL");
    let expected = vec!['I'];
    assert_eq!(a.chars(2), expected);
}

#[test]
#[ignore]
fn test_alphametic_chars4() {
    let a = Alphametic::new("I + BB == ILL");
    let expected = vec![];
    assert_eq!(a.chars(3), expected);
}

#[test]
#[ignore]
fn test_alphametic_evaluate_place1() {
    let mut alphametic = Alphametic::new("I + BB == ILL");
    let place = 0;
    let permutation = vec![('I', 1), ('B', 9), ('L', 0)];
    alphametic.insert(&permutation);
    let terms: Vec<&Term> = alphametic.addends2();
    let actual = alphametic.evaulate(place, terms);
    let expected = 10;
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn test_alphametic_evaluate_place2() {
    let mut alphametic = Alphametic::new("I + BB == ILL");
    println!("{}", alphametic);
    let place = 1;
    let permutation = vec![('I', 1), ('B', 9), ('L', 0)];
    alphametic.insert(&permutation);
    println!("{}", alphametic);
    let terms: Vec<&Term> = alphametic.addends2();
    let actual = alphametic.evaulate(place, terms);
    let expected = 9;
    assert_eq!(actual, expected);
}
*/



fn assert_alphametic_solution_eq(puzzle: &str, solution: &[(char, u8)]) {
    let answer = alphametics::solve(puzzle);
    let solution: HashMap<char, u8> = solution.iter().cloned().collect();
    assert_eq!(answer, Some(solution));
}

#[test]
//#[ignore]
fn test_with_three_letters() {
    assert_alphametic_solution_eq("I + BB == ILL", &[('I', 1), ('B', 9), ('L', 0)]);
}

#[test]
#[ignore]
fn test_must_have_unique_value_for_each_letter() {
    let answer = alphametics::solve("A == B");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn test_leading_zero_solution_is_invalid() {
    let answer = alphametics::solve("ACA + DD == BD");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn test_sum_must_be_wide_enough() {
    let answer = alphametics::solve("ABC + DEF == GH");
    assert_eq!(answer, None);
}

#[test]
#[ignore]
fn puzzle_with_two_digits_final_carry() {
    assert_alphametic_solution_eq(
        "A + A + A + A + A + A + A + A + A + A + A + B == BCC",
        &[('A', 9), ('B', 1), ('C', 0)],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_four_letters() {
    assert_alphametic_solution_eq("AS + A == MOM", &[('A', 9), ('S', 2), ('M', 1), ('O', 0)]);
}

#[test]
#[ignore]
fn test_puzzle_with_six_letters() {
    assert_alphametic_solution_eq(
        "NO + NO + TOO == LATE",
        &[('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_seven_letters() {
    assert_alphametic_solution_eq(
        "HE + SEES + THE == LIGHT",
        &[
            ('E', 4),
            ('G', 2),
            ('H', 5),
            ('I', 0),
            ('L', 1),
            ('S', 9),
            ('T', 7),
        ],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_eight_letters() {
    assert_alphametic_solution_eq(
        "SEND + MORE == MONEY",
        &[
            ('S', 9),
            ('E', 5),
            ('N', 6),
            ('D', 7),
            ('M', 1),
            ('O', 0),
            ('R', 8),
            ('Y', 2),
        ],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_ten_letters() {
    assert_alphametic_solution_eq(
        "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE",
        &[
            ('A', 5),
            ('D', 3),
            ('E', 4),
            ('F', 7),
            ('G', 8),
            ('N', 0),
            ('O', 2),
            ('R', 1),
            ('S', 6),
            ('T', 9),
        ],
    );
}

#[test]
#[ignore]
fn test_puzzle_with_ten_letters_and_199_addends() {
    assert_alphametic_solution_eq(
        "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
        &[
            ('A', 1),
            ('E', 0),
            ('F', 5),
            ('H', 8),
            ('I', 7),
            ('L', 2),
            ('O', 6),
            ('R', 3),
            ('S', 4),
            ('T', 9),
        ],
    );
}
