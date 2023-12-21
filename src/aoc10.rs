/*
--- Day 10: Pipe Maze ---

You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is densely packed with pipes; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....

If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....

In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF

In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ

If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....

You can count the distance each tile in the loop is from the starting point like this:

.....
.012.
.1.3.
.234.
.....

In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...

Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn delta_coords(&self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Field {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("invalid char {value}"),
        }
    }
}

impl Field {
    pub fn next(&self, incoming: Direction) -> Direction {
        self.try_next(incoming).expect("invalid incoming direction")
    }
    pub fn try_next(&self, incoming: Direction) -> Option<Direction> {
        match self {
            Self::Vertical => match incoming {
                Direction::North | Direction::South => Some(incoming),
                _ => None,
            },
            Self::Horizontal => match incoming {
                Direction::East | Direction::West => Some(incoming),
                _ => None,
            },
            Self::NorthEast => match incoming {
                Direction::South => Some(Direction::East),
                Direction::West => Some(Direction::North),
                _ => None,
            },
            Self::NorthWest => match incoming {
                Direction::South => Some(Direction::West),
                Direction::East => Some(Direction::North),
                _ => None,
            },
            Self::SouthEast => match incoming {
                Direction::North => Some(Direction::East),
                Direction::West => Some(Direction::South),
                _ => None,
            },
            Self::SouthWest => match incoming {
                Direction::North => Some(Direction::West),
                Direction::East => Some(Direction::South),
                _ => None,
            },
            _ => None,
        }
    }
}

#[test]
pub fn part1() {
    // idea: do a full readin of the data and then a processing step

    use std::io;

    let mut field = vec![];
    let mut start = (0, 0);
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let line: Vec<Field> = input.chars().enumerate().map(|(i, c)| {
            let f = c.into();
            if f == Field::Start {
                start = (i, field.len());
            }
            f
        }).collect();
        field.push(line);
    }
    let width = field[0].len() as isize;
    let height = field.len() as isize;
    // Now I got the start and the whole field.
    // First find the two directions that the animal could have run.
    let mut two_dirs = [Direction::North; 2];
    let mut index = 0;
    for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
        let delta = dir.delta_coords();
        let next = (start.0 as isize + delta.0, start.1 as isize + delta.1);
        if next.0 >= 0 && next.1 >= 0 && next.0 < width && next.1 < height {
            // next field is inside the map
            if let Some(_) = field[next.1 as usize][next.0 as usize].try_next(dir) {
                two_dirs[index] = dir;
                index += 1;
            }
        }
    }
    assert!(index == 2);
    println!("starting at {start:?} in the {width}x{height} field.");
    println!("{two_dirs:?}");
    // now start going around the loop in both directions simulataneously and stop when the heads are equal again
    let mut heads = [start; 2];
    let mut count = 0;
    loop {
        for (i, dir) in two_dirs.into_iter().enumerate() {
            let delta = dir.delta_coords();
            let next = ((heads[i].0 as isize + delta.0) as usize, (heads[i].1 as isize + delta.1) as usize);
            // no need to check validity here, as the input should not lead us outside the field
            two_dirs[i] = field[next.1][next.0].next(dir);
            heads[i] = next;
        }
        count += 1;
        //println!("{heads:?}");
        if heads[0] == heads[1] {
            // reached the farthest point!
            break;
        }
    }
    println!("The farthest point has distance {count}");
}

/*
--- Part Two ---

You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:

...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........

The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same loop again with those regions marked:

...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....

In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop and O is still outside the loop:

..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........

In both of the above examples, 4 tiles are enclosed by the loop.

Here's a larger example:

.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...

The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):

OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO

In this larger example, 8 tiles are enclosed by the loop.

Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

Here are just the tiles that are enclosed by the loop marked with I:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

In this last example, 10 tiles are enclosed by the loop.

Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?
*/

#[test]
pub fn part2() {
    // idea: use some trickery with overlapping rectangles to count the area

    use std::io;

    let mut field = vec![];
    let mut start = (0, 0);
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let line: Vec<Field> = input.chars().enumerate().map(|(i, c)| {
            let f = c.into();
            if f == Field::Start {
                start = (i, field.len());
            }
            f
        }).collect();
        field.push(line);
    }
    let width = field[0].len() as isize;
    let height = field.len() as isize;
    // Now I got the start and the whole field.
    // First find the two directions that the animal could have run.
    let mut current_dir = Direction::North;
    for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
        let delta = dir.delta_coords();
        let next = (start.0 as isize + delta.0, start.1 as isize + delta.1);
        if next.0 >= 0 && next.1 >= 0 && next.0 < width && next.1 < height {
            // next field is inside the map
            if let Some(_) = field[next.1 as usize][next.0 as usize].try_next(dir) {
                current_dir = dir;
                break;
            }
        }
    }
    println!("starting at {start:?} in the {width}x{height} field.");
    // now start going around the loop in one direction
    let mut head = start;
    let mut count = 0;
    let mut area = 0;
    loop {
        let delta = current_dir.delta_coords();
        let next = ((head.0 as isize + delta.0) as usize, (head.1 as isize + delta.1) as usize);
        // no need to check validity here, as the input should not lead us outside the field
        head = next;
        count += 1;
        // for each horizontal step, add a rectangle
        area += head.1 as isize * delta.0;
        if head == start {
            // reached the farthest point!
            break;
        }
        current_dir = field[next.1][next.0].next(current_dir);
    }
    // could run around the loop in the wrong direction -> negative area
    area = area.abs();
    // the area above counts also part of the path
    // subtract that part of the path:
    area -= count / 2 - 1;
    println!("The farthest point has distance {}", count/2);
    println!("The loop contains an area of {area}");
}