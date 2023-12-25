/*
--- Day 23: A Long Walk ---

The Elves resume water filtering operations! Clean water starts flowing over the edge of Island Island.

They offer to help you go over the edge of Island Island, too! Just hold on tight to one end of this impossibly long rope and they'll lower you down a safe distance from the massive waterfall you just created.

As you finally reach Snow Island, you see that the water isn't really reaching the ground: it's being absorbed by the air itself. It looks like you'll finally have a little downtime while the moisture builds up to snow-producing levels. Snow Island is pretty scenic, even without any snow; why not take a walk?

There's a map of nearby hiking trails (your puzzle input) that indicates paths (.), forest (#), and steep slopes (^, >, v, and <).

For example:

#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#

You're currently on the single path tile in the top row; your goal is to reach the single path tile in the bottom row. Because of all the mist from the waterfall, the slopes are probably quite icy; if you step onto a slope tile, your next step must be downhill (in the direction the arrow is pointing). To make sure you have the most scenic hike possible, never step onto the same tile twice. What is the longest hike you can take?

In the example above, the longest hike you can take is marked with O, and your starting position is marked S:

#S#####################
#OOOOOOO#########...###
#######O#########.#.###
###OOOOO#OOO>.###.#.###
###O#####O#O#.###.#.###
###OOOOO#O#O#.....#...#
###v###O#O#O#########.#
###...#O#O#OOOOOOO#...#
#####.#O#O#######O#.###
#.....#O#O#OOOOOOO#...#
#.#####O#O#O#########v#
#.#...#OOO#OOO###OOOOO#
#.#.#v#######O###O###O#
#...#.>.#...>OOO#O###O#
#####v#.#.###v#O#O###O#
#.....#...#...#O#O#OOO#
#.#########.###O#O#O###
#...###...#...#OOO#O###
###.###.#.###v#####O###
#...#...#.#.>.>.#.>O###
#.###.###.#.###.#.#O###
#.....###...###...#OOO#
#####################O#

This hike contains 94 steps. (The other possible hikes you could have taken were 90, 86, 82, 82, and 74 steps long.)

Find the longest hike you can take through the hiking trails listed on your map. How many steps long is the longest hike?
*/

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point(pub isize, pub isize);

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Clone, Debug)]
pub struct Node<K> {
    pub next: Vec<(K, u64)>, // next (node, edge weight)
}

impl<K> Node<K> {
    pub fn new() -> Self {
        Node { next: vec![] }
    }
}

#[derive(Clone, Debug)]
pub struct DirectedGraph<K> {
    pub nodes: HashMap<K, Node<K>>, // used a HashMap because I didn't want to deal with reindexing on simplification
}

pub fn rget<'a, K: PartialEq + Eq + Hash + Clone>(map: &'a HashMap<K, (K, u64)>, key: &'a K) -> (&'a K, u64) {
    let mut res = (key, 0u64);
    while let Some(edge) = map.get(&res.0) {
        res = (&edge.0, res.1 + edge.1);
    }
    res
}

impl<K: PartialEq + Eq + Hash + Clone + std::fmt::Debug> DirectedGraph<K> {
    pub fn new() -> Self {
        DirectedGraph { nodes: HashMap::new() }
    }

