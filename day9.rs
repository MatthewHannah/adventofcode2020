fn is_valid(num: i64, previous: &[i64]) -> bool {
    previous.iter()
            .flat_map(|i| previous.iter().map(move |j| (i,j)))
            .find_map(|(i,j)| {
                if (i != j) && (i + j == num) {
                    Some(true)
                } else {
                    None
                }
            })
            .unwrap_or(false)
}

fn first_invalid(sequence: &[i64], trailing: usize) -> Option<i64> {
    for i in trailing..sequence.len() {
        let previous = &sequence[i-trailing..i];
        if !is_valid(sequence[i], previous) {
            return Some(sequence[i])
        }
    }
    None
}

fn find_cont_sum<'a>(target: i64, sequence: &'a [i64]) -> &'a [i64] {
    for i in 0..sequence.len() {
        let mut j = i;
        let mut total = 0;
        while total < target {
            total += sequence[j];
            if total == target {
                return &sequence[i..j];
            }
            j += 1;
        }
    }
    return &sequence[0..0];
}

fn find_weakness(sequence: &[i64], trailing: usize) -> i64 {
    let invalid = first_invalid(sequence, trailing).expect("no invalid");
    let cont_seq = find_cont_sum(invalid, sequence);
    let min = cont_seq.iter().min().expect("some sequence");
    let max = cont_seq.iter().max().expect("some sequence");
    min + max
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines()
         .map(|s| s.trim().parse::<i64>().expect("int input"))
         .collect()
}

fn main() {
    let test_input = "\
    35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576";

    println!("is valid {:?}", is_valid(10, &[1,2,6,3,4,5]));
    let test_input = parse_input(&test_input);
    println!("first invalid in test: {}", first_invalid(&test_input, 5).expect("no invalid"));

    let real_input = parse_input(&include_str!("day9.txt"));
    println!("first invalid in real: {}", first_invalid(&real_input, 25).expect("no invalid"));

    println!("weakness in test: {}", find_weakness(&test_input, 5));
    println!("weakness in real: {}", find_weakness(&real_input, 25));
}