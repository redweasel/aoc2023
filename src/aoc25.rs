/*
--- Day 25: Snowverload ---

Still somehow without snow, you go to the last place you haven't checked: the center of Snow Island, directly below the waterfall.

Here, someone has clearly been trying to fix the problem. Scattered everywhere are hundreds of weather machines, almanacs, communication modules, hoof prints, machine parts, mirrors, lenses, and so on.

Somehow, everything has been wired together into a massive snow-producing apparatus, but nothing seems to be running. You check a tiny screen on one of the communication modules: Error 2023. It doesn't say what Error 2023 means, but it does have the phone number for a support line printed on it.

"Hi, you've reached Weather Machines And So On, Inc. How can I help you?" You explain the situation.

"Error 2023, you say? Why, that's a power overload error, of course! It means you have too many components plugged in. Try unplugging some components and--" You explain that there are hundreds of components here and you're in a bit of a hurry.

"Well, let's see how bad it is; do you see a big red reset button somewhere? It should be on its own module. If you push it, it probably won't fix anything, but it'll report how overloaded things are." After a minute or two, you find the reset button; it's so big that it takes two hands just to get enough leverage to push it. Its screen then displays:

SYSTEM OVERLOAD!

Connected components would require
power equal to at least 100 stars!

"Wait, how many components did you say are plugged in? With that much equipment, you could produce snow for an entire--" You disconnect the call.

You have nowhere near that many stars - you need to find a way to disconnect at least half of the equipment here, but it's already Christmas! You only have time to disconnect three wires.

Fortunately, someone left a wiring diagram (your puzzle input) that shows how the components are connected. For example:

jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr

Each line shows the name of a component, a colon, and then a list of other components to which that component is connected. Connections aren't directional; abc: xyz and xyz: abc both represent the same configuration. Each connection between two components is represented only once, so some components might only ever appear on the left or right side of a colon.

In this example, if you disconnect the wire between hfx/pzl, the wire between bvb/cmg, and the wire between nvd/jqt, you will divide the components into two separate, disconnected groups:

    9 components: cmg, frs, lhk, lsr, nvd, pzl, qnr, rsh, and rzs.
    6 components: bvb, hfx, jqt, ntq, rhn, and xhk.

Multiplying the sizes of these groups together produces 54.

Find the three wires you need to disconnect in order to divide the components into two separate groups. What do you get if you multiply the sizes of these two groups together?
*/

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Node {
    pub next: Vec<(usize, usize)>,
    pub merged: Vec<usize>, // uses the original indices
}

impl Node {
    pub fn new(index: usize) -> Self {
        Node { next: vec![], merged: vec![index] }
    }
}