    /// simplify the graph by combining edges
    pub fn simplify(&mut self, keep: &[K]) {
        // TODO something is still unstable inside this function...
        // sometimes is just randomly fails at asserts in my tiny example case.
        loop {
            // remove nodes, which are bidirectionally connected to one node and connected to a second node.
            // problem: iterate over all nodes, but also mutate multiple nodes!
            // I don't know a better Rust solution, than cloning the keys into a vec... :(
            let keys: Vec<_> = self.nodes.keys().filter(|k| !keep.contains(k)).cloned().collect();
            let key_map = &mut HashMap::<K, (K, u64)>::new(); // when keys get removed, save their new replacement key and the added length here
            let mut incoming_edges = HashMap::<K, Vec<K>>::new();
            for (key, node) in &self.nodes {
                for (next_key, _) in &node.next {
                    if let Some(incoming) = incoming_edges.get_mut(next_key) {
                        incoming.push(key.clone());
                    }
                    else {
                        incoming_edges.insert(next_key.clone(), vec![key.clone()]);
                    }
                }
            }
            let mut removed_key = false;
            for key in &keys {
                // clone again to release borrow on self.nodes... :(
                let node_next: Vec<_> = self.nodes.get_mut(key).unwrap().next.iter_mut().map(|edge| {
                    let (key, add) = rget(key_map, &edge.0);
                    let res = (key.clone(), edge.1 + add);
                    *edge = res.clone(); // save the found shortcuts
                    res
                    
                }).collect();
                // if this node has more than two edges (of which one needs to be bi), it can not be simplified
                let mut edges: HashSet<_> = incoming_edges[key].iter().cloned().collect();
                edges.extend(node_next.iter().map(|edge| edge.0.clone()));
                if edges.len() > 2 {
                    continue;
                }
                let bi = node_next.iter().enumerate().find(|(_, edge)| self.nodes[&edge.0].next.iter().find(|edge2| rget(key_map, &edge2.0).0 == key).is_some());
                if let Some((i, bi_edge)) = bi {
                    let (bi_key, mut here_to_bi_weight) = bi_edge.clone();
                    // situation:
                    // found a bidirectional connection with this node
                    // -> reduce that bidirection connection by removing this node (merge nodes)
                    // -> add the weight of the bidirectional connection to all connections to the other node
                    // let the edge from #bi point to all #other
                    let (bi_key, added_weight) = rget(key_map, &bi_key);
                    here_to_bi_weight += added_weight;
                    {
                        let bi_next = &mut self.nodes.get_mut(&bi_key).unwrap().next;
                        let bi_to_here = bi_next.iter().position(|edge| rget(key_map, &edge.0).0 == key).unwrap();
                        let (bi_key2, mut bi_to_here_weight) = bi_next.swap_remove(bi_to_here);
                        bi_to_here_weight += rget(key_map, &bi_key2).1;
                        assert_eq!(bi_to_here_weight, here_to_bi_weight);
                        for other_i in 0..node_next.len() {
                            if other_i == i {
                                continue;
                            }
                            let (other_key, here_to_other_weight) = &node_next[other_i];
                            bi_next.push((other_key.clone(), bi_to_here_weight + here_to_other_weight));
                        }
                    }
                    // let the edges other_to_here point from #other to #bi
                    key_map.insert(key.clone(), (bi_key.clone(), here_to_bi_weight));
                    // remove the node
                    self.nodes.remove(key);
                    removed_key = true;
                }
            }
            if !removed_key {
                break; // no nodes removed
            }
            // make the key_map real
            for (key, node) in &mut self.nodes {
                assert!(!key_map.contains_key(key));
                for (nkey, w) in &mut node.next {
                    while let Some((new_key, added_weight)) = key_map.get(nkey) {
                        assert_ne!(new_key, nkey);
                        *nkey = new_key.clone();
                        *w += added_weight;
                    }
                }
            }
        }
    }

    pub fn topological_sort(&self) -> Option<Vec<K>> {
        // A simple implementation of Kahn's algorithm
        // https://en.wikipedia.org/wiki/Topological_sorting
        // This needs to consume the graph edges.
        let mut incoming_edges = HashMap::<K, Vec<K>>::new();
        let mut incoming_edges_count = 0;
        for (key, node) in &self.nodes {
            for (next_key, _) in &node.next {
                if let Some(incoming) = incoming_edges.get_mut(next_key) {
                    incoming.push(key.clone());
                }
                else {
                    incoming_edges.insert(next_key.clone(), vec![key.clone()]);
                }
                incoming_edges_count += 1;
            }
        }
        let mut start_keys: Vec<_> = self.nodes.keys().filter(|key| !incoming_edges.contains_key(key)).cloned().collect();
        println!("{} starting node(s) in the graph", start_keys.len());
        let mut sorted = vec![];
        while let Some(key) = start_keys.pop() {
            sorted.push(key.clone());
            for (next_key, _) in &self.nodes[&key].next {
                let incoming = incoming_edges.get_mut(next_key).unwrap();
                if let Some(index) = incoming.iter().position(|from_key| from_key == &key) {
                    incoming.swap_remove(index);
                    incoming_edges_count -= 1;
                    if incoming.len() == 0 {
                        start_keys.push(next_key.clone());
                    }
                }
                // else -> the edge was no longer part of the graph.
            }
        }
        if incoming_edges_count != 0 {
            println!("{incoming_edges_count} remaining edge(s) in the graph");
            None
        }
        else {
            Some(sorted)
        }
    }

