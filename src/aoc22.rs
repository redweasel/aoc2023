/*
--- Day 22: Sand Slabs ---

Enough sand has fallen; it can finally filter water for Snow Island.

Well, almost.

The sand has been falling as large compacted bricks of sand, piling up to form an impressive stack here near the edge of Island Island. In order to make use of the sand to filter water, some of the bricks will need to be broken apart - nay, disintegrated - back into freely flowing sand.

The stack is tall enough that you'll have to be careful about choosing which bricks to disintegrate; if you disintegrate the wrong brick, large portions of the stack could topple, which sounds pretty dangerous.

The Elves responsible for water filtering operations took a snapshot of the bricks while they were still falling (your puzzle input) which should let you work out which bricks are safe to disintegrate. For example:

1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9

Each line of text in the snapshot represents the position of a single brick at the time the snapshot was taken. The position is given as two x,y,z coordinates - one for each end of the brick - separated by a tilde (~). Each brick is made up of a single straight line of cubes, and the Elves were even careful to choose a time for the snapshot that had all of the free-falling bricks at integer positions above the ground, so the whole snapshot is aligned to a three-dimensional cube grid.

A line like 2,2,2~2,2,2 means that both ends of the brick are at the same coordinate - in other words, that the brick is a single cube.

Lines like 0,0,10~1,0,10 or 0,0,10~0,1,10 both represent bricks that are two cubes in volume, both oriented horizontally. The first brick extends in the x direction, while the second brick extends in the y direction.

A line like 0,0,1~0,0,10 represents a ten-cube brick which is oriented vertically. One end of the brick is the cube located at 0,0,1, while the other end of the brick is located directly above it at 0,0,10.

The ground is at z=0 and is perfectly flat; the lowest z value a brick can have is therefore 1. So, 5,5,1~5,6,1 and 0,2,1~0,2,5 are both resting on the ground, but 3,3,2~3,3,3 was above the ground at the time of the snapshot.

Because the snapshot was taken while the bricks were still falling, some bricks will still be in the air; you'll need to start by figuring out where they will end up. Bricks are magically stabilized, so they never rotate, even in weird situations like where a long horizontal brick is only supported on one end. Two bricks cannot occupy the same position, so a falling brick will come to rest upon the first other brick it encounters.

Here is the same example again, this time with each brick given a letter so it can be marked in diagrams:

1,0,1~1,2,1   <- A
0,0,2~2,0,2   <- B
0,2,3~2,2,3   <- C
0,0,4~0,2,4   <- D
2,0,5~2,2,5   <- E
0,1,6~2,1,6   <- F
1,1,8~1,1,9   <- G

At the time of the snapshot, from the side so the x axis goes left to right, these bricks are arranged like this:

 x
012
.G. 9
.G. 8
... 7
FFF 6
..E 5 z
D.. 4
CCC 3
BBB 2
.A. 1
--- 0

Rotating the perspective 90 degrees so the y axis now goes left to right, the same bricks are arranged like this:

 y
012
.G. 9
.G. 8
... 7
.F. 6
EEE 5 z
DDD 4
..C 3
B.. 2
AAA 1
--- 0

Once all of the bricks fall downward as far as they can go, the stack looks like this, where ? means bricks are hidden behind other bricks at that location:

 x
012
.G. 6
.G. 5
FFF 4
D.E 3 z
??? 2
.A. 1
--- 0

Again from the side:

 y
012
.G. 6
.G. 5
.F. 4
??? 3 z
B.C 2
AAA 1
--- 0

Now that all of the bricks have settled, it becomes easier to tell which bricks are supporting which other bricks:

    Brick A is the only brick supporting bricks B and C.
    Brick B is one of two bricks supporting brick D and brick E.
    Brick C is the other brick supporting brick D and brick E.
    Brick D supports brick F.
    Brick E also supports brick F.
    Brick F supports brick G.
    Brick G isn't supporting any bricks.

Your first task is to figure out which bricks are safe to disintegrate. A brick can be safely disintegrated if, after removing it, no other bricks would fall further directly downward. Don't actually disintegrate any bricks - just determine what would happen if, for each brick, only that brick were disintegrated. Bricks can be disintegrated even if they're completely surrounded by other bricks; you can squeeze between bricks if you need to.

In this example, the bricks can be disintegrated as follows:

    Brick A cannot be disintegrated safely; if it were disintegrated, bricks B and C would both fall.
    Brick B can be disintegrated; the bricks above it (D and E) would still be supported by brick C.
    Brick C can be disintegrated; the bricks above it (D and E) would still be supported by brick B.
    Brick D can be disintegrated; the brick above it (F) would still be supported by brick E.
    Brick E can be disintegrated; the brick above it (F) would still be supported by brick D.
    Brick F cannot be disintegrated; the brick above it (G) would fall.
    Brick G can be disintegrated; it does not support any other bricks.

So, in this example, 5 bricks can be safely disintegrated.

Figure how the blocks will settle based on the snapshot. Once they've settled, consider disintegrating a single brick; how many bricks could be safely chosen as the one to get disintegrated?
*/

