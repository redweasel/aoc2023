/*
--- Day 21: Step Counter ---

You manage to catch the airship right as it's dropping someone else off on their all-expenses-paid trip to Desert Island! It even helpfully drops you off near the gardener and his massive farm.

"You got the sand flowing again! Great work! Now we just need to wait until we have enough sand to filter the water for Snow Island and we'll have snow again in no time."

While you wait, one of the Elves that works with the gardener heard how good you are at solving problems and would like your help. He needs to get his steps in for the day, and so he'd like to know which garden plots he can reach with exactly his remaining 64 steps.

He gives you an up-to-date map (your puzzle input) of his starting position (S), garden plots (.), and rocks (#). For example:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........

The Elf starts at the starting position (S) which also counts as a garden plot. Then, he can take one step north, south, east, or west, but only onto tiles that are garden plots. This would allow him to reach any of the tiles marked O:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#O#....
.##.OS####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........

Then, he takes a second step. Since at this point he could be at either tile marked O, his second step would allow him to reach any garden plot that is one step north, south, east, or west of any tile that he could have reached after the first step:

...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........

After two steps, he could be at any of the tiles marked O above, including the starting position (either by going north-then-south or by going west-then-east).

A single third step leads to even more possibilities:

...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.OS####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........

He will continue like this until his steps for the day have been exhausted. After a total of 6 steps, he could reach any of the garden plots marked O:

...........
.....###.#.
.###.##.O#.
.O#O#O.O#..
O.O.#.#.O..
.##O.O####.
.##.O#O..#.
.O.O.O.##..
.##.#.####.
.##O.##.##.
...........

In this example, if the Elf's goal was to get exactly 6 more steps today, he could use them to reach any of 16 garden plots.

However, the Elf actually needs to get 64 steps today, and the map he's handed you is much larger than the example map.

Starting from the garden plot marked S on your map, how many garden plots could the Elf reach in exactly 64 steps?
*/

use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    North = 0,
    East = 1,
    West = 2,
    South = 3,
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point(pub i64, pub i64);

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Point {
    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.0 >= 0 && self.1 >= 0 && (self.0 as usize) < width && (self.1 as usize) < height
    }
}

pub fn reachable(grid: &Vec<Vec<bool>>, start: Point, steps: usize) -> (usize, usize) {
    let width = grid[0].len();
    let height = grid.len();

    let mut old_border = HashSet::new();
    let mut border = HashSet::new();
    border.insert(start);

    let mut reachable = 1 - steps % 2;
    let mut reachable_alternate = 1 - reachable;
    for i in 0..steps {
        let mut new_border = HashSet::with_capacity(border.len() + 4);
        for point in &border {
            for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
                let next = *point + dir.delta();
                if next.in_bounds(width, height) && !grid[next.1 as usize][next.0 as usize] {
                    if !old_border.contains(&next) && !border.contains(&next) {
                        new_border.insert(next);
                    }
                }
            }
        }
        old_border = border;
        border = new_border;
        if (i + steps) % 2 == 1 {
            // in this step, all gardens are reachable
            reachable += border.len();
        }
        else {
            reachable_alternate += border.len();
        }
        if border.len() == 0 {
            break;
        }
    }
    (reachable, reachable_alternate)
}

#[test]
pub fn part1() {
    // idea: get the closest distance to all the blocks in the grid by flood filling.
    // Then note, that all of the blocks that are closer than/equal 64 blocks can be reached.
    // However only the blocks with even distance can be reached in exactly 64 steps.

    use std::io;

    let mut grid = vec![];
    let mut start = None;
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let line: Vec<_> = input.chars().enumerate().map(|(i, c)|
        match c { '.' => false, '#' => true, 'S' => {
            assert!(start.is_none(), "Can't have more than 1 start");
            start = Some(Point(i as i64, grid.len() as i64));
            false
        }, _ => panic!("invalid char {c}"), }).collect();
        grid.push(line);
    }
    let start = start.expect("Must have exactly one start");

    let reachable = reachable(&grid, start, 64).0;

    println!("{reachable} gardens are reachable");
}