    pub fn find_longest_simple_path(&self, start: &K, end: &K) -> u64 {
        // I've read up a bit on this and it seems this is NP-hard in general.
        // If we have a directed acyclic graph (DAG), this is solvable in linear time.
        // This is because if it's acyclic, the "simple" constraint on the path is
        // kept by definition.
        // I took a gamble and implemented a topological sort to check if it's a DAG
        // turns out the input is a DAG! (for part1...)
        if let Some(topo_sort) = self.topological_sort() {
            // see https://en.wikipedia.org/wiki/Longest_path_problem
            let mut longest = HashMap::<K, i64>::new();
            for key in topo_sort {
                let node = &self.nodes[&key];
                let base_weight = longest.get(&key).copied().unwrap_or(if &key == start { 0 } else { i64::MIN });
                for (next_key, w) in &node.next {
                    let w = *w as i64 + base_weight;
                    if let Some(weight) = longest.get_mut(next_key) {
                        *weight = (*weight).max(w);
                    }
                    else {
                        longest.insert(next_key.clone(), w);
                    }
                }
            }
            return longest[end] as u64;
        }
        // If it's not a DAG, then the solution is brute force search like I did it on day 17.
        // I could memoize it, but for that I would need to use the visited set inside the key... so I'm avoiding that for now.
        self.find_longest_simple_path_dfs(start, end, &mut HashSet::new()).unwrap()
    }

    fn find_longest_simple_path_dfs(&self, start: &K, end: &K, visited: &mut HashSet<K>) -> Option<u64> {
        if start == end {
            return Some(0); // can't leave the key and come pack, as that path would not be simple.
        }
        visited.insert(start.clone());
        let mut longest = None;
        for (key, w) in self.nodes[start].next.clone() {
            if visited.contains(&key) {
                continue;
            }
            if let Some(len) = self.find_longest_simple_path_dfs(&key, end, visited) {
                let len = w + len;
                if let Some(l) = longest {
                    if len > l {
                        longest = Some(len);
                    }
                }
                else {
                    longest = Some(len);
                }
            }
        }
        visited.remove(start);
        longest
    }
}

pub fn add_line_to_graph(input: &str, graph: &mut DirectedGraph<Point>, row: &mut isize, use_slopes: bool) {
    for (i, mut c) in input.chars().enumerate() {
        let i = i as isize;
        if c != '#' {
            if !use_slopes {
                c = '.';
            }
            let mut node = Node::new();
            let point = Point(i, *row);
            let left = point - Point(1, 0);
            let top = point - Point(0, 1);
            match c {
                '.' => {
                    if let Some(left_node) = graph.nodes.get_mut(&left) {
                        node.next.push((left, 1));
                        left_node.next.push((point, 1));
                    }
                    if let Some(top_node) = graph.nodes.get_mut(&top) {
                        node.next.push((top, 1));
                        top_node.next.push((point, 1));
                    }
                },
                '>' => {
                    if let Some(left_node) = graph.nodes.get_mut(&left) {
                        left_node.next.push((point, 1));
                    }
                    else {
                        panic!("invalid input, arrows need to be on a straight section");
                    }
                    if graph.nodes.contains_key(&top) {
                        panic!("invalid input, arrows need to be on a straight section");
                    }
                },
                '<' => {
                    if graph.nodes.contains_key(&left) {
                        node.next.push((left, 1));
                    }
                    else {
                        panic!("invalid input, arrows need to be on a straight section");
                    }
                    if graph.nodes.contains_key(&top) {
                        panic!("invalid input, arrows need to be on a straight section");
                    }
                },
                '^' => {
                    if graph.nodes.contains_key(&left) {
                        panic!("invalid input, arrows need to be on a straight section");
                    }
                    if graph.nodes.contains_key(&top) {
                        node.next.push((top, 1));
                    }
                    else {
                        panic!("invalid input, arrows need to be on a straight section");
                    }
                },
                'v' => {
                    if graph.nodes.contains_key(&left) {
                        panic!("invalid input, arrows need to be on a straight section");
                    }
                    if let Some(top_node) = graph.nodes.get_mut(&top) {
                        top_node.next.push((point, 1));
                    }
                    else {
                        panic!("invalid input, arrows need to be on a straight section");
                    }
                },
                _ => panic!("invalid character {c}"),
            }
            graph.nodes.insert(point, node);
        }
    }
    *row += 1;
}