/*
another example for testing

0,0,1~2,0,1
2,0,3~2,2,3
2,2,7~0,2,7
0,2,9~0,0,9
1,1,10~1,1,13
0,1,11~0,1,12

2 blocks can be disintegrated
stack height is 6
topple sum is 4+3+2+1 = 10

another example (Cantor stack)

0,0,1~27,0,1
0,0,2~9,0,2
18,0,2~27,0,2
0,0,3~3,0,3
6,0,3~9,0,3
18,0,3~21,0,3
24,0,3~27,0,3

4 can be disintegrated
stack height is 3
topple sum is 6+2+2 = 10

*/

use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point(pub i64, pub i64, pub i64);

impl TryFrom<&str> for Point {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, ()> {
        let mut v_iter = value.split(',').filter_map(|n| n.parse().ok());
        let x = v_iter.next().ok_or(())?;
        let y = v_iter.next().ok_or(())?;
        let z = v_iter.next().ok_or(())?;
        if !v_iter.next().is_none() {
            return Err(());
        }
        Ok(Point(x, y, z))
    }
}

impl Point {
    pub fn min(&self, other: &Self) -> Self {
        Point(self.0.min(other.0), self.1.min(other.1), self.2.min(other.2))
    }
    pub fn max(&self, other: &Self) -> Self {
        Point(self.0.max(other.0), self.1.max(other.1), self.2.max(other.2))
    }
}

pub struct Heightmap<T> {
    heights: Vec<T>,
    stride: usize,
    min: (i64, i64),
}

impl<T: Copy> Heightmap<T> {
    pub fn new(zero: T, x_min: i64, y_min: i64, x_max: i64, y_max: i64) -> Self {
        let stride = (x_max - x_min + 1) as usize;
        Heightmap { heights: vec![zero; stride * (y_max - y_min + 1) as usize], stride, min: (x_min, y_min) }
    }
}

impl<T> Index<(i64, i64)> for Heightmap<T> {
    type Output = T;
    fn index(&self, index: (i64, i64)) -> &Self::Output {
        &self.heights[(index.0 - self.min.0) as usize + (index.1 - self.min.1) as usize * self.stride]
    }
}

impl<T> IndexMut<(i64, i64)> for Heightmap<T> {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        &mut self.heights[(index.0 - self.min.0) as usize + (index.1 - self.min.1) as usize * self.stride]
    }
}

#[derive(Debug)]
pub struct Node {
    pub rests_on: Vec<usize>,
    pub is_support: bool,
    pub topple: u8,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

pub fn stack(blocks: &Vec<(Point, Point)>, min: &Point, max: &Point) -> Graph {
    let mut graph = Graph { nodes: vec![] };
    let mut height = Heightmap::new((0, None), min.0, min.1, max.0, max.1);
    for (i, block) in blocks.iter().enumerate() {
        let mut node = Node { rests_on: vec![], is_support: false, topple: 0 };
        // iterate through the cubes in the block
        if block.0.0 != block.1.0 {
            // x aligned
            let mut rest_height = 1;
            for x in block.0.0..=block.1.0 {
                rest_height = rest_height.max(height[(x, block.0.1)].0 + 1);
            }
            let tup = (rest_height, Some(i));
            for x in block.0.0..=block.1.0 {
                let entry = &mut height[(x, block.0.1)];
                if entry.0 == rest_height - 1 {
                    if let Some(block_id) = entry.1 {
                        if &block_id != node.rests_on.last().unwrap_or(&usize::MAX) {
                            node.rests_on.push(block_id);
                        }
                    }
                }
                *entry = tup;
            }
        }
        else if block.0.1 != block.1.1 {
            // y aligned
            let mut rest_height = 1;
            for y in block.0.1..=block.1.1 {
                rest_height = rest_height.max(height[(block.0.0, y)].0 + 1);
            }
            let tup = (rest_height, Some(i));
            for y in block.0.1..=block.1.1 {
                let entry = &mut height[(block.0.0, y)];
                if entry.0 == rest_height - 1 {
                    if let Some(block_id) = entry.1 {
                        if &block_id != node.rests_on.last().unwrap_or(&usize::MAX) {
                            node.rests_on.push(block_id);
                        }
                    }
                }
                *entry = tup;
            }
        }
        else {
            // z aligned (or single cube)
            let entry = &mut height[(block.0.0, block.0.1)];
            if let Some(block_id) = entry.1 {
                node.rests_on.push(block_id);
            }
            let rest_height = entry.0 + (block.1.2 - block.0.2 + 1);
            *entry = (rest_height, Some(i));
        }
        // if the node.rests_on nothing, then it's implicitly resting on the ground.
        graph.nodes.push(node);
    }
    println!("max stacked height is {}", height.heights.iter().map(|(height, _)| height).max().unwrap());
    // stacking and graphing complete
    // now find all supported nodes
    for i in 0..graph.nodes.len() {
        if graph.nodes[i].rests_on.len() == 1 {
            let support = graph.nodes[i].rests_on[0];
            graph.nodes[support].is_support = true;
        }
    }
    graph
}

#[test]
pub fn part1() {
    // idea: this is a graph type problem again.
    // 1. read in the data and sort it by minimal z-coordinate
    // 2. make a graph for which blocks are above which (list of linear constraints for each block)
    // 3. compute the fallen down state using the linear constraints from the graph
    // 4. find all nodes in the graph which are supported by exactly one other node and mark that other node as support.
    // 5. invert that selection to get all nodes, which can be removed first.

    use std::io;

    let mut blocks = vec![];
    let mut min: Option<Point> = None;
    let mut max: Option<Point> = None;
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let (p1, p2) = input.split_once('~').expect("no ~ to split the two points");
        let p1 = Point::try_from(p1).expect("failed to parse point");
        let p2 = Point::try_from(p2).expect("failed to parse point");
        // normalize data by making the first point, the one with smaller coordinates
        if p1.0 < p2.0 || p1.1 < p2.1 || p1.2 < p2.2 {
            blocks.push((p1, p2));
        }
        else {
            blocks.push((p2, p1));
        }
        min = Some(p1.min(&min.unwrap_or(p1)));
        max = Some(p2.max(&max.unwrap_or(p2)));
    }
    let min = min.expect("expected at least one block");
    let max = max.unwrap();
    blocks.sort_by_key(|block| block.0.2);
    println!("footprint: {min:?} <-> {max:?}");
    // blocks are read in and sorted - now create the graph
    // there is many ways of doing spatial tree hierachies.
    // I'm going for the simplest possible here. Just sweep and prune.
    // The data has a very small xy footprint, so I will exploit that!
    // -> Do it as a painting!
    // Don't exploit that the data is only positive.
    let graph = stack(&blocks, &min, &max);
    let supports = graph.nodes.iter().filter(|node| node.is_support).count();
    let free = graph.nodes.len() - supports;
    println!("{free} blocks can be disintegrated");
}

