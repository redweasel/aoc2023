/*
--- Day 19: Aplenty ---

The Elves of Gear Island are thankful for your help and send you on your way. They even have a hang glider that someone stole from Desert Island; since you're already going that direction, it would help them a lot if you would use it to get down there and return it to them.

As you reach the bottom of the relentless avalanche of machine parts, you discover that they're already forming a formidable heap. Don't worry, though - a group of Elves is already here organizing the parts, and they have a system.

To start, each part is rated in each of four categories:

    x: Extremely cool looking
    m: Musical (it makes a noise when you hit it)
    a: Aerodynamic
    s: Shiny

Then, each part is sent through a series of workflows that will ultimately accept or reject the part. Each workflow has a name and contains a list of rules; each rule specifies a condition and where to send the part if the condition is true. The first rule that matches the part being considered is applied immediately, and the part moves on to the destination described by the rule. (The last rule in each workflow has no condition and always applies if reached.)

Consider the workflow ex{x>10:one,m<20:two,a>30:R,A}. This workflow is named ex and contains four rules. If workflow ex were considering a specific part, it would perform the following steps in order:

    Rule "x>10:one": If the part's x is more than 10, send the part to the workflow named one.
    Rule "m<20:two": Otherwise, if the part's m is less than 20, send the part to the workflow named two.
    Rule "a>30:R": Otherwise, if the part's a is more than 30, the part is immediately rejected (R).
    Rule "A": Otherwise, because no other rules matched the part, the part is immediately accepted (A).

If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns. If a part is accepted (sent to A) or rejected (sent to R), the part immediately stops any further processing.

The system works, but it's not keeping up with the torrent of weird metal shapes. The Elves ask if you can help sort a few parts and give you the list of workflows and some part ratings (your puzzle input). For example:

px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}

The workflows are listed first, followed by a blank line, then the ratings of the parts the Elves would like you to sort. All parts begin in the workflow named in. In this example, the five listed parts go through the following workflows:

    {x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
    {x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
    {x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
    {x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
    {x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A

Ultimately, three parts are accepted. Adding up the x, m, a, and s rating for each of the accepted parts gives 7540 for the part with x=787, 4623 for the part with x=2036, and 6951 for the part with x=2127. Adding all of the ratings for all of the accepted parts gives the sum total of 19114.

Sort through all of the parts you've been given; what do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted?
*/

use std::cmp::Ordering;
use std::ops::*;
use std::time::*;
use std::collections::HashMap;

// a general type that captures all the conditions
pub enum Condition {
    Less(u8, u32),
    Equal(u8, u32),
    Greater(u8, u32),
    True,
}

impl Condition {
    pub fn check(&self, part: &Part) -> bool {
        match self {
            Condition::Less(c, value) => &part.get(*c) < value,
            Condition::Greater(c, value) => &part.get(*c) > value,
            Condition::Equal(c, value) => &part.get(*c) == value,
            Condition::True => true,
        }
    }
}

pub struct Workflow {
    pub conditions: Vec<Condition>,
    pub next: Vec<String>,
}

impl Workflow {
    pub fn next(&self, part: &Part) -> Option<&str> {
        for (c, next) in self.conditions.iter().zip(self.next.iter()) {
            if c.check(part) {
                return Some(next);
            }
        }
        None
    }
}

fn char_to_index(xmas_char: char) -> u8 {
    "xmas".find(xmas_char).expect("invalid char (not from xmas)") as u8
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let value = value.trim_end_matches('}').trim_start_matches('{');
        let mut conditions = vec![];
        let mut next = vec![];
        for instruction in value.split(',') {
            if let Some((condition, next_name)) = instruction.split_once(':') {
                if let Some((var, value)) = condition.split_once('<') {
                    conditions.push(Condition::Less(char_to_index(var.chars().next().unwrap()), value.parse().expect("failed to parse number")));
                }
                else if let Some((var, value)) = condition.split_once('>') {
                    conditions.push(Condition::Greater(char_to_index(var.chars().next().unwrap()), value.parse().expect("failed to parse number")));
                }
                else if let Some((var, value)) = condition.split_once('=') {
                    conditions.push(Condition::Equal(char_to_index(var.chars().next().unwrap()), value.parse().expect("failed to parse number")));
                }
                else {
                    panic!("The condition wasn't <, >, =");
                }
                next.push(next_name.to_string());
            }
            else {
                // end condition
                conditions.push(Condition::True);
                next.push(instruction.to_string());
            }
        }
        Workflow { conditions, next }
    }
}

pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn get(&self, c: u8) -> u32 {
        match c {
            0 => self.x,
            1 => self.m,
            2 => self.a,
            3 => self.s,
            _ => panic!("invalid index letter"),
        }
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let value = value.trim_end_matches('}').trim_start_matches('{');
        let xmas: Vec<_> = value.split(',').collect();
        let x = xmas.iter().find_map(|eq| eq.split_once('=').map_or(None, |(a, b)| if a == "x" { Some(b.parse::<u32>().expect("couldn't parse number")) } else { None })).expect("no x found");
        let m = xmas.iter().find_map(|eq| eq.split_once('=').map_or(None, |(a, b)| if a == "m" { Some(b.parse::<u32>().expect("couldn't parse number")) } else { None })).expect("no m found");
        let a = xmas.iter().find_map(|eq| eq.split_once('=').map_or(None, |(a, b)| if a == "a" { Some(b.parse::<u32>().expect("couldn't parse number")) } else { None })).expect("no a found");
        let s = xmas.iter().find_map(|eq| eq.split_once('=').map_or(None, |(a, b)| if a == "s" { Some(b.parse::<u32>().expect("couldn't parse number")) } else { None })).expect("no s found");
        Part { x , m , a , s }
    }
}

#[test]
pub fn part1() {
    // I call it the xmas algorithmus

    use std::io;

    let mut workflows = HashMap::<String, Workflow>::new();
    let mut parts = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if input.starts_with('{') {
            // part
            parts.push(Part::from(input));
        }
        else if input.len() != 0 {
            // workflow
            let (name, workflow) = input.split_at(input.find('{').expect("line invalid"));
            workflows.insert(name.to_string(), Workflow::from(workflow));
        }
    }

    let mut sum = 0;
    for part in parts {
        let mut workflow_name = "in";
        loop {
            let workflow = workflows.get(workflow_name).unwrap_or_else(|| panic!("no workflow named {}", workflow_name));
            workflow_name = workflow.next(&part).unwrap();
            if workflow_name == "R" {
                // reject
                break;
            }
            if workflow_name == "A" {
                // accept
                sum += part.x + part.m + part.a + part.s;
                break;
            }
        }
    }
    println!("The accepted checksum is {sum}");
}

/*
--- Part Two ---

Even with your help, the sorting process still isn't fast enough.

One of the Elves comes up with a new plan: rather than sort parts individually through all of these workflows, maybe you can figure out in advance which combinations of ratings will be accepted or rejected.

Each of the four ratings (x, m, a, s) can have an integer value ranging from a minimum of 1 to a maximum of 4000. Of all possible distinct combinations of ratings, your job is to figure out which ones will be accepted.

In the above example, there are 167409079868000 distinct combinations of ratings that will be accepted.

Consider only your list of workflows; the list of part ratings that the Elves wanted you to sort is no longer relevant. How many distinct combinations of ratings will be accepted by the Elves' workflows?
*/

#[test]
pub fn part2() {
    // combinatorics again eeh?
    // Idea 1: the part ratings act as a kinda guide in the workflow graph.
    // However, some ratings are equivalent in how they guide the workflow.
    // 1. figure out the equivalence classes assuming all conditions in all workflows are used (representant and size)
    // 2. go through all combinations of the equivalence classes
    // this works well if there are only a few workflows with a few conditions.

    // Idea 2: starting from the back of the graph (at A) one can
    // immediately discard a lot of inputs, because they could never lead to A.
    // - this may speed it up or slow it down, depending on the case
    // - if we're lucky some representants can be discarded completely -> huge speedup in special cases
    // - not very good idea for the given data it seems like...

    // Idea 3: turn this into a boolean satisfiability problem by
    // 1. finding all paths, which lead to A
    // 2. path conditions: AND of the used conditions along the path
    // 3. total condition: OR of all path conditions.
    // These types of problems are NP-complete though. Is the given problem NP-complete?
    // There are many workflow, but few conditions per workflow.
    // I don't think the amount of paths grows too quickly,
    // as conditions quickly cancel and become unsatisfiable for long paths.
    // This can be already considered while finding all paths!

    use std::io;
    
    let start = Instant::now();

    let mut workflows = HashMap::<String, Workflow>::new();
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if read_bytes == 0 {
            break;
        }
        let input = input.trim();
        if input.starts_with('{') {
            // part (ignore)
        }
        else if input.len() != 0 {
            // workflow
            let (name, workflow) = input.split_at(input.find('{').expect("line invalid"));
            workflows.insert(name.to_string(), Workflow::from(workflow));
        }
    }

    //representants_approach(workflows); // gave me 116738260946855
    paths_approach(workflows); // gave me 116738260946855 as well!

    println!("Time: {:?}", Instant::now() - start);
}

