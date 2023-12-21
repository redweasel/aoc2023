/*
--- Day 12: Hot Springs ---

You finally reach the hot springs! You can see steam rising from secluded areas attached to the primary, ornate building.

As you turn to enter, the researcher stops you. "Wait - I thought you were looking for the hot springs, weren't you?" You indicate that this definitely looks like hot springs to you.

"Oh, sorry, common mistake! This is actually the onsen! The hot springs are next door."

You look in the direction the researcher is pointing and suddenly notice the massive metal helixes towering overhead. "This way!"

It only takes you a few more steps to reach the main gate of the massive fenced-off area containing the springs. You go through the gate and into a small administrative building.

"Hello! What brings you to the hot springs today? Sorry they're not very hot right now; we're having a lava shortage at the moment." You ask about the missing machine parts for Desert Island.

"Oh, all of Gear Island is currently offline! Nothing is being manufactured at the moment, not until we get more lava to heat our forges. And our springs. The springs aren't very springy unless they're hot!"

"Say, could you go up and see why the lava stopped flowing? The springs are too cold for normal operation, but we should be able to find one springy enough to launch you up there!"

There's just one problem - many of the springs have fallen into disrepair, so they're not actually sure which springs would even be safe to use! Worse yet, their condition records of which springs are damaged (your puzzle input) are also damaged! You'll need to help them repair the damaged records.

In the giant field just outside, the springs are arranged into rows. For each row, the condition records show every spring and whether it is operational (.) or damaged (#). This is the part of the condition records that is itself damaged; for some springs, it is simply unknown (?) whether the spring is operational or damaged.

However, the engineer that produced the condition records also duplicated some of this information in a different format! After the list of springs for a given row, the size of each contiguous group of damaged springs is listed in the order those groups appear in the row. This list always accounts for every damaged spring, and each number is the entire size of its contiguous group (that is, groups are always separated by at least one operational spring: #### would always be 4, never 2,2).

So, condition records with no unknown spring conditions might look like this:

#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1

However, the condition records are partially damaged; some of the springs' conditions are actually unknown (?). For example:

???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1

Equipped with this information, it is your job to figure out how many different arrangements of operational and broken springs fit the given criteria in each row.

In the first line (???.### 1,1,3), there is exactly one way separate groups of one, one, and three broken springs (in that order) can appear in that row: the first three unknown springs must be broken, then operational, then broken (#.#), making the whole row #.#.###.

The second line is more interesting: .??..??...?##. 1,1,3 could be a total of four different arrangements. The last ? must always be broken (to satisfy the final contiguous group of three broken springs), and each ?? must hide exactly one of the two broken springs. (Neither ?? could be both broken springs or they would form a single contiguous group of two; if that were true, the numbers afterward would have been 2,3 instead.) Since each ?? can either be #. or .#, there are four possible arrangements of springs.

The last line is actually consistent with ten different arrangements! Because the first number is 3, the first and second ? must both be . (if either were #, the first number would have to be 4 or higher). However, the remaining run of unknown spring conditions have many different ways they could hold groups of two and one broken springs:

?###???????? 3,2,1
.###.##.#...
.###.##..#..
.###.##...#.
.###.##....#
.###..##.#..
.###..##..#.
.###..##...#
.###...##.#.
.###...##..#
.###....##.#

In this example, the number of possible arrangements for each row is:

    ???.### 1,1,3 - 1 arrangement
    .??..??...?##. 1,1,3 - 4 arrangements
    ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    ????.#...#... 4,1,1 - 1 arrangement
    ????.######..#####. 1,6,5 - 4 arrangements
    ?###???????? 3,2,1 - 10 arrangements

Adding all of the possible arrangement counts together produces a total of 21 arrangements.

For each row, count all of the different arrangements of operational and broken springs that meet the given criteria. What is the sum of those counts?
*/

/*
additional testing data

..??#?#.#?#???#??#? 5,5,1,1
1 arrangement

.??..??.#?#.??..??.#?#.??..??.#?#.??..??.#?#.??..??.#?#

*/

use std::{collections::HashMap, cell::RefCell, sync::Mutex};

pub fn binom(n: usize, k: usize) -> u128 {
    let mut binom = 1;
    for j in 1..=k as u128 {
        binom *= n as u128 + 1 - j;
        binom /= j;
    }
    binom
}

