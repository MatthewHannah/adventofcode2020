fn find_earliest_bus(timestamp: i32, buses: &[i32]) -> Option<(i32, i32)> {
    buses.iter()
         .map(|b| (b, b - (timestamp % b)))
         .min_by_key(|(_b, val)| *val)
         .map(|(&b, val)| (b, val))
}

fn parse_input_part1(lines: &str) -> (i32, Vec<i32>) {
    let mut lines = lines.lines();
    let timestamp = 
        lines.next().expect("line 1").trim().parse::<i32>().expect("Valid timestamp");
    let buses = 
    lines.next().expect("line 2")
         .trim()
         .split(',')
         .map(|s| match s {
           "x" => None,
           x   => {
               if let Ok(t) = x.parse::<i32>() {
                   Some(t)
               } else {
                   panic!("Invalid input")
               }
           }
         })
         .filter_map(|x| x)
         .collect();
    (timestamp, buses)
}

fn find_result_part1(input: &str) -> i32 {
    let (timestamp, buses) = parse_input_part1(&input);
    let (bus, wait) = find_earliest_bus(timestamp, &buses).expect("bus is there");
    bus * wait
}

fn parse_input_part2(input: &str) -> Vec<Option<i64>> {
    let mut lines = input.lines();
    let _timestamp = lines.next().expect("line 1");
    lines.next().expect("line 2")
         .trim()
         .split(',')
         .map(|s| match s {
           "x" => None,
           x   => {
               if let Ok(t) = x.parse::<i64>() {
                   Some(t)
               } else {
                   panic!("Invalid input")
               }
           }
         })
         .collect()
}

fn find_first_timestamp(buses: &[Option<i64>]) -> i64 {
    let mut buses : Vec<(i64, i64)> = 
        buses.iter()
             .enumerate()
             .filter_map(|(i,x)| x.map(|x| (x, i as i64)))
             .collect();
    buses.sort_by_key(|(a, _rem_a)| *a);
    buses.reverse();
    println!("{:?}", buses);

    let mut val = buses[0].0 - buses[0].1;
    let mut incr = 1;
    for (&(new_incr, _), &(modulus, target)) in buses[0..].iter().zip(buses[1..].iter()) {
        incr *= new_incr;
        let adjusted_target = (modulus - target).rem_euclid(modulus);
        println!("val: {}, incr: {}, modulus: {}, target: {}, adjusted_target: {}", val, incr, modulus, target, adjusted_target);
        while val % modulus != adjusted_target {
            val += incr;
        }
    }
    val
}

fn find_result_part2(input: &str) -> i64 {
    find_first_timestamp(&parse_input_part2(input))
}

fn main() {
    let test_input = "\
    939
    7,13,x,x,59,x,31,19";

    println!("test result for pt 1 is {}", find_result_part1(&test_input));

    let real_input = include_str!("day13.txt");
    println!("real result for pt 1 is {}", find_result_part1(&real_input));

    let test_inputs = [
        "0\n17,x,13,19",
        "0\n67,7,59,61",
        "0\n67,x,7,59,61",
        "0\n67,7,x,59,61",
        "0\n1789,37,47,1889",
        "0\n7,13,x,x,59,x,31,19",
    ];
    for input in test_inputs.iter() {
        println!("result for {:?} is {}", input, find_result_part2(input))
    }
    println!("result for real input is {}", find_result_part2(&real_input));
}