pub fn representants_approach(workflows: HashMap<String, Workflow>) {
    // collect all values for which the next higher value would lead to a change in the condition
    const MAX_VALUE: u32 = 4000; // start counting at 1!
    let mut representants = [vec![MAX_VALUE], vec![MAX_VALUE], vec![MAX_VALUE], vec![MAX_VALUE]];
    for workflow in workflows.values() {
        for condition in &workflow.conditions {
            match *condition {
                Condition::Less(c, v) => {
                    if v <= MAX_VALUE {
                        representants[c as usize].push(v - 1);
                    }
                }
                Condition::Greater(c, v) => {
                    if v < MAX_VALUE {
                        representants[c as usize].push(v);
                    }
                }
                Condition::Equal(c, v) => {
                    let dst = &mut representants[c as usize];
                    if v <= MAX_VALUE {
                        dst.push(v-1);
                    }
                    if v < MAX_VALUE {
                        dst.push(v);
                    }
                }
                Condition::True => (),
            }
        }
    }
    for vec in &mut representants {
        vec.sort();
        vec.dedup();
    }
    // at this point the largest representant for all equivalent classes is known.
    // The sized can be read out from the neighboring indices.
    let representants = [0, 1, 2, 3].map(|i| {
        let first = *representants[i].first().unwrap();
        let mut representants_sizes = vec![(first, first)];
        for w in representants[i].windows(2) {
            representants_sizes.push((w[1], w[1] - w[0]));
        }
        representants_sizes
    });
    // For all combinations of these inputs, do the workflow procedure
    // and figure out the exact list of accepted parts (don't save it)
    println!("checking a list of {} combinations", representants.iter().map(|v| v.len()).product::<usize>());
    let mut combinations = 0;
    let start = Instant::now();
    // obviously parallelizing with rayon would be awesome here, but I'm going without libraries for this project.
    for (i, &(x, size_x)) in representants[0].iter().enumerate() {
        let size = size_x as u128;
        for &(m, size_m) in representants[1].iter() {
            let size = size * size_m as u128;
            for &(a, size_a) in representants[2].iter() {
                let size = size * size_a as u128;
                for &(s, size_s) in representants[3].iter() {
                    let size = size * size_s as u128;
                    let part = Part { x, m, a, s };
                    let mut workflow_name = "in";
                    loop {
                        let workflow = workflows.get(workflow_name).unwrap_or_else(|| panic!("no workflow named {}", workflow_name));
                        workflow_name = workflow.next(&part).unwrap();
                        if workflow_name == "R" {
                            // reject
                            break;
                        }
                        if workflow_name == "A" {
                            // accept
                            combinations += size;
                            break;
                        }
                    }
                }
            }
        }
        let total = representants[0].len() as u32;
        let processed = (i+1) as u32;
        let remaining = total - processed;
        println!("({processed}/{total}) avg per step: {:?}, remaining: {:?}", (Instant::now() - start) / processed, ((Instant::now() - start) / processed) * remaining);
    }
    println!("There are {combinations} combinations");
}


/// Create a range, if it is not empty, otherwise return None
pub fn checked_range<T: Ord>(start: T, end: T) -> Option<Range<T>> {
    if start < end {
        Some(start..end)
    }
    else {
        None
    }
}

/// Subtract a range from this one.
/// This can result in up to 2 ranges, which are returned in order.
pub fn sub_range<T: Ord + Clone>(lhs: Range<T>, rhs: Range<T>) ->[Option<Range<T>>; 2] {
    [checked_range(lhs.start.clone(), rhs.start.min(lhs.end.clone())),
        checked_range(rhs.end.max(lhs.start), lhs.end)]
}

#[derive(Clone)]
pub struct RangeVec<T: Ord> {
    ranges: Vec<Range<T>>,
}

impl<T: Ord> RangeVec<T> {
    pub fn contains(&self, value: &T) -> bool {
        let start = match self.ranges.binary_search_by(|x| if &x.end <= value { Ordering::Less } else { Ordering::Greater }) {
            Ok(index) => index,
            Err(index) => index,
        };
        self.ranges.get(start).map_or(false, |range| range.contains(value))
    }
}

