use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines()
         .map(|s| s.trim().parse::<u64>().expect("num"))
         .collect()
}

fn count_diff(input: &[u64]) -> (usize, usize, usize) {
    let mut input = Vec::from(input);
    input.sort();
    input.insert(0, 0); // first element
    input.push(input.last().unwrap() + 3); // device
    let off_by_one = input.iter().skip(1);
    input.iter()
         .zip(off_by_one)
         .map(|(a, b)| {
             b - a 
         })
         .fold((0,0,0), |acc, diff| match diff {
             1 => (acc.0+1, acc.1, acc.2),
             2 => (acc.0, acc.1+1, acc.2),
             3 => (acc.0, acc.1, acc.2+1),
             x => panic!("unexpected diff {}", x),
         })
}

fn find_result(input: &[u64]) -> usize {
    let diffs = count_diff(input);
    diffs.0 * diffs.2
}

type Memo = HashMap<u64, usize>;

fn find_recursive(curr: u64, remaining: &[u64], memo: &mut Memo) -> usize {
    if remaining.len() == 0 {
        memo.insert(curr, 1);
        1
    } else {
        match memo.get(&curr) {
            Some(&x) => x,
            None => {
                let mut total = 0;
                for (i, &num) in remaining.iter().enumerate() {
                    if num - curr <= 3 {
                        total += find_recursive(num, &remaining[i+1..], memo)
                    } else {
                        break
                    }
                }
                memo.insert(curr, total);
                total
            },
        }
    }
}

fn find_all_combinations(input: &[u64]) -> usize {
    let mut input = Vec::from(input);
    input.sort();
    find_recursive(0, &input, &mut HashMap::new())
}

fn main() {
    let test_input = "\
    16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4";
    let test_input = parse_input(&test_input);

    let test_input2 = "\
    28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3";
    let test_input2 = parse_input(&test_input2);

    println!("{:?}", count_diff(&test_input));
    println!("{:?}", count_diff(&test_input2));

    let real_input = parse_input(&include_str!("day10.txt"));
    println!("counts: {:?}", count_diff(&real_input));
    println!("{:?}", find_result(&real_input));

    println!("combination count of test {:?}", find_all_combinations(&test_input));
    println!("combination count of test2 {:?}", find_all_combinations(&test_input2));
    println!("combination count of real {:?}", find_all_combinations(&real_input));

}