pub fn just_questions(len: usize, numbers: &[usize]) -> u128 {
    // here it's all just ???????, "len" times
    // 1. if the numbers don't fit, there is no solution
    if numbers.iter().sum::<usize>() + numbers.len() > len + 1 {
        return 0; // the numbers can't fit
    }
    // 2. this can be solved mathematically using the binomial coefficient
    binom(len + 1 - numbers.iter().sum::<usize>(), numbers.len())
}

#[test]
pub fn test_just_questions() {
    for numbers in [[1, 1, 1], [1, 1, 2], [1, 2, 3]] {
        for len in 0..20 {
            // for my old dfs code...
            assert_eq!(just_questions(len, &numbers), dfs2(&"?".repeat(len), &numbers), "failed for len {len} and numbers {numbers:?}");
        }
    }
}

static MEMO: Mutex<RefCell<Option<HashMap<(String, Vec<usize>), u128>>>> = Mutex::new(RefCell::new(None));

// memoization helped very much!!!
pub fn dfs_memo(line: &str, numbers: &[usize]) -> u128 {
    let line = line.trim_matches('.');
    if numbers.iter().sum::<usize>() + numbers.len() > line.len() + 1 {
        return 0; // the numbers can't fit (anymore)
    }
    // check memoized results first
    let key = (line.to_string(), numbers.to_vec());
    {
        let guard = MEMO.lock().expect("failed to lock memo mutex.");
        let mut map = (*guard).borrow_mut();
        if let Some(map) = &*map {
            if let Some(value) = map.get(&key) {
                return *value;
            }
        }
        else {
            *map = Some(HashMap::new());
        }
    }
    let count = dfs(line, numbers);
    // add to the memo{
    let guard = MEMO.lock().expect("failed to lock memo mutex.");
    let mut map = (*guard).borrow_mut();
    (*map).as_mut().unwrap().insert(key, count);
    count
}

// depth first search
pub fn dfs(line: &str, numbers: &[usize]) -> u128 {
    // start at the first ? and try both options (copying the string twice)
    // but first, write a checker for inputs without ? to get the code started.
    let line = line.trim_matches('.');
    if numbers.iter().sum::<usize>() + numbers.len() > line.len() + 1 {
        return 0; // the numbers can't fit (anymore)
    }

    // after the first #, there needs to be a fixed number of # following determined by number
    if let Some((first_index, _)) = line.chars().enumerate().find(|(_, c)| c == &'#') {
        if numbers.len() == 0 {
            return 0; // wrong line, not matching numbers, too many #
        }
        // check for question marks and make sure they are filled up, up to first_index
        // NOTE: this could also be done from the end to start (reverse) and that would be
        // more efficient if there is less question marks at the end
        if let Some((first_q_index, _)) = line[..first_index].chars().enumerate().find(|(_, c)| c == &'?') {
            // found a questionmark that came too early
            // up to (first_index - first_q_index)/2 numbers could be consumed by these question marks
            // for each number of consumed question marks,
            // find the number of possibilities and keep going with dfs,
            // multiplying it's result with the number of possibilities here (huge complexity reduction)
            let mut count = 0;
            for i in 0..((first_index - first_q_index)/2+1).min(numbers.len()) {
                // now we don't know how the group that contains first_index is positioned, so iterate through all possible positions
                let group_len = numbers[i];
                for j in 1..=group_len {
                    // if this position is possible, assume it and count/multiply in both sections
                    let end = first_index+j;
                    if !(first_index+j >= group_len && end <= line.len()) {
                        continue;
                    }
                    let start = first_index+j-group_len;
                    if start < first_q_index {
                        continue;
                    }
                    if !line[start..end].chars().all(|c| c != '.') {
                        continue;
                    }
                    let mut next_line = line.to_string();
                    next_line.replace_range(start..end, &"?".repeat(group_len));
                    // the character before start can never be # because of first_index
                    // however, it needs to be replaced with . because of the group constraint
                    // -> better start the left dfs with that character excluded
                    // the right character could still be anything, so check
                    // if it's a # -> invalid group position
                    // if it's a ? -> needs to be . (skip in dfs input)
                    if let Some(end_c) = next_line.chars().nth(end) {
                        if end_c == '#' {
                            continue;
                        }
                    }
                    let left = if start > 0 && first_q_index < start-1 {
                        dfs_memo(&next_line[first_q_index..start-1], &numbers[..i])
                    }
                    else if i == 0 {
                        1
                    }
                    else {
                        0
                    };
                    let right = if end+1 < next_line.len() {
                        dfs_memo(&next_line[end+1..], &numbers[i+1..])
                    }
                    else if i == numbers.len()-1 {
                        1
                    }
                    else {
                        0
                    };
                    //println!("left: {i} offset: {j} ({start}, {end})-> ({left}, {right})");
                    count += left * right;
                }
            }
            return count;
        }
        // check the whole group
        // from here: assuming everything before first_index is not ?
        let group_len = numbers[0];
        let next_empty = first_index+group_len;
        if next_empty > line.len() {
            return 0; // wrong line, not enough characters to fit the group
        }
        if !line[first_index..next_empty].chars().all(|c| c != '.') {
            return 0; // group was incomplete
        }
        if next_empty < line.len() && line.chars().nth(next_empty).unwrap() == '#' {
            return 0; // group was longer than expected
        }
        // remove the group and recurse
        dfs_memo(&line[(next_empty+1).min(line.len())..], &numbers[1..])
    }
    else if numbers.len() == 0 {
        1 // in these case, following ? need to be ., so only 1 solution
    }
    else if let Some((first_q_index, _)) = line.chars().enumerate().find(|(_, c)| c == &'?') {
        // found a questionmark thats left over
        if let Some((dot_index, _)) = line[first_q_index+1..].chars().enumerate().find(|(_, c)| c == &'.') {
            let dot_index = first_q_index+1+dot_index;
            // split the rest by . and then go through all combinations in which numbers can be distributed on the sections
            // each of the only ??? sections can be computed directly and then multiplied together.
            let mut count = dfs_memo(&line[dot_index+1..], numbers);
            for i in 1..=numbers.len() {
                let left = just_questions(dot_index, &numbers[..i]);
                if left != 0 {
                    let right = dfs_memo(&line[dot_index+1..], &numbers[i..]);
                    count += left * right;
                }
            }
            count
        }
        else {
            // full sequence of ??? -> easy
            just_questions(line.len(), numbers)
        }
    }
    else {
        0
    }
}

