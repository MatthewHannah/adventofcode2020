use std::collections::HashMap;

fn process_group_union(group : &str) -> usize {
    let mut total : Vec<char> =
        group.chars()
             .filter(|c| c.is_ascii_alphabetic())
             .collect();
    total.sort();
    total.dedup();
    total.len()
}

fn process_group_intersection(group : &str) -> usize {
    let map : HashMap<char, usize> = HashMap::new();
    let map =
    group.chars()
         .filter(|c| c.is_ascii_alphabetic())
         .fold(map, |mut map, c| {
            map.entry(c)
               .and_modify(|x| *x += 1)
               .or_insert(1);
            map
         } );
    let lines : usize = group.lines().map(|_| 1).sum();

    map.iter()
       .map(|(_, &i)| if i == lines { 1 } else { 0 })
       .sum()
}

fn process_all_groups_union(groups: &str) -> usize {
    groups.split("\n\n")
          .map(|group| process_group_union(group))
          .sum()
}

fn process_all_groups_intersection(groups: &str) -> usize {
    groups.split("\n\n")
          .map(|group| process_group_intersection(group))
          .sum()
}

fn main() {
    let test_input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    println!("test groups val is {:?}", process_all_groups_union(test_input));

    let real_input = include_str!("day6.txt");
    println!("real groups val is {:?}", process_all_groups_union(real_input));

    println!("test groups intersection is {:?}", process_all_groups_intersection(test_input));
    println!("real groups intersection is {:?}", process_all_groups_intersection(real_input));

}