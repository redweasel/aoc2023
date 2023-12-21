/*
--- Day 18: Lavaduct Lagoon ---

Thanks to your efforts, the machine parts factory is one of the first factories up and running since the lavafall came back. However, to catch up with the large backlog of parts requests, the factory will also need a large supply of lava for a while; the Elves have already started creating a large lagoon nearby for this purpose.

However, they aren't sure the lagoon will be big enough; they've asked you to take a look at the dig plan (your puzzle input). For example:

R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)

The digger starts in a 1 meter cube hole in the ground. They then dig the specified number of meters up (U), down (D), left (L), or right (R), clearing full 1 meter cubes as they go. The directions are given as seen from above, so if "up" were north, then "right" would be east, and so on. Each trench is also listed with the color that the edge of the trench should be painted as an RGB hexadecimal color code.

When viewed from above, the above example dig plan would result in the following loop of trench (#) having been dug out from otherwise ground-level terrain (.):

#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######

At this point, the trench could contain 38 cubic meters of lava. However, this is just the edge of the lagoon; the next step is to dig out the interior so that it is one meter deep as well:

#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######

Now, the lagoon can contain a much more respectable 62 cubic meters of lava. While the interior is dug out, the edges are also painted according to the color codes in the dig plan.

The Elves are concerned the lagoon won't be large enough; if they follow their dig plan, how many cubic meters of lava could it hold?
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    North = 0,
    East = 1,
    West = 2,
    South = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point(pub i64, pub i64);

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Mul<i64> for Point {
    type Output = Point;
    fn mul(self, rhs: i64) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl Direction {
    pub fn delta(&self) -> Point {
        match self {
            Self::North => Point(0, -1),
            Self::East => Point(1, 0),
            Self::South => Point(0, 1),
            Self::West => Point(-1, 0),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::North,
            'R' => Self::East,
            'D' => Self::South,
            'L' => Self::West,
            _ => panic!("invalid char {value}"),
        }
    }
}

#[test]
pub fn part1() {
    // idea: do a full readin of the data and then a processing step

    use std::io;

    let mut polygon = vec![Point(0, 0)];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        // Not using the color here...
        let (a, _) = input.split_once('(').unwrap();
        let (dir, steps) = a.trim().split_once(' ').unwrap();
        assert_eq!(dir.len(), 1, "Direction needs to be one letter");
        let dir: Direction = dir.chars().next().unwrap().into();
        let steps = steps.parse().expect("can't parse step number");
        polygon.push(*polygon.last().unwrap() + dir.delta() * steps);
    }
    assert_eq!(polygon.last(), polygon.first(), "The loop wasn't closed");
    let mut area = 0;
    let mut length = 0;
    for w in polygon.windows(2) {
        let (start, end) = (w[0], w[1]);
        area += start.0 * (end.1 - start.1);
        length += start.0.abs_diff(end.0) + start.1.abs_diff(end.1);
    }
    area += (length / 2) as i64 + 1; // to also include the full boundary
    println!("The area is {area} square meters");
}

/*
--- Part Two ---

The Elves were right to be concerned; the planned lagoon would be much too small.

After a few minutes, someone realizes what happened; someone swapped the color and instruction parameters when producing the dig plan. They don't have time to fix the bug; one of them asks if you can extract the correct instructions from the hexadecimal codes.

Each hexadecimal code is six hexadecimal digits long. The first five hexadecimal digits encode the distance in meters as a five-digit hexadecimal number. The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.

So, in the above example, the hexadecimal codes can be converted into the true instructions:

    #70c710 = R 461937
    #0dc571 = D 56407
    #5713f0 = R 356671
    #d2c081 = D 863240
    #59c680 = R 367720
    #411b91 = D 266681
    #8ceee2 = L 577262
    #caa173 = U 829975
    #1b58a2 = L 112010
    #caa171 = D 829975
    #7807d2 = L 491645
    #a77fa3 = U 686074
    #015232 = L 5411
    #7a21e3 = U 500254

Digging out this loop and its interior produces a lagoon that can hold an impressive 952408144115 cubic meters of lava.

Convert the hexadecimal color codes into the correct instructions; if the Elves follow this new dig plan, how many cubic meters of lava could the lagoon hold?
*/

#[test]
pub fn part2() {
    // idea: do a full readin of the data and then a processing step

    use std::io;

    let mut polygon = vec![Point(0, 0)];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let (_, b) = input.split_once('(').unwrap();
        // silly elves, lets quickly do the conversion then
        let color = b.trim_start_matches('#').trim_end_matches(')');
        let steps: i64 = i64::from_str_radix(&color[..5], 16).expect("Failed to parse color hex code for steps");
        let dir: u32 = u32::from_str_radix(&color[5..], 16).expect("Failed to parse color hex code for direction");
        // 0 means R means East
        // 1 means D means South
        // 2 means L means West
        // 3 means U means North
        let dir = match dir {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => panic!("invalid direction {dir}"),
        };
        polygon.push(*polygon.last().unwrap() + dir.delta() * steps);
    }
    assert_eq!(polygon.last(), polygon.first(), "The loop wasn't closed");
    let mut area = 0i128;
    let mut length = 0u64;
    for w in polygon.windows(2) {
        let (start, end) = (w[0], w[1]);
        area += (start.0 as i128) * ((end.1 - start.1) as i128);
        length += start.0.abs_diff(end.0) + start.1.abs_diff(end.1);
    }
    area += (length / 2) as i128 + 1; // to also include the full boundary
    println!("The area is {area} square meters");
}