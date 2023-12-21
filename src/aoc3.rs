/*
--- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
...*..#...
617.1..1..
......+.58
.592.1....
....+.754.
...$.*....
.664.598..
.5.5..7.7.
..$.....*.
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 5642.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/


pub fn is_part(c: char) -> bool {
    !c.is_digit(10) && !c.is_whitespace() && c != '.'
}

use std::str::Chars;

#[derive(Clone, Copy, Debug)]
pub enum NumberState<T> {
    None,
    Started(T),
    Partial(T),
    Finished(T),
}

impl<T> NumberState<T> {
    pub fn is_started(&self) -> bool {
        match self {
            NumberState::Started(_) => true,
            _ => false,
        }
    }
    pub fn is_finished(&self) -> bool {
        match self {
            NumberState::Finished(_) => true,
            _ => false,
        }
    }
    pub fn is_some(&self) -> bool {
        match self {
            NumberState::None => false,
            _ => true,
        }
    }
    pub fn unwrap(self) -> Option<T> {
        match self {
            NumberState::Started(num) => Some(num),
            NumberState::Partial(num) => Some(num),
            NumberState::Finished(num) => Some(num),
            NumberState::None => None,
        }
    }
    pub fn unwrap_finished(self) -> Option<T> {
        match self {
            NumberState::Finished(num) => Some(num),
            _ => None,
        }
    }
    pub fn unfinished(self) -> NumberState<T> {
        match self {
            NumberState::Started(num) => NumberState::Started(num),
            NumberState::Partial(num) => NumberState::Partial(num),
            _ => NumberState::None,
        }
    }
}

/// an iterator over chars, which detects positive number as it goes.
pub struct NumberIterator<'a> {
    base: Chars<'a>,
    current_num: Option<u32>,
}

impl Iterator for NumberIterator<'_> {
    type Item = (char, NumberState<u32>);
    fn next(&mut self) -> Option<Self::Item> {
        let c = self.base.next()?;
        let state = if let Some(num) = self.current_num {
            if !c.is_digit(10) {
                // finish number
                self.current_num = None;
                NumberState::Finished(num)
            }
            else {
                // extend number
                let num = num * 10 + c.to_digit(10).unwrap();
                self.current_num = Some(num);
                NumberState::Partial(num)
            }
        }
        else if c.is_digit(10) {
            let num = c.to_digit(10).unwrap();
            self.current_num = Some(num);
            NumberState::Started(num)
        }
        else {
            NumberState::None
        };
        Some((c, state))
    }
}

pub fn read_numbers(line: &str) -> NumberIterator {
    NumberIterator { base: line.chars(), current_num: None }
}

#[test]
pub fn part1() {
    // idea: still go line by line, but this time keep the last line around
    // then do:
    // 1. check if unused numbers from the last line match parts of the current line
    // 2. check if numbers from the current line match parts from the last line and remove these numbers
    // the only issue with this idea is, that I need to parse the numbers twice, but I consider that OK.
    
    use std::io;

    let mut sum: u64 = 0;
    let mut last_line = None;
    loop {
        let mut line = String::new();
        let read_bytes = io::stdin().read_line(&mut line).expect("Failed to read line");
        if line.len() <= 1 || read_bytes == 0 {
            break;
        }
        line = line.trim().into();
        line.push('.'); // add a . to never end on a number (simplifies stuff)
        if last_line.is_none() {
            last_line = ".".repeat(line.len()).into();
        }
        // check that all lines have equal length (otherwise I will need to fill them up with .)
        assert!(line.len() == last_line.as_ref().unwrap().len());
        let mut line_buffer = String::with_capacity(line.len());
        let mut bot_number_confirmed = false;
        let mut top_number_confirmed = false;
        let mut last_bot_part = false;
        let mut last_top_part = false;
        for ((bot_c, bot_num), (top_c, top_num))
                in read_numbers(&line).zip(read_numbers(&last_line.unwrap())) {
            if let NumberState::Finished(num) = bot_num {
                // finish number
                if bot_number_confirmed || is_part(bot_c) || is_part(top_c) {
                    sum += num as u64;
                    // remove the number from the line buffer and replace it with .
                    let num_length = line_buffer.chars().rev().enumerate().find_map(|(i, c)|
                        if c.is_digit(10) { None }
                        else { Some(i) }).unwrap_or(line_buffer.len());
                    assert!(num_length > 0);
                    line_buffer.drain(line_buffer.len()-num_length..);
                    for _ in 0..num_length {
                        line_buffer.push('.');
                    }
                }
                bot_number_confirmed = false;
            }
            if let NumberState::Finished(num) = top_num {
                // finish number
                if top_number_confirmed || is_part(bot_c) {
                    sum += num as u64;
                }
                top_number_confirmed = false;
            }

            if bot_num.is_started() {
                bot_number_confirmed = last_bot_part || last_top_part;
            }
            else if is_part(bot_c) {
                bot_number_confirmed |= bot_num.is_some();
                top_number_confirmed |= top_num.is_some() || top_c.is_digit(10);
            }
            if top_num.is_started() {
                top_number_confirmed = last_bot_part | is_part(bot_c);
            }
            else if is_part(top_c) {
                bot_number_confirmed |= bot_num.is_some();
                top_number_confirmed |= top_num.is_some();
            }
            last_bot_part = is_part(bot_c);
            last_top_part = is_part(top_c);
            line_buffer.push(bot_c);
        }
        //println!("{line_buffer} {sum}");
        last_line = Some(line_buffer);
    }
    println!("The engine part sum is {sum}");
}

/*
--- Part Two ---

The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?
*/