/*
#.#######
#.......#
#####v#v#
###.....#
###v#####
###.>...#
###v###.#
###.#####

longest path 17

this makes simplify panic sometimes!
No problem on the actual input however.
*/

#[test]
pub fn part1() {
    // idea: build a weighted directed graph and then find the longest path in it.
    // to build the graph, build a dense graph first row by row, then simplify it.
    // still working on the ideas to find the longest path...

    // also at this point I partly regret my decision to not use libraries,
    // as this is all simple implemented stuff in graph libraries.

    let mut graph: DirectedGraph<Point> = DirectedGraph::new();
    let mut start = None; // find start in the top row
    let mut end = None; // find start in the top row
    let mut row = 0;
    loop {
        let mut input = String::new();
        let read_bytes = std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if start.is_none() {
            start = Some(input.chars().enumerate().find_map(|(i, c)| if c == '.' { Some(i) } else { None }).expect("First row must have an entrance to the maze"));
        }
        end = input.chars().enumerate().find_map(|(i, c)| if c == '.' { Some(i) } else { None });
        add_line_to_graph(input, &mut graph, &mut row, true);
    }
    let start = Point(start.expect("Input must have at least one line.") as isize, 0);
    let end = Point(end.expect("The maze must have an exit on the last line.") as isize, row - 1);
    println!("The original graph has {} nodes", graph.nodes.len());
    
    graph.simplify(&[start, end]);
    if false {
        // debug code
        for (key, node) in &graph.nodes {
            print!("({}, {}) -> ", key.0, key.1);
            for next in &node.next {
                print!("({}, {})[{}], ", next.0.0, next.0.1, next.1);
            }
            println!();
        }
    }
    println!("The simplified graph has {} nodes", graph.nodes.len());

    let length = graph.find_longest_simple_path(&start, &end);
    println!("longest path length is {length}");
}

/*
--- Part Two ---

As you reach the trailhead, you realize that the ground isn't as slippery as you expected; you'll have no problem climbing up the steep slopes.

Now, treat all slopes as if they were normal paths (.). You still want to make sure you have the most scenic hike possible, so continue to ensure that you never step onto the same tile twice. What is the longest hike you can take?

In the example above, this increases the longest hike to 154 steps:

#S#####################
#OOOOOOO#########OOO###
#######O#########O#O###
###OOOOO#.>OOO###O#O###
###O#####.#O#O###O#O###
###O>...#.#O#OOOOO#OOO#
###O###.#.#O#########O#
###OOO#.#.#OOOOOOO#OOO#
#####O#.#.#######O#O###
#OOOOO#.#.#OOOOOOO#OOO#
#O#####.#.#O#########O#
#O#OOO#...#OOO###...>O#
#O#O#O#######O###.###O#
#OOO#O>.#...>O>.#.###O#
#####O#.#.###O#.#.###O#
#OOOOO#...#OOO#.#.#OOO#
#O#########O###.#.#O###
#OOO###OOO#OOO#...#O###
###O###O#O###O#####O###
#OOO#OOO#O#OOO>.#.>O###
#O###O###O#O###.#.#O###
#OOOOO###OOO###...#OOO#
#####################O#

Find the longest hike you can take through the surprisingly dry hiking trails listed on your map. How many steps long is the longest hike?
*/

#[test]
pub fn part2() {
    // you must be kidding!
    // now it's not a DAG anymore!

    // 6298 is the answer

    let mut graph: DirectedGraph<Point> = DirectedGraph::new();
    let mut start = None; // find start in the top row
    let mut end = None; // find start in the top row
    let mut row = 0;
    loop {
        let mut input = String::new();
        let read_bytes = std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if start.is_none() {
            start = Some(input.chars().enumerate().find_map(|(i, c)| if c == '.' { Some(i) } else { None }).expect("First row must have an entrance to the maze"));
        }
        end = input.chars().enumerate().find_map(|(i, c)| if c == '.' { Some(i) } else { None });
        add_line_to_graph(input, &mut graph, &mut row, false);
    }
    let start = Point(start.expect("Input must have at least one line.") as isize, 0);
    let end = Point(end.expect("The maze must have an exit on the last line.") as isize, row - 1);
    println!("The original graph has {} nodes", graph.nodes.len());
    graph.simplify(&[start, end]);
    println!("The simplified graph has {} nodes", graph.nodes.len());

    let length = graph.find_longest_simple_path(&start, &end);
    println!("longest path length is {length}");
}