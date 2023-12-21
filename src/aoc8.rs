/*
--- Day 8: Haunted Wasteland ---

You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)

Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)

Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left = 0,
    Right = 1,
}

#[test]
pub fn part1() {
    use std::io;
    use std::collections::HashMap;

    let mut nodes = HashMap::new();
    let mut path = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if nodes.len() > 0 && input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if input.len() == 0 {
            continue;
        }
        if path.len() == 0 {
            path.extend(input.chars().map(|c| {
                match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("invalid letter {c} encountered in path"),
                }
            }));
        }
        else {
            let (src, dst) = input.split_once('=').expect("can't split the line at =");
            let dst = dst.trim();
            let (dst_l, dst_r) = dst[1..dst.len()-1].split_once(',').expect("can't split the line at ,");
            let src = src.trim().to_string();
            let dst_l = dst_l.trim().to_string();
            let dst_r = dst_r.trim().to_string();
            nodes.insert(src, [dst_l, dst_r]);
        }
    }
    // Now start the algorithm!
    // All we need to do is follow the instructions until we find ZZZ.
    // Since we are in a finite graph with no dead ends, we will necessarily run into a cycle.
    // The input seems to guarantee, that this cycle will contain ZZZ if we start from AAA.
    // Hoping the naive solution is fine, just go through the path until ZZZ is reached.
    let res = path.iter().cycle().enumerate().try_fold("AAA", |node, (i, instruction)| {
        let next = &nodes[node][*instruction as usize][..];
        if next == "ZZZ" {
            Err(i + 1)
        }
        else {
            Ok(next)
        }
    }).err().unwrap();
    println!("{res} steps are needed to reach ZZZ from AAA");
}

/*
--- Part Two ---

The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

For example:

LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)

Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

    Step 0: You are at 11A and 22A.
    Step 1: You choose all of the left paths, leading you to 11B and 22B.
    Step 2: You choose all of the right paths, leading you to 11Z and 22C.
    Step 3: You choose all of the left paths, leading you to 11B and 22Z.
    Step 4: You choose all of the right paths, leading you to 11Z and 22B.
    Step 5: You choose all of the left paths, leading you to 11B and 22C.
    Step 6: You choose all of the right paths, leading you to 11Z and 22Z.

So, in this example, you end up entirely on nodes that end in Z after 6 steps.

Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?
*/

// greatest common divisor (euclidean algorithm)
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

// extended euclidean algorithm
pub fn xgcd(mut a: u64, mut b: u64) -> (u64, i64, i64) {
    // return (g, x, y) such that a*x + b*y = g = gcd(a, b)
    let mut x0 = 0;
    let mut x1 = 1;
    let mut y0 = 1;
    let mut y1 = 0;
    while a != 0 {
        let q;
        (q, a, b) = ((b / a) as i64, b % a, a);
        (y0, y1) = (y1, y0 - q * y1); // for b >= 2^63 this q * y1 can be outside of i64 range, but still in u64
        (x0, x1) = (x1, x0 - q * x1);
    }
    (b, x0, y0)
}

