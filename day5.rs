fn retrieve_seat_id(val: &str) -> i32 {
    let row : i32 = val.char_indices()
                       .take(7)
                       .map(|(i, c)| if c=='B' { 1 << (6-i) } else { 0 })
                       .sum();
    let col : i32 = val.char_indices()
                       .skip(7)
                       .take(3)
                       .map(|(i, c)| if c=='R' { 1 << (2-(i-7)) } else { 0 })
                       .sum();
    row * 8 + col
}

fn find_missing(seats : &[i32]) -> Option<i32> {
    let one_off = seats.iter().skip(1);
    let zipped = seats.iter().zip(one_off);
    for pair in zipped {
        if (pair.1 - pair.0) > 1 {
            return Some(pair.0 + 1);
        }
    }
    None
}

fn main() {
    let test_vals = [
        "BFFFBBFRRR",
        "FFFBBBFRRR",
        "BBFFBBFRLL",
    ];

    for val in test_vals.iter() {
        println!("val: {:?} result: {:?}", val, retrieve_seat_id(val));
    }

    let real_vals = include_str!("day5.txt");

    let seat_ids = 
        real_vals.lines()
                 .map(|seat| retrieve_seat_id(seat));

    let highest = seat_ids.clone().max().unwrap();
    println!("Highest val for real is {}", highest);

    let mut sorted_seat_ids : Vec<i32> = seat_ids.collect();
    sorted_seat_ids.sort();
    println!("My seat is {}", find_missing(&sorted_seat_ids).unwrap());
}