#[test]
pub fn test_dfs() {
    for numbers in [[1, 1, 1], [1, 1, 2], [1, 2, 3]] {
        for len in 0..20 {
            assert_eq!(dfs(&"?".repeat(len), &numbers),
                       dfs2(&"?".repeat(len), &numbers),
                       "failed for len {len} and numbers {numbers:?}");
        }
    }
}

// simpler version of depth first search without the mathematical treatment.
pub fn dfs2(line: &str, numbers: &[usize]) -> u128 {
    // start at the first ? and try both options (copying the string twice)
    // but first, write a checker for inputs without ? to get the code started.
    let line = line.trim_matches('.');
    if numbers.iter().sum::<usize>() + numbers.len() > line.len() + 1 {
        return 0; // the numbers can't fit (anymore)
    }
    // after the first #, there needs to be a fixed number of # following determined by number
    if let Some((first_index, _)) = line.chars().enumerate().find(|(_, c)| c == &'#') {
        if numbers.len() == 0 {
            return 0; // wrong line, not matching numbers, too many #
        }
        // check for question marks and make sure they are filled up, up to first_index
        // NOTE: this could also be done from the end to start (reverse) and that would be
        // more efficient if there is less question marks at the end
        if let Some((first_q_index, _)) = line[..first_index].chars().enumerate().find(|(_, c)| c == &'?') {
            // found a questionmark that came too early
            // up to (first_index - first_q_index)/2 numbers could be consumed by these question marks
            let mut count = 0;
            // naive solution
            for c in ['.', '#'] {
                let mut next_line = line[first_q_index+1..].to_string();
                next_line.insert(0, c);
                count += dfs2(&next_line, &numbers);
            }
            return count;
        }
        // check the whole group
        // from here: assuming everything before first_index is not ?
        let group_len = numbers[0];
        let next_empty = first_index+group_len;
        if next_empty > line.len() {
            return 0; // wrong line, not enough characters to fit the group
        }
        if !line[first_index..next_empty].chars().all(|c| c != '.') {
            return 0; // group was incomplete
        }
        if next_empty < line.len() && line.chars().nth(next_empty).unwrap() == '#' {
            return 0; // group was longer than expected
        }
        // remove the group and recurse
        dfs2(&line[(next_empty+1).min(line.len())..], &numbers[1..])
    }
    else if numbers.len() == 0 {
        1 // in these case, following ? need to be ., so only 1 solution
    }
    else if let Some((first_q_index, _)) = line.chars().enumerate().find(|(_, c)| c == &'?') {
        // found a questionmark thats left over
        // try to place one of the remaining groups
        let group_len = numbers[0];
        if let Some((dot_index, _)) = line[first_q_index..first_q_index+group_len].chars().enumerate().find(|(_, c)| c == &'.') {
            // can't place group, skip until after next .
            dfs2(&line[first_q_index+dot_index+1..], numbers)
        }
        else {
            let mut count = 0;
            // place group
            count += if group_len+1 < line.len() {
                dfs2(&line[group_len+1..], &numbers[1..])
            }
            else if numbers.len() == 1 {
                1
            }
            else {
                0
            };
            // or don't place it yet
            count += dfs2(&line[1..], &numbers);
            count
        }
    }
    else {
        0
    }
}