/*
--- Part Two ---

Disintegrating bricks one at a time isn't going to be fast enough. While it might sound dangerous, what you really need is a chain reaction.

You'll need to figure out the best brick to disintegrate. For each brick, determine how many other bricks would fall if that brick were disintegrated.

Using the same example as above:

    Disintegrating brick A would cause all 6 other bricks to fall.
    Disintegrating brick F would cause only 1 other brick, G, to fall.

Disintegrating any other brick would cause no other bricks to fall. So, in this example, the sum of the number of other bricks that would fall as a result of disintegrating each brick is 7.

For each brick, determine how many other bricks would fall if that brick were disintegrated. What is the sum of the number of other bricks that would fall?
*/

#[test]
pub fn part2() {
    // 66039 too high

    use std::io;

    let mut blocks = vec![];
    let mut min: Option<Point> = None;
    let mut max: Option<Point> = None;
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let (p1, p2) = input.split_once('~').expect("no ~ to split the two points");
        let p1 = Point::try_from(p1).expect("failed to parse point");
        let p2 = Point::try_from(p2).expect("failed to parse point");
        // normalize data by making the first point, the one with smaller coordinates
        if p1.0 < p2.0 || p1.1 < p2.1 || p1.2 < p2.2 {
            blocks.push((p1, p2));
        }
        else {
            blocks.push((p2, p1));
        }
        min = Some(p1.min(&min.unwrap_or(p1)));
        max = Some(p2.max(&max.unwrap_or(p2)));
    }
    let min = min.expect("expected at least one block");
    let max = max.unwrap();
    blocks.sort_by_key(|block| block.0.2);
    println!("footprint: {min:?} <-> {max:?}");
    let mut graph = stack(&blocks, &min, &max);
    // now compute the topple count for each node, that is a support.
    // There is probably some fancy algorithm for this, but my input is small,
    // so I'm going to do the naive thing of counting the nodes for each support.
    let mut topple_counter = 0;
    for i in 0..graph.nodes.len() {
        let topple_marker = (i % 255 + 1) as u8;
        if topple_marker == 1 && i != 0 {
            // wrapped, reset all topple markers
            for node in &mut graph.nodes {
                node.topple = 0;
            }
        }
        if graph.nodes[i].is_support {
            // let it fall
            graph.nodes[i].topple = topple_marker;
            for j in i+1..graph.nodes.len() {
                // if all supports are falling, let it fall
                if graph.nodes[j].rests_on.len() > 0 && graph.nodes[j].rests_on.iter().all(|&k| graph.nodes[k].topple == topple_marker) {
                    graph.nodes[j].topple = topple_marker;
                    topple_counter += 1;
                }
            }
        }
    }
    println!("{topple_counter} is the sum of the number of blocks that would fall");
}