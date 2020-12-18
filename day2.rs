use std::fs::File;
use std::io::Read;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
struct Policy(usize, usize, char);

fn check_password(policy : Policy, password : &str) -> bool {
    let Policy(min, max, character) = policy;
    let count = password.chars().filter(|x| *x == character).count();
    //println!("Count for {:?} and policy {:?} is {}", password, policy, count);
    min <= count && count <= max
}

fn check_password2(policy : Policy, password : &str) -> bool {
    let Policy(first, second, character) = policy;
    let password : Vec<char> = password.chars().collect();
    (password[first-1] == character) ^ (password[second-1] == character)
}

fn extract_policy(line : &str) -> (Policy, &str) {
    let mut pos = line.split_ascii_whitespace();
    let min_max = pos.next().unwrap();
    let character = pos.next().unwrap();
    let password = pos.next().unwrap();

    let mut pos = min_max.split('-');
    let min = pos.next().unwrap().parse::<usize>().unwrap();
    let max = pos.next().unwrap().parse::<usize>().unwrap();

    let mut pos = character.split(':');
    let character = pos.next().unwrap().chars().next().unwrap();

    (Policy(min, max, character), &password)
}

fn check_lines(lines : &str) -> usize {
    lines.lines()
         .filter( |line| {
            let (policy, password) = extract_policy(line);
            check_password(policy, password) })
         .count()
}

fn check_lines2(lines : &str) -> usize {
    lines.lines()
         .filter( |line| {
            let (policy, password) = extract_policy(line);
            check_password2(policy, password) })
         .count()
}

fn check_file(filename : &str) -> usize {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    let mut contents = String::new();
    buf.read_to_string(&mut contents).unwrap();

    check_lines(&contents)
}

fn check_file2(filename : &str) -> usize {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    let mut contents = String::new();
    buf.read_to_string(&mut contents).unwrap();

    check_lines2(&contents)
}


fn main() {
    let input =
"1-3 a: abcde
 1-3 b: cdefg
 2-9 c: ccccccccc";

    println!("Test input count {:?}", check_lines(&input));
    println!("Input count {:?}", check_file("day2.txt"));
    println!("Input count for 2 {:?}", check_file2("day2.txt"));
}