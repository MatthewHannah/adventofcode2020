use std::fs::File;
use std::io::Read;
use std::io::BufReader;

fn find_result(in_vec : &[i32]) -> i32 {
    let mut result = 0;
    for val1 in in_vec.iter() {
        for val2 in in_vec.iter() {
            if val1 + val2 == 2020 {
                result = val1 * val2;
                break;
            }
        }
    }
    result
}

fn find_second_result(in_vec : &[i32]) -> i32 {
    let mut result = 0;
    for val1 in in_vec.iter() {
        for val2 in in_vec.iter() {
            for val3 in in_vec.iter() {
                if val1 + val2 + val3 == 2020 {
                    result = val1 * val2 * val3;
                    break;
                }
            }
        }
    }
    result
}

fn read_from_file(filename : &str) -> Vec<i32> {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    let mut contents = String::new();
    buf.read_to_string(&mut contents).unwrap();

    contents.lines().map(|val| val.parse::<i32>().unwrap()).collect()
}

fn main() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    println!("Result is : {}", find_result(&input));
    let file_input = read_from_file("day1.txt");
    println!("Result from file is : {}", find_result(&file_input));
    println!("Result 2 is : {}", find_second_result(&input));
    println!("Result 2 from file is : {}", find_second_result(&file_input));
}