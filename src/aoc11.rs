/*
--- Day 11: Cosmic Expansion ---

You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:

...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....

The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.

Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.

In the above example, three columns and two rows contain no galaxies:

   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^

These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:

....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......

Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:

....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......

In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......

This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:

    Between galaxy 1 and galaxy 7: 15
    Between galaxy 3 and galaxy 6: 17
    Between galaxy 8 and galaxy 9: 5

In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
*/

pub fn stretch_space(pos: usize, empty_sorted: &[usize], stretch: usize) -> usize {
    pos + match empty_sorted.binary_search(&pos) {
        Ok(_) => panic!("can't expand on row/column {pos} which contains a galaxy"),
        Err(i) => i * (stretch - 1),
    }
}

#[test]
pub fn part1() {
    // idea: parse the data first
    use std::io;
    use std::collections::HashSet;

    let mut galaxies = vec![];
    let mut width = None;
    let mut y = 0;
    let mut empty_rows = vec![];
    let mut empty_columns = HashSet::new();
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if y == 0 {
            width = Some(input.len());
            // init empty_columns
            empty_columns.extend(0..width.unwrap());
        }
        else {
            assert_eq!(input.len(), width.unwrap(), "one line wasn't the same length as the previous.");
        }
        let len_before = galaxies.len();
        galaxies.extend(input.chars().enumerate().filter_map(|(x, c)| {
            match c {
                '.' => None,
                '#' => { empty_columns.remove(&x); Some((x, y)) },
                _ => panic!("unknown character {c} in input"),
            }
        }));
        if galaxies.len() == len_before {
            // no galaxies in this row -> empty row
            empty_rows.push(y);
        }
        y += 1;
    }
    // empty_rows is sorted by design
    let mut empty_columns: Vec<_> = empty_columns.into_iter().collect();
    empty_columns.sort(); // not sure which order the set outputs... -> sort
    println!("empty rows: {empty_rows:?}");
    println!("empty columns: {empty_columns:?}");
    // at this point I already now which columns and rows are empty and where the galaxies are.
    // the shortest distance length is the manhatten distance |dx|+|dy|
    // There are two options to include the space stretching
    // 1. every shortest path which crosses an empty row/colum gets 1 longer
    // 2. reposition the galaxies before computing distances
    // obviously the second is better since for the 1. it would need to run the stretch detection O(n^2) times.
    let galaxies: Vec<_> = galaxies.into_iter().map(|(x, y)|
        (stretch_space(x, &empty_columns, 2),
         stretch_space(y, &empty_rows, 2))
    ).collect();
    let mut sum = 0;
    for (i, &coord_a) in galaxies.iter().enumerate() {
        for &coord_b in galaxies[i+1..].iter() {
            sum += coord_a.0.abs_diff(coord_b.0) + coord_a.1.abs_diff(coord_b.1);
        }
    }
    println!("The sum of the shortest paths between galaxies is {sum}");
}

/*
--- Part Two ---

The galaxies are much older (and thus much farther apart) than the researcher initially estimated.

Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.

(In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)

Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
*/

#[test]
pub fn part2() {
    // wow this was trivial!
    use std::io;
    use std::collections::HashSet;

    let mut galaxies = vec![];
    let mut width = None;
    let mut y = 0;
    let mut empty_rows = vec![];
    let mut empty_columns = HashSet::new();
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if y == 0 {
            width = Some(input.len());
            // init empty_columns
            empty_columns.extend(0..width.unwrap());
        }
        else {
            assert_eq!(input.len(), width.unwrap(), "one line wasn't the same length as the previous.");
        }
        let len_before = galaxies.len();
        galaxies.extend(input.chars().enumerate().filter_map(|(x, c)| {
            match c {
                '.' => None,
                '#' => { empty_columns.remove(&x); Some((x, y)) },
                _ => panic!("unknown character {c} in input"),
            }
        }));
        if galaxies.len() == len_before {
            // no galaxies in this row -> empty row
            empty_rows.push(y);
        }
        y += 1;
    }
    // empty_rows is sorted by design
    let mut empty_columns: Vec<_> = empty_columns.into_iter().collect();
    empty_columns.sort(); // not sure which order the set outputs... -> sort
    println!("empty rows: {empty_rows:?}");
    println!("empty columns: {empty_columns:?}");
    // just add 1000000 instead of 2 for each empty row. Easy!
    let galaxies: Vec<_> = galaxies.into_iter().map(|(x, y)|
        (stretch_space(x, &empty_columns, 1000000),
         stretch_space(y, &empty_rows, 1000000))
    ).collect();
    let mut sum = 0;
    for (i, &coord_a) in galaxies.iter().enumerate() {
        for &coord_b in galaxies[i+1..].iter() {
            sum += coord_a.0.abs_diff(coord_b.0) + coord_a.1.abs_diff(coord_b.1);
        }
    }
    println!("The sum of the shortest paths between galaxies is {sum}");
}