impl<T: Ord + Clone> RangeVec<T> {
    pub fn remove_range<B: RangeBounds<T>>(&mut self, range: B) {
        // find the range in last_locations, which gets mapped
        let start = match range.start_bound() {
            Bound::Included(start) => {
                Some((match self.ranges.binary_search_by(|x| if &x.end <= start { Ordering::Less } else { Ordering::Greater }) {
                    Ok(index) => index,
                    Err(index) => index,
                }, start.clone()))
            },
            Bound::Unbounded => None,
            _ => panic!("unimplemented bound type"),
        };
        let end = match range.end_bound() {
            Bound::Excluded(end) => {
                Some((match self.ranges.binary_search_by(|x| if &x.start < end { Ordering::Less } else { Ordering::Greater }) {
                    Ok(index) => index,
                    Err(index) => index,
                }, end.clone()))
            },
            Bound::Unbounded => None,
            _ => panic!("unimplemented bound type"),
        };
        // 4 possibilities now
        if let Some((start, start_elem)) = start {
            if let Some((end, end_elem)) = end {
                let range = start_elem..end_elem;
                // remove all ranges in this region and check the first and last
                let mut removed_ranges = self.ranges.drain(start..end);
                let mut splits = vec![];
                if let Some(removed) = removed_ranges.next() {
                    splits.push(sub_range(removed, range.clone()));
                }
                if let Some(removed) = removed_ranges.last() {
                    splits.push(sub_range(removed, range));
                }
                // put the splits back in
                self.ranges.splice(start..start, splits.into_iter().flatten().filter_map(|i| i));
            }
            else {
                if start < self.ranges.len() {
                    let start_ref = &mut self.ranges[start].end;
                    if *start_ref > start_elem {
                        *start_ref = start_elem;
                    }
                    if self.ranges[start].start == self.ranges[start].end {
                        self.ranges.drain(start..);
                    }
                    else {
                        self.ranges.drain(start+1..);
                    }
                }
            }
        }
        else {
            if let Some((end, end_elem)) = end {
                if end > 0 {
                    let end_ref = &mut self.ranges[end-1].start;
                    if *end_ref < end_elem {
                        *end_ref = end_elem;
                    }
                    if self.ranges[end-1].start == self.ranges[end-1].end {
                        self.ranges.drain(..end);
                    }
                    else {
                        self.ranges.drain(..end-1);
                    }
                }
            }
            else {
                // remove everything (clear)
                self.ranges.clear();
            }
        }
    }
}
impl<T: Ord + Default> RangeVec<T>
where for<'a> &'a T: Add<Output = T> + Sub<Output = T> {
    pub fn volume(&self) -> T {
        self.ranges.iter().map(|range| &range.end - &range.start).reduce(|sum, len| &sum + &len).unwrap_or_default()
    }
}

impl<T: Ord> From<Range<T>> for RangeVec<T> {
    fn from(value: Range<T>) -> Self {
        RangeVec { ranges: vec![value] }
    }
}

pub fn find_paths(workflows: &HashMap<String, Workflow>, start: &str, mut path_condition: [RangeVec<u32>; 4], paths: &mut Vec<[RangeVec<u32>; 4]>) {
    let workflow = workflows.get(start).unwrap();
    for (cond, next) in workflow.conditions.iter().zip(&workflow.next) {
        // make a new path condition by combining the new condition with the existing path
        let mut path_condition2 = path_condition.clone();
        let mut path_anti_condition = path_condition.clone();
        match *cond {
            Condition::Equal(c, value) => {
                let pcond = &mut path_condition2[c as usize];
                let anti = &mut path_anti_condition[c as usize];
                *pcond = (value..value+1).into();
                anti.remove_range(value..value+1);
            },
            Condition::Less(c, value) => {
                let pcond = &mut path_condition2[c as usize];
                let anti = &mut path_anti_condition[c as usize];
                pcond.remove_range(value..);
                anti.remove_range(..value);
            },
            Condition::Greater(c, value) => {
                let pcond = &mut path_condition2[c as usize];
                let anti = &mut path_anti_condition[c as usize];
                pcond.remove_range(..value+1);
                anti.remove_range(value+1..);
            },
            Condition::True => (),
        }
        if path_condition2.iter().map(|cond| cond.volume() as u128).product::<u128>() > 0 {
            if next == "A" {
                paths.push(path_condition2.clone());
            }
            else if next != "R" {
                find_paths(workflows, next, path_condition2.clone(), paths);
            }
        }
        if let Condition::True = cond {
            break;
        }
        path_condition = path_anti_condition;
    }
}

pub fn paths_approach(workflows: HashMap<String, Workflow>) {
    // find all paths from "in" to "A", which are possible for a part
    let mut paths = vec![];
    find_paths(&workflows, "in", [(1..4001).into(), (1..4001).into(), (1..4001).into(), (1..4001).into()], &mut paths);
    // important insight:
    // if two parts have gone different paths, they have different ratings!
    // Therefore we can do the following sum without double counting:
    let combinations = paths.iter().map(|path_condition| {
        path_condition.iter().map(|cond| cond.volume() as u128).product::<u128>()
    }).sum::<u128>();
    println!("There are {combinations} combinations");
}