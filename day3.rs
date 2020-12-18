type Map = Vec<Vec<bool>>; // where true == tree

fn transform_map(map_str : &str) -> Map {
    map_str.lines()
           .map(|line| {
              line.chars()
                  .map(|x| x == '#')
                  .collect::<Vec<bool>>()
           })
           .collect()
}

fn regularize_point(
    point : (usize, usize),
    dimension : (usize, usize),
) -> (usize, usize) {
    (point.0, point.1 % dimension.1)
}

fn count_trees(
    incr_dir : (usize, usize),
    map : &Map,
) -> usize {
    let mut count = 0;
    let initial_pos = (0,0);

    let mut pos = initial_pos;
    while pos.0 < map.len() {
        // add current count
        if map[pos.0][pos.1] {
            count += 1
        }
        pos = regularize_point(
            (pos.0 + incr_dir.0, pos.1 + incr_dir.1), 
            (map.len(), map[0].len()),
        );
    }
    count
}

fn main() {
    let input = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#\
";

    let real_input = include_str!("day3.txt");

    let map = transform_map(&input);

    println!("Result : {}", count_trees((1, 3), &map));

    let map = transform_map(&real_input);
    println!("Real result : {}", count_trees((1, 3), &map));

    let slopes = vec![(1,1), (1,3), (1,5), (1,7), (2,1)];
    let total = slopes.iter()
                      .map(|slope| {
                        let result = count_trees(*slope, &map);
                        println!("total for this go is {}", result);
                        result 
                      })
                      .fold(1, |acc, x| acc * x);
    println!("Second total result: {}", total);

}