#[derive(Clone, Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: vec![] }
    }

    /// get mutable reference to the node at index i, if the index is just out of range, a new node is created.
    pub fn get_mut(&mut self, index: usize) -> &mut Node {
        while self.nodes.len() <= index {
            self.nodes.push(Node::new(self.nodes.len()));
        }
        &mut self.nodes[index]
    }

    /// add edge weight
    pub fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        assert_ne!(from, to);
        if let Some((_, w)) = self.get_mut(from).next.iter_mut().find(|(next, _)| *next == to) {
            // this edge already exists
            *w += weight;
            self.get_mut(to).next.iter_mut().find(|(next, _)| *next == from).unwrap().1 += weight;
        }
        else {
            self.get_mut(from).next.push((to, weight));
            self.get_mut(to).next.push((from, weight));
        }
    }

    pub fn merge_verts(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        if a > b {
            // don't allow b < a, since then moved == a is a possibility
            self.merge_verts(b, a);
            return;
        }
        // remove the connections to b
        for (next, _) in &self.nodes[b].next.clone() {
            self.nodes[*next].next.retain(|(i, _)| *i != b);
        }
        // swap remove b
        let Node { next: removed_next, merged } = self.nodes.swap_remove(b);
        self.nodes[a].merged.extend(merged);
        // change the connections that referred to the last node, to b
        let moved = self.nodes.len();
        if b != moved {
            let moved_node = &self.nodes[b];
            for (next, _) in moved_node.next.clone() {
                self.nodes[next].next.iter_mut().for_each(|(i, _)| if *i == moved { *i = b; });
            }
        }
        // add the connections to b back in as connections to a
        for (mut next, w) in removed_next {
            if next == moved {
                next = b;
            }
            if a != next {
                self.add_edge(a, next, w);
            }
        }
    }

    pub fn minimum_phase_cut(&mut self) -> (usize, Vec<usize>) {
        let a = 0;
        let mut a_vec = vec![a];
        let mut a_set = HashSet::new();
        a_set.insert(a);
        let mut last = None;
        // O(V E log E) complexity
        while a_vec.len() < self.nodes.len() {
            last = a_vec.last().copied();
            let mut a_next: Vec<_> = a_vec.iter().map(|j| &self.nodes[*j].next).flatten().filter(|(i, _)| !a_set.contains(i)).copied().collect();
            a_next.sort_unstable_by_key(|(i, _)| *i);
            a_next.dedup_by(|a, b| {
                if a.0 == b.0 {
                    // combine into b
                    b.1 += a.1;
                    true
                }
                else {
                    false
                }
            });
            let new = a_next.into_iter().max_by_key(|(_, w)| *w).unwrap().0;
            a_vec.push(new);
            a_set.insert(new);
        }
        let v1 = *a_vec.last().unwrap();
        let cut_weight = self.nodes[v1].next.iter().map(|(_, w)| *w).sum();
        let new_cut = self.nodes[v1].merged.clone();
        if let Some(v2) = last {
            self.merge_verts(v1, v2);
        }
        (cut_weight, new_cut)
    }

    pub fn min_cut(mut self, max_cut: usize) -> Vec<usize> {
        // I would have liked to implement Karger's algorithm,
        // but I don't feel like doing my own random number rn.
        // -> implement the Stoer–Wagner algorithm
        // see https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm
        let mut best_cut = Vec::new();
        let mut best_cut_weight = self.nodes.len() * self.nodes.len();
        for _ in 1..self.nodes.len() {
            let (weight, new_cut) = self.minimum_phase_cut();
            println!("weight {weight} with cut-size {}", new_cut.len());
            if weight < best_cut_weight {
                best_cut = new_cut;
                best_cut_weight = weight;
            }
            if weight <= max_cut {
                break;
            }
        }
        best_cut
    }
}

pub fn get_index(map: &mut HashMap<String, usize>, name: &str) -> usize {
    if let Some(index) = map.get(name) {
        *index
    }
    else {
        let index = map.len();
        map.insert(name.to_string(), index);
        index
    }
}

/*
simple test case (2 triangles)
a: b
b: c
c: a d
d: e
e: f
f: d
*/

#[test]
pub fn part1() {
    // typical graph problem again
    // This can be solved with the minimum cut of the graph, as that will be <= 3

    let mut graph = Graph::new();
    let mut node_names = HashMap::new();
    loop {
        let mut input = String::new();
        let read_bytes = std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let input = input.trim();
        let (from, to) = input.split_once(":").expect("missing :, invalid line");
        let from = from.trim();
        let from = get_index(&mut node_names, from);
        for to in to.split(' ') {
            let to = to.trim();
            if to.len() > 0 {
                let to = get_index(&mut node_names, to);
                graph.add_edge(from, to, 1);
            }
        }
    }
    let mut rev_node_names = HashMap::new();
    for (k, v) in node_names {
        rev_node_names.insert(v, k);
    }
    // do a partition of the graph
    let graph_len = graph.nodes.len();
    let cut = graph.min_cut(3);
    let cut_named: Vec<_> = cut.iter().map(|x| &rev_node_names[x]).collect();
    //println!("{n}, {cut_named:?}");
    println!("{cut_named:?}");
    println!("The cut resulted in a region sized {} and {}", cut.len(), graph_len - cut.len());
    println!("multiplied sizes: {}", cut.len() * (graph_len - cut.len()));
}

/*
--- Part Two ---

You climb over weather machines, under giant springs, and narrowly avoid a pile of pipes as you find and disconnect the three wires.

A moment after you disconnect the last wire, the big red reset button module makes a small ding noise:

System overload resolved!
Power required is now 50 stars.

Out of the corner of your eye, you notice goggles and a loose-fitting hard hat peeking at you from behind an ultra crucible. You think you see a faint glow, but before you can investigate, you hear another small ding:

Power required is now 49 stars.

Please supply the necessary stars and
push the button to restart the system.

------

You supply all fifty stars and restart global snow production!

As you reach the edge of Snow Island, you can already tell from way up here that everyone will have a white Christmas this year after all.
*/