#[test]
pub fn part2() {
    use std::io;
    use std::collections::HashMap;

    let mut starts = vec![];
    let mut nodes = HashMap::new();
    let mut path = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if nodes.len() > 0 && input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if input.len() == 0 {
            continue;
        }
        if path.len() == 0 {
            path.extend(input.chars().map(|c| {
                match c {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("invalid letter {c} encountered in path"),
                }
            }));
        }
        else {
            let (src, dst) = input.split_once('=').expect("can't split the line at =");
            let dst = dst.trim();
            let (dst_l, dst_r) = dst[1..dst.len()-1].split_once(',').expect("can't split the line at ,");
            let src = src.trim().to_string();
            let dst_l = dst_l.trim().to_string();
            let dst_r = dst_r.trim().to_string();
            if src.ends_with('A') {
                starts.push(src.clone());
            }
            nodes.insert(src, [dst_l, dst_r]);
        }
    }
    // Now start the algorithm!
    // I love this one!!! <3 <3 <3
    // It's math! To solve it, figure out the cycle sizes
    // and offsets into the cycle that have a Z at the end.
    // Then combine all the knowledge using the chinese remainder theorem.
    // at least that would work, if the graph was guaranteed
    // to be inside the cycle when the first ??Z is reached.
    // Since that is not guaranteed, check all iterations,
    // until all paths are inside the cycles and then compute
    // the rest using the chinese remainder theorem.

    let mut path_data = vec![];
    let mut acyclic_length = 0;
    for start in &starts {
        let mut ends = vec![];
        // The longest possible cycle would be <= path.len() * nodes.len()
        let mut wraps = HashMap::new();
        let (cycle_start_node, cycle_start, cycle_length) = path.iter().enumerate().cycle().enumerate().try_fold(&start[..], |node, (j, (i, instruction))| {
            let next = &nodes[node][*instruction as usize][..];
            if next.ends_with('Z') {
                ends.push((j + 1, next.to_string()));
            }
            if i == 0 {
                // detect cycle
                if let Some(old_j) = wraps.insert(node.to_string(), j) {
                    // node was already in the cycle!
                    // cycle detected!!!
                    Err((node.to_string(), old_j, j - old_j))
                }
                else {
                    Ok(next)
                }
            }
            else {
                Ok(next)
            }
        }).err().unwrap();
        println!("The cycle of {start} is {cycle_length} steps long and {} ends occurred in the run.", ends.len());
        let cycle_ends: Vec<_> = ends.iter().filter(|(i, _)| i >= &cycle_start).cloned().collect();
        println!("The cycle has {} end points. {cycle_ends:?}", cycle_ends.len());

        path_data.push((cycle_ends, cycle_start_node, cycle_start as u64, cycle_length as u64));
        acyclic_length = acyclic_length.max(cycle_start);
    }

    // for the acyclic start, use the naive version of the algorithm.
    if let Err((xxz, res)) = path.iter().cycle().take(acyclic_length).enumerate().try_fold(starts, |c_nodes, (i, instruction)| {
        let next: Vec<_> = c_nodes.iter().map(|node| {
            nodes[node][*instruction as usize].clone()
        }).collect();
        if next.iter().all(|node| node.ends_with('Z')) {
            Err((next, i + 1))
        }
        else {
            Ok(next)
        }
    }) {
        println!("{res} steps are needed to reach {xxz:?}");
        return;
    }
    println!("The end is not reached in the first {acyclic_length} steps.");

    // Now extend the rest using the chinese remainder theorem.
    // Seems like my data only has one endpoint along each cycle, so it's super simple.
    // My data seems even more simple, since all cycles start at the start of the data and finish with the end node xxZ.
    // -> lcm -> 7_309_459_565_207
    // -> 7_309_459_565_207 - 1 steps are needed
    // However I'm implementing this for general offsets.
    let mut n1 = 1;
    let mut a1 = 0u64;
    for (cycle_ends, _, _, n2) in path_data {
        assert!(cycle_ends.len() == 1, "only implemented for one endpoint per cycle");
        let end = cycle_ends[0].0;
        let a2 = (end as i64 - acyclic_length as i64).rem_euclid(n2 as i64) as u64;
        let gcd = gcd(n1, n2 as u64);
        let n = (n1 as i128 * n2 as i128) / gcd as i128;
        // chinese remainder theorem only works for gcd = 1,
        // however if the a1 and a2 are the same (mod gcd) then it can be reduced to that.
        assert!(a1.rem_euclid(gcd) == a2.rem_euclid(gcd), "can't find a solution");
        // in this case do the chinese remainder on a1 (mod n1/gcd), a2 (mod n2/gcd), a1 = a2 =: a3 (mod gcd)
        // which are pairwise coprime. Of course that simplifies to
        // a1 (mod n1), a2 (mod n2/gcd)
        let (_, m1, _) = xgcd(n1, n2 as u64 / gcd);
        // careful with the next formula as not all forms hold when gcd isn't 1
        a1 = (a1 as i128 + (a2 as i128 - a1 as i128) * n1 as i128 * m1 as i128).rem_euclid(n) as u64;
        n1 = n as u64;
    }
    a1 += acyclic_length as u64;
    println!("at most {} steps could be needed with the cycle structure.", n1);
    println!("{a1} steps are actually needed.");
}