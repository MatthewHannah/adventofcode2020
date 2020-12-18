use std::collections::HashMap;

type Bags<'a> = HashMap<&'a str, Bag<'a>>;

#[derive(Debug, Default)]
struct Bag<'a> {
}

fn process_line<'a>(line: &'a str, bags: &'a mut Bags<'a>) {
    let mut contain_split = line.split("contain");
    let first_half = contain_split.next().unwrap();
    let second_half = contain_split.next().unwrap();

    let owner = first_half.split("bags").next().unwrap().trim();

    let children = second_half.split(",");
    for child in children {
        let child = child.trim();
        if child == "no other bags." {
            continue
        }
        let mut child = child.split(" ");
        let num : usize = child.next().unwrap().parse().unwrap();
        let label : &str = child.next().unwrap();

        let mut child = bags.entry(label).or_default();
    }
}

fn main() {
    let test_input = "\
    light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags.";

    for line in test_input.lines() {
        process_line(line, &mut HashMap::new());
    }
}