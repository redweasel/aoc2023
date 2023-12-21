/*
--- Day 5: If You Give A Seed A Fertilizer ---

You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.

"A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.

"Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.

"I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"

You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just arrived and we're having trouble making sense of it."

The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

For example:

seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

Consider again the example seed-to-soil map:

50 98 2
52 50 48

The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

So, the entire list of seed numbers and their corresponding soil numbers looks like this:

seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51

With this map, you can look up the soil number required for each initial seed number:

    Seed number 79 corresponds to soil number 81.
    Seed number 14 corresponds to soil number 14.
    Seed number 55 corresponds to soil number 57.
    Seed number 13 corresponds to soil number 13.

The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:

    Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
    Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
    Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
    Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.

So, the lowest location number in this example is 35.

What is the lowest location number that corresponds to any of the initial seed numbers?
*/

#[test]
pub fn aoc5_part1() {
    // idea: make a custom type for the mapping
    // -> assuming the mapping is correct, sort the source list and use binary search for each lookup
    // then read in all the data into the mappings
    // this is the best general idea, but for speed one can also do
    // - read in the seeds
    // - read in the tables in order and always just convert the current numbers from the last table directly

    use std::io;

    let mut last_locations = vec![];
    let mut locations = vec![];
    let mut last_location_len = 0;
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if read_bytes == 0 {
            break; // only break on EOF (Ctrl+D)
        }
        if last_location_len == 0 {
            // wait for seeds first, no seeds is an error
            if input.starts_with("seeds:") {
                last_locations.extend(input
                    .trim_start_matches("seeds:")
                    .trim()
                    .split(' ')
                    .filter_map(|num| num.parse::<i64>().ok()));
                last_location_len = last_locations.len();
            }
        }
        else {
            // expect the conversion tables to come in correct order
            if input.trim().ends_with(":") {
                // new table
                locations.extend(last_locations);
                assert_eq!(locations.len(), last_location_len);
                last_locations = locations;
                last_locations.sort(); // sort here! last_locations is always sorted!
                //println!("{last_locations:?}");
                locations = vec![];
            }
            else if input.chars().next().unwrap().is_digit(10) {
                // number row of the table with meaning
                // source index, dest index, length
                let numbers: Vec<i64> = input.trim().split(' ').filter_map(|s| s.parse().ok()).collect();
                assert_eq!(numbers.len(), 3, "numbers = {numbers:?}");
                let src = numbers[1];
                let add = numbers[0] - src;
                let len = numbers[2];
                // find the range in last_locations, which gets mapped
                let start = match last_locations.binary_search(&src) {
                    Ok(index) => index,
                    Err(index) => index,
                };
                let end = match last_locations[start..].binary_search(&(src + len)) {
                    Ok(index) => index,
                    Err(index) => index,
                } + start;
                locations.extend(last_locations.drain(start..end).map(|x| x + add));
            }
        }
    }
    locations.extend(last_locations);
    assert_eq!(locations.len(), last_location_len);
    //println!("{locations:?}");

    println!("The lowest location number is {}", locations.iter().min().expect("no locations available"));
}

/*
--- Part Two ---

Everyone will starve if you only plant such a small number of seeds. Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair, the first value is the start of the range and the second value is the length of the range. So, in the first line of the example above:

seeds: 79 14 55 13

This line describes two ranges of seed numbers to be planted in the garden. The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92. The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

In the above example, the lowest location number can be obtained from seed number 82, which corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and location 46. So, the lowest location number is 46.

Consider all of the initial seed numbers listed in the ranges on the first line of the almanac. What is the lowest location number that corresponds to any of the initial seed numbers?
*/

