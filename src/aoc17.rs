/*
--- Day 17: Clumsy Crucible ---

The lava starts flowing rapidly once the Lava Production Facility is operational. As you leave, the reindeer offers you a parachute, allowing you to quickly reach Gear Island.

As you descend, your bird's-eye view of Gear Island reveals why you had trouble finding anyone on your way up: half of Gear Island is empty, but the half below you is a giant factory city!

You land near the gradually-filling pool of lava at the base of your new lavafall. Lavaducts will eventually carry the lava throughout the city, but to make use of it immediately, Elves are loading it into large crucibles on wheels.

The crucibles are top-heavy and pushed by hand. Unfortunately, the crucibles become very difficult to steer at high speeds, and so it can be hard to go in a straight line for very long.

To get Desert Island the machine parts it needs as soon as possible, you'll need to find the best way to get the crucible from the lava pool to the machine parts factory. To do this, you need to minimize heat loss while choosing a route that doesn't require the crucible to go in a straight line for too long.

Fortunately, the Elves here have a map (your puzzle input) that uses traffic patterns, ambient temperature, and hundreds of other parameters to calculate exactly how much heat loss can be expected for a crucible entering any particular city block.

For example:

2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533

Each city block is marked by a single digit that represents the amount of heat loss if the crucible enters that block. The starting point, the lava pool, is the top-left city block; the destination, the machine parts factory, is the bottom-right city block. (Because you already start in the top-left block, you don't incur that block's heat loss unless you leave that block and then return to it.)

Because it is difficult to keep the top-heavy crucible going in a straight line for very long, it can move at most three blocks in a single direction before it must turn 90 degrees left or right. The crucible also can't reverse direction; after entering each city block, it may only turn left, continue straight, or turn right.

One way to minimize heat loss is this path:

2>>34^>>>1323
32v>>>35v5623
32552456v>>54
3446585845v52
4546657867v>6
14385987984v4
44578769877v6
36378779796v>
465496798688v
456467998645v
12246868655<v
25465488877v5
43226746555v>

This path never moves more than three consecutive blocks in the same direction and incurs a heat loss of only 102.

Directing the crucible from the lava pool to the machine parts factory, but not moving more than three consecutive blocks in the same direction, what is the least heat loss it can incur?
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    North = 0,
    East = 1,
    West = 2,
    South = 3,
}

impl Direction {
    pub fn is_opposite(&self, other: &Direction) -> bool {
        // Trick: I ordered the Direction names such that this becomes simple
        (*self as u8 + *other as u8) == 3
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
pub enum PathLength {
    GreaterThan(u32), // lower bound
    Exactly(u32), // best path known
    Blocked,
}

pub struct Memo {
    width: usize,
    grid: Vec<Vec<((Direction, u32), (PathLength, Option<(Direction, u32)>))>>,
}

impl Memo {
    pub fn new(width: usize, height: usize) -> Memo {
        Memo { width, grid: (0..width*height).map(|_| Vec::with_capacity(8)).collect() }
    }

    pub fn get(&self, key: &CrucibleState) -> Option<&(PathLength, Option<(Direction, u32)>)> {
        let bucket = &self.grid[key.pos.1 as usize * self.width + key.pos.0 as usize];
        // search for a fitting condition in the bucket
        for (b_key, b_value) in bucket {
            if b_key.0 == key.dir && b_key.1 == key.momentum {
                return Some(b_value);
            }
        }
        None
    }

    /// insert a key-value pair into the memo.
    /// NOTE: make sure to always insert the key with the maximal allowed momentum.
    pub fn insert(&mut self, key: &CrucibleState, value: (PathLength, Option<(Direction, u32)>)) {
        //assert!(momentum_range.contains(&key.momentum));
        let bucket = &mut self.grid[key.pos.1 as usize * self.width + key.pos.0 as usize];
        let new_b_key = (key.dir, key.momentum);
        for (b_key, b_value) in bucket.iter_mut() {
            if b_key == &new_b_key {
                *b_value = value;
                return;
            }
        }
        bucket.push((new_b_key, value));
    }
}

pub struct Crucible {
    pub min_steps_to_turn: u32,
    pub max_steps_to_turn: u32,
    pub grid: Vec<Vec<u32>>,
    pub memo: Memo,
    pub end: Point,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct CrucibleState {
    pos: Point,
    dir: Direction,
    momentum: u32,
}

impl Crucible {
    pub fn next(&self, state: &CrucibleState, dir: &Direction) -> Option<CrucibleState> {
        // the crucible can't turn around
        if dir.is_opposite(&state.dir) {
            return None;
        }
        // The crucible can't move more than a couple times in one direction
        if dir == &state.dir && state.momentum >= self.max_steps_to_turn {
            return None;
        }
        if state.momentum != 0 && dir != &state.dir && state.momentum < self.min_steps_to_turn {
            return None;
        }
        Some(CrucibleState {
            pos: state.pos + dir.delta(),
            dir: *dir,
            momentum: if dir == &state.dir { state.momentum + 1 } else { 1 }
        })
    }

    pub fn has_next_options(&self, state: &CrucibleState) -> bool {
        // There is this one case where there is guaranteed only one next direction, so I don't need to keep memos for this
        !(state.momentum != 0 && state.momentum < self.min_steps_to_turn)
    }

    pub fn find_shortest_path(&mut self, start: &CrucibleState, max_length: u32) -> (PathLength, Option<(Direction, u32)>) {
        let mut distance = max_length;
        let use_memo = self.has_next_options(&start);
        // check if the zero path (start == end) is requested
        if start.pos == self.end {
            if start.momentum >= self.min_steps_to_turn {
                return (PathLength::Exactly(0), Some((start.dir, 0)));
            }
            else {
                // this makes the crucible crash/overshoot
                return (PathLength::Blocked, None);
            }
        }
        // check memoized results first
        if use_memo {
            if let Some(value) = self.memo.get(start) {
                if let PathLength::GreaterThan(max_computed_path_len) = value.0 {
                    if max_computed_path_len >= distance {
                        return *value;
                    }
                }
                else {
                    return *value;
                }
            }
        }
        // add to memoization table to mark it as being currently processed
        // (if a path has a perfect state loop, it is not the shortest!)
        if use_memo {
            self.memo.insert(start, (PathLength::Blocked, None));
        }
        // calculate distance
        let width = self.grid[0].len();
        let height = self.grid.len();
        // best_dir contains the best direction and the length of the run without changing direction afterwards
        let mut best_dir = None;
        // find directions to explore
        for dir in [Direction::South, Direction::East, Direction::West, Direction::North] {
            // make the step
            if let Some(next) = self.next(start, &dir) {
                // the crucible can't go out of bounds (would be funny if it could)
                if !next.pos.in_bounds(width, height) {
                    continue;
                }
                // check if the crucible reaches the end
                let dir_length = self.grid[next.pos.1 as usize][next.pos.0 as usize];
                // heuristic for minimal length to the end
                let to_end = next.pos - self.end;
                let min_length = dir_length + (to_end.0.abs() + to_end.1.abs()) as u32;
                // if there is a chance for improving the current best path, try it.
                if min_length < distance {
                    let (path_len, next_dir) = self.find_shortest_path(&next, 
                        distance - dir_length);
                    if let PathLength::Exactly(mut d) = path_len {
                        let next_dir = next_dir.unwrap();
                        d = d + dir_length;
                        if d <= distance {
                            distance = d;
                            best_dir = Some((dir, if next_dir.0 == dir { next_dir.1 + 1 } else { 1 }));
                        }
                    }
                }
            }
        }
        let result = if best_dir.is_some() {
            (PathLength::Exactly(distance), best_dir)
        }
        else {
            // In this case the algorithm decided to not find the best path,
            // so the only thing known is, that this is not part of the problem at this stage.
            (PathLength::GreaterThan(distance), best_dir)
        };
        if use_memo {
            self.memo.insert(start, result);
        }
        result
    }

    pub fn print_shortest_path(&mut self, start: &CrucibleState) {
        let mut state = *start;
        loop {
            let (len, best_dir) = self.find_shortest_path(&state, u32::MAX);
            let best_dir = best_dir.unwrap().0;
            if let PathLength::Exactly(len) = len {
                println!("{:?} {best_dir:?} {len}", state.pos);
                state = self.next(&state, &best_dir).unwrap();
                if state.pos == self.end {
                    break;
                }
            }
            else {
                panic!("walked unknown path...");
            }
        }
    }
}

#[test]
pub fn part1() {
    // This is (almost) a classical graph problem. The goal is finding the shortest path.
    // There is just one complication! The options for where to go next depend on the past of the path!
    // This is not allowed for classical path finding algorithms.
    // So instead do another dfs with memoization.
    // This will result in a very similar algorithm to the classical flood fill path finding.
    // I can even add a heuristic into the dfs to sort the search directions to prefer going to the goal.
    // NOTE: the memo depends on the grid, so I won't put it into a static!

    // What I missed:
    // Dijkstra does still work.
    // One can still step carefully from the start, always taking the step that produces the smallest total pathlength.
    // Then one will still find the end first in the smallest path.
    // The only difference to the classical path finding is, that the nodes visited will include direction and momentum.
    // I'm not going to change my solution now though. This worked.

    // my solution was 1138

    use std::io;

    let mut grid = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let line: Vec<u32> = input.chars().map(|c| c.to_digit(10).expect("expected number")).collect();
        grid.push(line);
    }
    // do the shortest path calculation
    let width = grid[0].len();
    let height = grid.len();
    let end = Point(width as isize - 1, height as isize - 1);
    let mut crucible = Crucible {
        max_steps_to_turn: 3,
        min_steps_to_turn: 1,
        end,
        grid,
        memo: Memo::new(width, height),
    };
    let start = CrucibleState { pos: Point(0, 0), dir: Direction::East, momentum: 0 };
    let (shortest, _) = crucible.find_shortest_path(&start, u32::MAX);
    // stuff in the memo:
    println!("area: {}", width * height);
    /*for ((point, dir, momentum), (d, best_dir)) in memo.iter() {
        if let PathLength::Exactly(d) = d {
            println!("{point:?} {dir:?} {momentum} -> {d:?} {best_dir:?}")
        }
    }*/
    println!("The shortest path looses {shortest:?} arbitrary heat units.");
    // The path is
    //crucible.print_shortest_path(start);
}

