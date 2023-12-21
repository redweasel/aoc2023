/*
--- Day 14: Parabolic Reflector Dish ---

You reach the place where all of the mirrors were pointing: a massive parabolic reflector dish attached to the side of another large mountain.

The dish is made up of many small mirrors, but while the mirrors themselves are roughly in the shape of a parabolic reflector dish, each individual mirror seems to be pointing in slightly the wrong direction. If the dish is meant to focus light, all it's doing right now is sending it in a vague direction.

This system must be what provides the energy for the lava! If you focus the reflector dish, maybe you can go where it's pointing and use the light to fix the lava production.

Upon closer inspection, the individual mirrors each appear to be connected via an elaborate system of ropes and pulleys to a large metal platform below the dish. The platform is covered in large rocks of various shapes. Depending on their position, the weight of the rocks deforms the platform, and the shape of the platform controls which ropes move and ultimately the focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even has a control panel on the side that lets you tilt it in one of four directions! The rounded rocks (O) will roll when the platform is tilted, while the cube-shaped rocks (#) will stay in place. You note the positions of all of the empty spaces (.) and rocks (your puzzle input). For example:

O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....

Start by tilting the lever so all of the rocks will slide north as far as they will go:

OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....

You notice that the support beams along the north side of the platform are damaged; to ensure the platform doesn't collapse, you should calculate the total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge of the platform, including the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.) So, the amount of load caused by each rock in each row is as follows:

OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1

The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support beams?
*/

#[test]
pub fn part1() {
    use std::io;
    use std::time::*;
    let start = Instant::now();

    let mut load = vec![];
    let mut obstacle = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if obstacle.len() == 0 {
            obstacle = [1].repeat(input.len());
        }
        load.push(0);
        for (i, c) in input.trim().chars().enumerate() {
            match c {
                '#' => {
                    obstacle[i] = 0;
                },
                'O' => {
                    let index = load.len() - obstacle[i];
                    obstacle[i] -= 1;
                    load[index] += 1;
                },
                '.' => (),
                _ => panic!("invalid input char {c}"),
            }
        }
        for x in obstacle.iter_mut() {
            *x += 1;
        }
    }
    let sum = load.into_iter().rev().enumerate().map(|(i, load)| (i+1) * load).sum::<usize>();
    println!("The sum of the load on the north support beams is {sum}");
    println!("Time: {:?}", Instant::now() - start);
}

/*
--- Part Two ---

The parabolic reflector dish deforms, but not in a way that focuses the beam. To do that, you'll need to move the rocks to the edges of the platform. Fortunately, a button on the side of the control panel labeled "spin cycle" attempts to do just that!

Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order.

Here's what happens in the example above after each of the first few cycles:

After 1 cycle:
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....

After 2 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O

After 3 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O

This process should work if you leave it running long enough, but you're still worried about the north support beams. To make sure they'll survive for a while, you need to calculate the total load on the north support beams after 1000000000 cycles.

In the above example, after 1000000000 cycles, the total load on the north support beams is 64.

Run the spin cycle for 1000000000 cycles. Afterward, what is the total load on the north support beams?
*/

pub fn tilt<I: Iterator<Item=I2>, I2: Iterator<Item=char>>(src: I) -> Vec<Vec<char>> {
    let mut field = vec![];
    let mut obstacle = vec![];
    for line in src {
        let len = line.size_hint().0; // trust size_hint!
        if obstacle.len() == 0 {
            obstacle = [1].repeat(len);
        }
        field.push(['.'].repeat(len));
        for (i, c) in line.enumerate() {
            match c {
                '#' => {
                    obstacle[i] = 0;
                    field.last_mut().unwrap()[i] = '#';
                },
                'O' => {
                    let index = field.len() - obstacle[i];
                    obstacle[i] -= 1;
                    field[index][i] = 'O';
                },
                '.' => (),
                _ => panic!("invalid input char {c}"),
            }
        }
        for x in obstacle.iter_mut() {
            *x += 1;
        }
    }
    field
}

#[test]
pub fn part2() {
    // as expected I need to reimplement everything, but that's fine.
    // To run it for 1000000000 cycles, there will need to be a cycle detection,
    // detecting when 4 tilts don't make a difference anymore -> abort there.
    // turns out there can be cycles of longer length...
    // find the cycle length!

    use std::io;
    use std::collections::HashMap;
    use std::time::*;
    let start = Instant::now();

    let mut field: Vec<Vec<char>> = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let line = input.trim().chars().collect::<Vec<_>>();
        if line.len() > 0 {
            field.push(line);
        }
    }
    // the first cycle is special as it doesn't need the rotation.
    field = tilt(field.into_iter().map(|row| row.into_iter()));
    let mut cycle_detect: HashMap<Vec<Vec<char>>, u64> = HashMap::new();
    let mut cycles = 1000000000 - 1;
    let mut cycle_detected = false;
    let mut i = 0u64;
    while i < cycles {
        // save for cycle detection
        if !cycle_detected && (i + 4).is_power_of_two() {
            cycle_detect.insert(field.clone(), i);
        }
        // now use the tilt function to tilt west, south, east, north
        for j in 0..4 {
            if j == 3 {
                // debug rendering
                if false {
                    for row in &field {
                        println!("{}", String::from_iter(row.iter()));
                    }
                    println!();
                }
            }
            // rotate in the iterator
            field = tilt((0..field[0].len()).map(|i| field.iter().rev().map(move |row| row[i])));
        }
        i += 1;
        if let Some(last_index) = cycle_detect.get(&field) {
            // found a cycle!
            let cycle_length = i - last_index;
            println!("cycle of length {cycle_length} detected");
            cycles -= i;
            i = 0;
            cycles %= cycle_length;
            cycle_detected = true;
        }
    }
    // rotate until the east tilt is reached
    for _ in 0..3 {
        field = tilt((0..field[0].len()).map(|i| field.iter().rev().map(move |row| row[i])));
    }
    // calculate load, but remember that the field is still rotated
    let load = field.into_iter().map(|row| row.into_iter().rev().enumerate().map(|(i, c)| (i+1) * (c == 'O') as usize).sum::<usize>()).sum::<usize>();
    println!("The sum of the load on the north support beams is {load}");
    println!("Time: {:?}", Instant::now() - start);
}