/// Interval with invariant start < end, start inclusive, end exclusive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    pub fn new(start: i64, end: i64) -> Interval {
        assert!(start < end);
        Interval { start, end }
    }
    pub fn new_checked(start: i64, end: i64) -> Option<Interval> {
        if start < end {
            Some(Interval { start, end })
        }
        else {
            None
        }
    }

    pub fn start(&self) -> i64 {
        self.start
    }

    pub fn end(&self) -> i64 {
        self.end
    }

    /// Subtract an interval from this one.
    /// This can result in up to 2 intervals, which are returned in order.
    pub fn sub(&self, other: Interval) ->[Option<Interval>; 2] {
        [Interval::new_checked(self.start, other.start.min(self.end)),
         Interval::new_checked(other.end.max(self.start), self.end)]
    }

    /// Compute the intersection of this interval with the other interval.
    /// Since Intervals can't be empty, return None in case the intersection is empty.
    pub fn intersection(&self, other: Interval) -> Option<Interval> {
        Interval::new_checked(self.start.max(other.start), self.end.min(other.end))
    }
}

impl std::ops::Add<i64> for Interval {
    type Output = Interval;
    fn add(self, rhs: i64) -> Self::Output {
        Interval { start: self.start + rhs, end: self.end + rhs }
    }
}

impl std::fmt::Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}

pub fn volume(v: &Vec<Interval>) -> u64 {
    v.iter().map(|i| (i.end() - i.start()) as u64).sum()
}

#[test]
pub fn aoc5_part2() {
    // actually... the custom type wasn't needed!
    // I now need to consider intervals.
    // -> creating an interval type is the cleanest solution

    use std::io;

    let mut last_locations = vec![];
    let mut locations = vec![];
    let mut last_location_len = 0;
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if read_bytes == 0 {
            break; // only break on EOF (Ctrl+D)
        }
        if last_location_len == 0 {
            // wait for seeds first, no seeds is an error
            if input.starts_with("seeds:") {
                let numbers: Vec<_> = input
                        .trim_start_matches("seeds:")
                        .trim()
                        .split(' ')
                        .filter_map(|num| num.parse::<i64>().ok())
                        .collect();
                last_locations.extend(numbers
                    .chunks_exact(2)
                    .map(|c| Interval::new(c[0], c[0] + c[1])));
                last_location_len = volume(&last_locations);
            }
        }
        else {
            // expect the conversion tables to come in correct order
            if input.trim().ends_with(":") {
                // new table
                locations.extend(last_locations);
                assert_eq!(volume(&locations), last_location_len);
                last_locations = locations;
                last_locations.sort_by_key(|i| i.start); // sort here! last_locations is always sorted!
                //println!("{last_locations:?}");
                locations = vec![];
            }
            else if input.chars().next().unwrap().is_digit(10) {
                // number row of the table with meaning
                // source index, dest index, length
                let numbers: Vec<i64> = input.trim().split(' ').filter_map(|s| s.parse().ok()).collect();
                assert_eq!(numbers.len(), 3, "numbers = {numbers:?}");
                let src = numbers[1];
                let add = numbers[0] - src;
                let len = numbers[2];
                let src_interval = Interval::new(src, src + len);
                // find the range in last_locations, which gets mapped
                let start = match last_locations.binary_search_by_key(&(src + 1), |i| i.end()) {
                    Ok(index) => index,
                    Err(index) => index,
                };
                let end = match last_locations[start..].binary_search_by_key(&(src + len), |i| i.start()) {
                    Ok(index) => index,
                    Err(index) => index,
                } + start;
                let mut remains = vec![];
                locations.extend(last_locations.drain(start..end).map(|x| {
                    remains.push(x.sub(src_interval));
                    x.intersection(src_interval).map(|i| i + add)
                }).filter_map(|x| x));
                // put the splits back in
                last_locations.splice(start..start, remains.into_iter().flatten().filter_map(|i| i));
                assert!(last_locations.is_sorted_by_key(|i| i.start()));
                assert!(last_locations.is_sorted_by_key(|i| i.end()));
            }
        }
    }
    locations.extend(last_locations);
    assert_eq!(volume(&locations), last_location_len);
    //println!("{locations:?}");

    println!("The lowest location number is {}", locations.iter().map(|i| i.start()).min().expect("no locations available"));
}