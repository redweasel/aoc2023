/*
--- Day 16: The Floor Will Be Lava ---

With the beam of light completely focused somewhere, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input). For example:

.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....

The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:

    If the beam encounters empty space (.), it continues in the same direction.
    If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
    If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
    If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.

Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or split in it.

In the above example, here is how the beam of light bounces around the contraption:

>|<<<\....
|v-.\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
<->-/vv|..
.|<<<2-|.\
.v//.|.v..

Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is energized (#) or not (.):

######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..

Ultimately, in this example, 46 tiles become energized.

The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, how many tiles end up being energized?
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point(pub isize, pub isize);

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Point {
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.0 >= 0 && self.1 >= 0 && (self.0 as usize) < width && (self.1 as usize) < height
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Field {
    Vertical,
    Horizontal,
    Mirror,
    BackMirror,
    Empty,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '/' => Self::Mirror,
            '\\' => Self::BackMirror,
            '.' => Self::Empty,
            _ => panic!("invalid char {value}"),
        }
    }
}

impl Field {
    pub fn next(&self, incoming: Direction) -> (Direction, Option<Direction>) {
        match self {
            Self::Vertical => match incoming {
                Direction::North | Direction::South => (incoming, None),
                _ => (Direction::North, Some(Direction::South)),
            },
            Self::Horizontal => match incoming {
                Direction::East | Direction::West => (incoming, None),
                _ => (Direction::East, Some(Direction::West)),
            },
            Self::Mirror => match incoming {
                Direction::North => (Direction::East, None),
                Direction::East => (Direction::North, None),
                Direction::South => (Direction::West, None),
                Direction::West => (Direction::South, None),
            },
            Self::BackMirror => match incoming {
                Direction::North => (Direction::West, None),
                Direction::West => (Direction::North, None),
                Direction::South => (Direction::East, None),
                Direction::East => (Direction::South, None),
            },
            Self::Empty => (incoming, None),
        }
    }
}

pub fn shine(field: &Vec<Vec<Field>>, start: (Point, Direction)) -> usize {
    let width = field[0].len();
    let height = field.len();
    // setup the field for the light
    let mut light: Vec<_> = (0..height).map(|_| [[false; 4]].repeat(width)).collect();
    let mut heads = vec![start];
    loop {
        let mut new_heads = vec![];
        for (point, dir) in heads {
            // check if the head is valid
            if !point.in_bounds(width, height) {
                continue;
            }
            // check if a head has been here before
            if !light[point.1 as usize][point.0 as usize][dir as usize] {
                // mark light
                light[point.1 as usize][point.0 as usize][dir as usize] = true;
                // get new heads
                let (next1, optional_next2) = field[point.1 as usize][point.0 as usize].next(dir);
                new_heads.push((point + next1.delta(), next1));
                if let Some(next2) = optional_next2 {
                    new_heads.push((point + next2.delta(), next2));
                }
            }
        }
        if new_heads.len() == 0 {
            break;
        }
        heads = new_heads;
    }

    // Now evaluate the board
    light.into_iter().flatten().map(|light_dir| light_dir.into_iter().any(|x| x) as usize).sum::<usize>()
}

#[test]
pub fn part1() {
    // idea: do a full readin of the data and then a processing step
    // for the processing, save for each tile which directions have light moving in it
    // do a stepwise "simulation" to fill up the grid with light

    use std::io;

    let mut field = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let line: Vec<Field> = input.chars().map(|c| c.into()).collect();
        field.push(line);
    }
    
    // Now I got the whole field. Time to shine!
    let sum = shine(&field, (Point(0, 0), Direction::East));
    println!("A total of {sum} squares are covered by light.");
}

/*
--- Part Two ---

As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from any edge tile and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)

So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that energizes as many tiles as possible.

In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:

.|<2<\....
|v-v\^....
.v.v.|->>>
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
<-2-/vv|..
.|<<<2-|.\
.v//.|.v..

Using this configuration, 51 tiles are energized:

.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..

Find the initial beam configuration that energizes the largest number of tiles; how many tiles are energized in that configuration?
*/

#[test]
pub fn part2() {
    // idea: do a full readin of the data and then a processing step
    // for the processing, save for each tile which directions have light moving in it
    // do a stepwise "simulation" to fill up the grid with light

    use std::io;

    let mut field = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let line: Vec<Field> = input.chars().map(|c| c.into()).collect();
        field.push(line);
    }
    
    // Now I got the whole field. Time to shine for real!
    let width = field[0].len();
    let height = field.len();
    let mut max = 0;
    for i in 0..height {
        max = max.max(shine(&field, (Point(0, i as isize), Direction::East)));
        max = max.max(shine(&field, (Point((width-1) as isize, i as isize), Direction::West)));
    }
    for i in 0..width {
        max = max.max(shine(&field, (Point(i as isize, 0), Direction::South)));
        max = max.max(shine(&field, (Point(i as isize, (height-1) as isize), Direction::North)));
    }
    println!("A total of {max} squares can be covered light.");
}