#[test]
pub fn part1() {
    // idea: parse line after line and heavily use combinatorics!
    // actually... use DFS!

    // solution was 7032

    use std::io;
    use std::time::*;
    let start = Instant::now();

    let mut sum = 0;
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let (record, numbers) = input.trim().split_once(' ').expect("failed to split line in two");
        let numbers: &[usize] = &numbers.split(',').map(|n| n.parse::<usize>().ok().expect("failed to parse number")).collect::<Vec<_>>();
        let combinations = dfs(record, numbers);
        println!("arrangements: {combinations}");
        sum += combinations;
    }
    println!("The sum of the different arrangements is {sum}");
    println!("Time: {:?}", Instant::now() - start);
}

/*
--- Part Two ---

As you look out at the field of springs, you feel like there are way more springs than the condition records list. When you examine the records, you discover that they were actually folded up this whole time!

To unfold the records, on each row, replace the list of spring conditions with five copies of itself (separated by ?) and replace the list of contiguous groups of damaged springs with five copies of itself (separated by ,).

So, this row:

.# 1

Would become:

.#?.#?.#?.#?.# 1,1,1,1,1

The first line of the above example would become:

???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3

In the above example, after unfolding, the number of possible arrangements for some rows is now much larger:

    ???.### 1,1,3 - 1 arrangement
    .??..??...?##. 1,1,3 - 16384 arrangements
    ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    ????.#...#... 4,1,1 - 16 arrangements
    ????.######..#####. 1,6,5 - 2500 arrangements
    ?###???????? 3,2,1 - 506250 arrangements

After unfolding, adding all of the possible arrangement counts together produces 525152.

Unfold your condition records; what is the new sum of possible arrangement counts?
*/

#[test]
pub fn part2() {
    // try not to complicate things and just try the naive solution, I don't like this task.
    // this isn't working good enough...
    // idea: combine the simpler solutions here
    // 1. go through all 16 possible choices for the added ?
    // solve the first section for an arbitrary amount of used numbers.
    // for each of those solve the second part

    // I got 2408264176193, but that's wrong appearantly...
    // turns out I had an additional question mark at the end...
    // now I got 1493340882140

    use std::io;
    use std::time::*;
    let start = Instant::now();

    const REPEATS: usize = 5;

    let mut sum = 0;
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let (record, numbers) = input.trim().split_once(' ').expect("failed to split line in two");
        let numbers: &[usize] = &numbers.split(',').map(|n| n.parse::<usize>().ok().expect("failed to parse number")).collect::<Vec<_>>();
        let mut record = record.to_string();
        record.push('?');
        let record = &record.repeat(REPEATS);
        let record = &record[..record.len()-1];
        //println!("{record}");
        let numbers = &numbers.repeat(REPEATS);
        let combinations = dfs_memo(record, numbers);
        println!("arrangements: {combinations}");
        sum += combinations;
    }
    println!("The sum of the different arrangements is {sum}");
    println!("Time: {:?}", Instant::now() - start);
}