/*
--- Part Two ---

The Elf seems confused by your answer until he realizes his mistake: he was reading from a list of his favorite numbers that are both perfect squares and perfect cubes, not his step counter.

The actual number of steps he needs to get today is exactly 26501365.

He also points out that the garden plots and rocks are set up so that the map repeats infinitely in every direction.

So, if you were to look one additional map-width or map-height out from the edge of the example map above, you would find that it keeps repeating:

.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##..S####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................

This is just a tiny three-map-by-three-map slice of the inexplicably-infinite farm layout; garden plots and rocks repeat as far as you can see. The Elf still starts on the one middle tile marked S, though - every other repeated S is replaced with a normal garden plot (.).

Here are the number of reachable garden plots in this new infinite version of the example map for different numbers of steps:

    In exactly 6 steps, he can still reach 16 garden plots.
    In exactly 10 steps, he can reach any of 50 garden plots.
    In exactly 50 steps, he can reach 1594 garden plots.
    In exactly 100 steps, he can reach 6536 garden plots.
    In exactly 500 steps, he can reach 167004 garden plots.
    In exactly 1000 steps, he can reach 668697 garden plots.
    In exactly 5000 steps, he can reach 16733044 garden plots.

However, the step count the Elf needs is much larger! Starting from the garden plot marked S on your infinite map, how many garden plots could the Elf reach in exactly 26501365 steps?
*/

pub fn reachable_periodic(grid: &Vec<Vec<bool>>, start: Point, steps: usize) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut old_border = HashSet::new();
    let mut border = HashSet::new();
    border.insert(start);

    let mut reachable = 1 - steps % 2;
    for i in 0..steps {
        let mut new_border = HashSet::with_capacity(border.len() + 4);
        for point in &border {
            for dir in [Direction::North, Direction::East, Direction::South, Direction::West] {
                let next = *point + dir.delta();
                if !grid[next.1.rem_euclid(height as i64) as usize][next.0.rem_euclid(width as i64) as usize] {
                    if !old_border.contains(&next) && !border.contains(&next) {
                        new_border.insert(next);
                    }
                }
            }
        }
        old_border = border;
        border = new_border;
        if (i + steps) % 2 == 1 {
            // in this step, all gardens are reachable
            // add to the reachable chunk
            reachable += border.len();
        }
    }
    reachable
}

#[test]
pub fn part2() {
    // Do the same, but don't keep the full thing in memory.
    // Just keep the boundary and the last boundary in memory
    // and add up the even area as it goes.
    // To reduce the computation, think about the structure of the problem.
    // In the given data, there is always a complete border out of gardens.
    // That means, that from a given corner, the shortest path
    // to the next corner is always along the free outside path.
    // So knowing when the 4 initial corners are reached is going to massively help.
    // Actually it seems like every (width+height) steps, it just adds a multiple of the reacheable area.
    // This might be problematic if the width+height is odd,
    // so in that case just duplicate the grid in the odd direction.
    // So that makes it trivial.
    // This would actually work similarly for any data structure.

    // correct answer 598044246091826
    // first try!
    
    use std::io;

    let mut grid = vec![];
    let mut start = None;
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let line: Vec<_> = input.chars().enumerate().map(|(i, c)|
        match c { '.' => false, '#' => true, 'S' => {
            assert!(start.is_none(), "Can't have more than 1 start");
            start = Some(Point(i as i64, grid.len() as i64));
            false
        }, _ => panic!("invalid char {c}"), }).collect();
        grid.push(line);
    }
    let start = start.expect("Must have exactly one start");
    let width = grid[0].len();
    let height = grid.len();
    
    let steps = 26501365;
    let mut step_mod = width + height;
    
    //let (mut area, area_alt) = reachable(&grid, start, width * height);
    println!("{width}x{height}");
    
    if step_mod % 2 == 1 {
        step_mod *= 2;
    }
    let simulate_steps = 2; // if this is too low and the start doesn't manage to reach the corners in time, and the computation fails.
    let big_steps = ((steps / step_mod + simulate_steps).max(simulate_steps) - 2 * simulate_steps) as u128;
    println!("{step_mod} {big_steps}");
    
    let area1 = reachable_periodic(&grid, start, steps % step_mod + step_mod * simulate_steps) as u128;
    let area2 = reachable_periodic(&grid, start, steps % step_mod + step_mod * (simulate_steps + 1)) as u128;
    let area3 = reachable_periodic(&grid, start, steps % step_mod + step_mod * (simulate_steps + 2)) as u128;
    
    // extrapolate quadratically using lagrange polynomials
    let l3 = area3 * big_steps * (big_steps - 1) / 2;
    let l2 = area2 * big_steps * (big_steps - 2);
    let l1 = area1 * (big_steps - 1) * (big_steps - 2) / 2;
    let reachable = l1 + l3 - l2;
    println!("{reachable} gardens are reachable");
}