/*
--- Part Two ---

The crucibles of lava simply aren't large enough to provide an adequate supply of lava to the machine parts factory. Instead, the Elves are going to upgrade to ultra crucibles.

Ultra crucibles are even more difficult to steer than normal crucibles. Not only do they have trouble going in a straight line, but they also have trouble turning!

Once an ultra crucible starts moving in a direction, it needs to move a minimum of four blocks in that direction before it can turn (or even before it can stop at the end). However, it will eventually start to get wobbly: an ultra crucible can move a maximum of ten consecutive blocks without turning.

In the above example, an ultra crucible could follow this path to minimize heat loss:

2>>>>>>>>1323
32154535v5623
32552456v4254
34465858v5452
45466578v>>>>
143859879845v
445787698776v
363787797965v
465496798688v
456467998645v
122468686556v
254654888773v
432267465553v

In the above example, an ultra crucible would incur the minimum possible heat loss of 94.

Here's another example:

111111111111
999999999991
999999999991
999999999991
999999999991

Sadly, an ultra crucible would need to take an unfortunate path like this one:

1>>>>>>>1111
9999999v9991
9999999v9991
9999999v9991
9999999v>>>>

This route causes the ultra crucible to incur the minimum possible heat loss of 71.

Directing the ultra crucible from the lava pool to the machine parts factory, what is the least heat loss it can incur?
*/

/*
flipped example

19999
19999
19999
19999
19999
19999
19999
19999
19999
19999
19999
11111
*/

#[test]
pub fn part2() {
    // my solution is 1312
    use std::io;

    let mut grid = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let line: Vec<u32> = input.chars().map(|c| c.to_digit(10).expect("expected number")).collect();
        grid.push(line);
    }
    // do the shortest path calculation
    let width = grid[0].len();
    let height = grid.len();
    let end = Point(width as isize - 1, height as isize - 1);
    let mut crucible = Crucible {
        max_steps_to_turn: 10,
        min_steps_to_turn: 4,
        end,
        grid,
        memo: Memo::new(width, height),
    };
    let start = CrucibleState { pos: Point(0, 0), dir: Direction::East, momentum: 0 };
    let (shortest, _) = crucible.find_shortest_path(&start, u32::MAX);
    // stuff in the memo:
    println!("area: {}", width * height);
    println!("The shortest path looses {shortest:?} arbitrary heat units.");
    //crucible.print_shortest_path(start);
}