/// WIP gear during parsing
pub struct Gear {
    left: [Option<u32>; 3],
    right: [NumberState<u32>; 3],
}

impl Gear {
    pub fn is_valid(&self) -> bool {
        // valid if there is exactly 2 numbers
        let mut count = 0;
        for i in 0..3 {
            count += self.left[i].is_some() as u32;
            count += self.right[i].is_some() as u32;
        }
        count == 2
    }
    pub fn ratio(&self) -> u64 {
        // find the two numbers and add them up
        let mut count = 0;
        let mut ratio = 1;
        for i in 0..3 {
            if let Some(num) = self.left[i] {
                ratio *= num as u64;
                count += 1;
            }
            if let NumberState::Finished(num) = self.right[i] {
                ratio *= num as u64;
                count += 1;
            }
        }
        assert!(count == 2, "gear wasn't a finished valid gear");
        ratio
    }
    pub fn update(&mut self, right: [NumberState<u32>; 3]) -> bool {
        let mut count_finished = 0;
        for i in 0..3 {
            match self.right[i] {
                NumberState::Partial(_) | NumberState::Started(_) => self.right[i] = right[i],
                _ => (),
            }
            match self.right[i] {
                NumberState::Finished(_) | NumberState::None => count_finished += 1,
                _ => (),
            }
        }
        count_finished == 3
    }
}

#[test]
pub fn part2() {
    // idea: do it in a scan like in last solution but use 3 lines here.
    // start with 2 empty lines
    // 1. read the 3 lines simultaneously and detect numbers like before
    // 2. when a gear is encountered in the center, check how many numbers are around
    //    if it's 2, then do the ratio thing
    // 3. when a gear was encountered in the center at last char and not used, check again
    // 4. end with an additional empty line
    use std::io;

    let mut sum: u64 = 0;
    let mut last_line: Option<String> = None;
    let mut last2_line = None;
    loop {
        let mut line = String::new();
        let read_bytes = io::stdin().read_line(&mut line).expect("Failed to read line");
        let end = line.len() <= 1 || read_bytes == 0;
        if end {
            line = ".".repeat(last_line.as_ref().unwrap().len()).into();
        }
        else {
            line = line.trim().into();
            line.push('.'); // add a . to never end on a number (simplifies stuff)
        }
        if last_line.is_none() {
            assert!(last2_line.is_none());
            last_line = ".".repeat(line.len()).into();
            last2_line = ".".repeat(line.len()).into();
        }
        // check that all lines have equal length (otherwise I will need to fill them up with .)
        assert!(line.len() == last_line.as_ref().unwrap().len());
        assert!(line.len() == last2_line.as_ref().unwrap().len());
        let mut last_center = '.';
        let mut last_gear_data = [Some(0); 3];
        let mut gears: Vec<Gear> = vec![];
        for (i1, (i2, i3)) in read_numbers(last2_line.as_ref().unwrap()).zip(read_numbers(last_line.as_ref().unwrap()).zip(read_numbers(&line))) {
            let (_, num1) = i1;
            let (c2, num2) = i2;
            let (_, num3) = i3;
            if last_center == '*' {
                // here all the necessary data is already collected
                // except for the numbers at the right (which none of them are finished at this point)
                last_gear_data = [
                    last_gear_data[0].or(num1.unwrap_finished()),
                    last_gear_data[1].or(num2.unwrap_finished()),
                    last_gear_data[2].or(num3.unwrap_finished()),
                ];
                let gear = Gear {
                    left: last_gear_data,
                    right: [num1.unfinished(), num2.unfinished(), num3.unfinished()],
                };
                if gear.is_valid() {
                    gears.push(gear);
                }
            }
            if c2 == '*' {
                // collect all the data to complete it in the next character read
                last_gear_data = [
                    num1.unwrap_finished(),
                    num2.unwrap_finished(),
                    num3.unwrap_finished(),
                ];
            }
            // update the existing gears
            gears.retain_mut(|gear| {
                if gear.update([num1, num2, num3]) {
                    sum += gear.ratio();
                    false
                }
                else {
                    true
                }
            });
            last_center = c2;
        }
        if end {
            break;
        }
        last2_line = last_line;
        last_line = Some(line);
    }
    println!("The gear ratio sum is {sum}");
}