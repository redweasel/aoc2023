/*
--- Day 20: Pulse Propagation ---

With your help, the Elves manage to find the right parts and fix all of the machines. Now, they just need to send the command to boot up the machines and get the sand flowing again.

The machines are far apart and wired together with long cables. The cables don't connect to the machines directly, but rather to communication modules attached to the machines that perform various initialization tasks and also act as communication relays.

Modules communicate using pulses. Each pulse is either a high pulse or a low pulse. When a module sends a pulse, it sends that type of pulse to each module in its list of destination modules.

There are several different types of modules:

Flip-flop modules (prefix %) are either on or off; they are initially off. If a flip-flop module receives a high pulse, it is ignored and nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse.

Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.

There is a single broadcast module (named broadcaster). When it receives a pulse, it sends the same pulse to all of its destination modules.

Here at Desert Machine Headquarters, there is a module with a single button on it called, aptly, the button module. When you push the button, a single low pulse is sent directly to the broadcaster module.

After pushing the button, you must wait until all pulses have been delivered and fully handled before pushing it again. Never push the button if modules are still processing pulses.

Pulses are always processed in the order they are sent. So, if a pulse is sent to modules a, b, and c, and then module a processes its pulse and sends more pulses, the pulses sent to modules b and c would have to be handled first.

The module configuration (your puzzle input) lists each module. The name of the module is preceded by a symbol identifying its type, if any. The name is then followed by an arrow and a list of its destination modules. For example:

broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a

In this module configuration, the broadcaster has three destination modules named a, b, and c. Each of these modules is a flip-flop module (as indicated by the % prefix). a outputs to b which outputs to c which outputs to another module named inv. inv is a conjunction module (as indicated by the & prefix) which, because it has only one input, acts like an inverter (it sends the opposite of the pulse type it receives); it outputs to a.

By pushing the button once, the following pulses are sent:

button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> a

After this sequence, the flip-flop modules all end up off, so pushing the button again repeats the same sequence.

Here's a more interesting example:

broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output

This module configuration includes the broadcaster, two flip-flops (named a and b), a single-input conjunction module (inv), a multi-input conjunction module (con), and an untyped module named output (for testing purposes). The multi-input conjunction module con watches the two flip-flop modules and, if they're both on, sends a low pulse to the output module.

Here's what happens if you push the button once:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -high-> output
b -high-> con
con -low-> output

Both flip-flops turn on and a low pulse is sent to output! However, now that both flip-flops are on and con remembers a high pulse from each of its two inputs, pushing the button a second time does something different:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output

Flip-flop a turns off! Now, con remembers a low pulse from module a, and so it sends only a high pulse to output.

Push the button a third time:

button -low-> broadcaster
broadcaster -low-> a
a -high-> inv
a -high-> con
inv -low-> b
con -low-> output
b -low-> con
con -high-> output

This time, flip-flop a turns on, then flip-flop b turns off. However, before b can turn off, the pulse sent to con is handled first, so it briefly remembers all high pulses for its inputs and sends a low pulse to output. After that, flip-flop b turns off, which causes con to update its state and send a high pulse to output.

Finally, with a on and b off, push the button a fourth time:

button -low-> broadcaster
broadcaster -low-> a
a -low-> inv
a -low-> con
inv -high-> b
con -high-> output

This completes the cycle: a turns off, causing con to remember only low pulses and restoring all modules to their original states.

To get the cables warmed up, the Elves have pushed the button 1000 times. How many pulses got sent as a result (including the pulses sent by the button itself)?

In the first example, the same thing happens every time the button is pushed: 8 low pulses and 4 high pulses are sent. So, after pushing the button 1000 times, 8000 low pulses and 4000 high pulses are sent. Multiplying these together gives 32000000.

In the second example, after pushing the button 1000 times, 4250 low pulses and 2750 high pulses are sent. Multiplying these together gives 11687500.

Consult your module configuration; determine the number of low pulses and high pulses that would be sent after pushing the button 1000 times, waiting for all pulses to be fully handled after each push of the button. What do you get if you multiply the total number of low pulses sent by the total number of high pulses sent?
*/

use std::collections::{VecDeque, HashMap};

// a general type that captures all the conditions
pub enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(u128),
}

pub struct Module {
    pub module_type: ModuleType,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

impl Module {
    pub fn translate_from_str(value: &str, translation_map: &HashMap<String, usize>, default: usize) -> Module {
        // a bit hacky but for valid input, it's valid.
        let module_type = match value.chars().next().expect("got empty string") {
            '%' => ModuleType::FlipFlop(false),
            '&' => ModuleType::Conjunction(0),
            'b' => ModuleType::Broadcaster,
            _ => panic!("invalid module type"),
        };
        let outputs = value.split_once('>')
            .expect("line had no ->").1
            .split(',')
            .map(|s| *translation_map.get(s.trim())
            .unwrap_or(&default))
            .collect();
        Module {
            module_type,
            inputs: vec![],
            outputs,
        }
    }

    pub fn passive() -> Module {
        Module { inputs: vec![], outputs: vec![], module_type: ModuleType::Broadcaster }
    }

    pub fn push_pulse(&mut self, value: bool, src: usize) -> Option<bool> {
        match &mut self.module_type {
            ModuleType::Broadcaster => {
                Some(value)
            },
            ModuleType::Conjunction(state) => {
                let index = self.inputs.iter().enumerate().find_map(|(i, x)| if x == &src { Some(i) } else { None }).expect("input from unknown source.");
                if value {
                    *state |= 1 << index;
                }
                else {
                    *state &= !(1 << index);
                }
                Some(*state != (1 << self.inputs.len()) - 1)
            },
            ModuleType::FlipFlop(state) => {
                if !value {
                    *state = !*state;
                    Some(*state)
                }
                else {
                    None
                }
            },
        }
    }
}

pub struct Network {
    pub modules: Vec<Module>,
    pub broadcaster: usize,
}

impl Network {
    pub fn broadcast<F: FnMut(usize, bool)>(&mut self, value: bool, mut pulse_event: F) {
        assert!(if let ModuleType::Broadcaster = self.modules[self.broadcaster].module_type { true } else { false }, "broadcaster needs to be module type broadcaster");
        let mut queue = VecDeque::<(usize, bool)>::new();
        pulse_event(self.broadcaster, value);
        queue.push_back((self.broadcaster, self.modules[self.broadcaster].push_pulse(value, self.broadcaster).unwrap()));
        loop {
            let (index, pulse) = queue.pop_front().unwrap();
            for output_index in self.modules[index].outputs.clone() {
                pulse_event(output_index, pulse);
                if let Some(output) = self.modules[output_index].push_pulse(pulse, index) {
                    queue.push_back((output_index, output));
                }
            }
            if queue.len() == 0 {
                break;
            }
        }
    }

    pub fn connect_modules(&mut self) {
        for module in &mut self.modules {
            module.inputs.clear(); // reset the inputs vec
        }
        // create the inputs vecs
        for i in 0..self.modules.len() {
            for output in self.modules[i].outputs.clone() {
                self.modules[output].inputs.push(i);
            }
        }
    }

    pub fn state_str(&self) -> String {
        let mut state = String::new();
        for module in &self.modules {
            match module.module_type {
                ModuleType::Broadcaster => (),
                ModuleType::Conjunction(module_state) => {
                    for i in 0..module.inputs.len() {
                        state.push(if (module_state >> i) & 1 != 0 { '1' } else { '0' });
                    }
                    state.push(' ');
                },
                ModuleType::FlipFlop(module_state) => {
                    state.push(if module_state { 'H' } else { 'L' });
                }
            }
        }
        state
    }
}

pub fn get_module_name(value: &str) -> &str {
    if value.starts_with('b') {
        value
    }
    else {
        &value[1..]
    }.split_once('-').expect("line had no ->").0.trim()
}

#[test]
pub fn part1() {
    // The data is given in a weird format.
    // The conjunction needs to know how many inputs it has and has to actually be able to identify them.
    // Therefore the usual format for these types of problems is better, where each node only knows it's precursor.
    // However if we want to have a very large circuit with few nodes fired in every step,
    // then we have to now the next nodes as well.
    // So go for a bidirectional approach!
    //
    // To build that, read the data twice.
    // 1. get all the names of the nodes and make a mapping to indices
    // 2. read in the actual nodes

    // 811901844 is too low
    // 790988331 is too low
    // 886347020 correct
    // This problem was not well defined at all!

    use std::io;
    use std::collections::HashMap;

    let mut indices = HashMap::<String, usize>::new();
    let mut modules = vec!();
    let null_sink;
    {
        let mut lines = vec![];
        loop {
            let mut input = String::new();
            let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            if input.len() == 0 || read_bytes == 0 {
                break;
            }
            indices.insert(get_module_name(input).to_string(), lines.len());
            lines.push(input.to_string());
        }
        null_sink = indices.len();
        for line in lines {
            modules.push(Module::translate_from_str(&line, &indices, null_sink));
        }
        modules.push(Module::passive()); // output module
    }
    let broadcaster = *indices.get("broadcaster").expect("There was no broadcaster");
    let mut network = Network { modules, broadcaster };
    network.connect_modules();

    let mut low_high = [0, 0];
    // now do the steps
    for _ in 1..=1000 {
        network.broadcast(false, |to, pulse| {
            low_high[pulse as usize] += 1;
            if to == null_sink {
                //println!("{pulse} to {to}");
            }
        });
        println!("{}", network.state_str());
    }
    let [low, high] = low_high;
    println!("{low} low pulses have been sent.");
    println!("{high} high pulses have been sent.");
    println!("multiplied it's {}", low as u128 * high as u128);
}

/*
--- Part Two ---

The final machine responsible for moving the sand down to Island Island has a module attached named rx. The machine turns on when a single low pulse is sent to rx.

Reset all modules to their default states. Waiting for all pulses to be fully handled after each button press, what is the fewest number of button presses required to deliver a single low pulse to the module named rx?
*/

/*
simple test case
build a 4-bit counter

broadcaster -> a
%a -> b, e
%b -> c, e
%c -> d, e
%d -> e
&e -> rx

3874 low
3626 high
-> 14047124
15 button presses required for -low-> rx
*/

/*
This problem is NP-complete with a computer, but for my human brain it wasn't that hard.
1. look at the data and find patterns

broadcaster -> bx
%fn -> kn
%hr -> ff
%xc -> nx
%ff -> xc
%sk -> hr
%nx -> fn, rn
%mv -> fk, rn
%fx -> sk, rn
%fk -> rn, rv
%bx -> rn, fx
%kn -> rn, mv
%rv -> rn
&rn -> fn, hr, bx, ff, xc, sp, sk
&sp -> gf
&gf -> rx

broadcaster -> jq
%lp -> jm
%zb -> fb
%fb -> vp
%vp -> lp, jt
%qt -> lj, jt
%dt -> jt, zb
%lj -> jt, dt
%jq -> jt, qt
%xk -> jt, nk
%jm -> jt, xk
%nk -> jt, vk
%vk -> jt
&jt -> fb, zb, jq, sv, lp
&sv -> gf
&gf -> rx

broadcaster -> nv
%xb -> jf
%hx -> xb
%bc -> xm
%jf -> bc, mh
%nr -> cj, mh
%xm -> mh, gv
%nv -> mh, th
%gv -> mh, nr
%cj -> mh, vh
%vh -> mh, jh
%th -> mh, hx
%jh -> mh
&mh -> bc, qs, hx, xb, nv
&qs -> gf
&gf -> rx

broadcaster -> jp
%tx -> dx
%hp -> tx
%vr -> hp
%mb -> jc
%kt -> ct
%ph -> mb, pz
%ct -> kd, pz
%kd -> pz, pp
%dx -> pz, ph
%jp -> pz, vr
%jc -> pz, kt
%pp -> pz
&pz -> kt, pg, mb, vr, hp, jp, tx
&pg -> gf
&gf -> rx

&gf -> rx

clearly there are 4 groups of separate circuits.
So solve each one individually first.

3907 button presses are required.
3907 cycle length with low at [0]

3919 button presses are required.
3919 cycle length with low at [0]

4051 button presses are required.
4051 cycle length with low at [0]

3761 button presses are required.
3761 cycle length with low at [0]

Very simple case again! There was no guarantee this would happen...

lcm(3907, 3919, 4051, 3761)
= 233283622908263
*/

#[test]
pub fn part2() {
    // Tested it, brute forcing seems bad...
    // So I will have to use some chinese remainder again or something like that...
    // why is it always combinatorics and number theory?
    // Additionally, this system is turing complete,
    // so I'm solving the halting problem for a finite turing machine.

    use std::io;
    use std::collections::HashMap;

    let mut indices = HashMap::<String, usize>::new();
    let mut modules = vec!();
    let rx;
    {
        let mut lines = vec![];
        loop {
            let mut input = String::new();
            let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            if input.len() == 0 || read_bytes == 0 {
                break;
            }
            indices.insert(get_module_name(input).to_string(), lines.len());
            lines.push(input.to_string());
        }
        rx = indices.len();
        indices.insert("rx".to_string(), indices.len());
        let null_sink = indices.len();
        for line in lines {
            modules.push(Module::translate_from_str(&line, &indices, null_sink));
        }
        modules.push(Module::passive()); // rx module
        modules.push(Module::passive()); // output module
    }
    let broadcaster = *indices.get("broadcaster").expect("There was no broadcaster");
    let mut network = Network { modules, broadcaster };
    network.connect_modules();

    let mut btn_pressed = 0;
    let mut detect_cycle = None;
    let mut low_in_cycle = vec![];
    loop {
        btn_pressed += 1;
        let mut had_low_rx = false;
        network.broadcast(false, |to, pulse| {
            if to == rx {
                had_low_rx |= !pulse;
            }
        });
        if had_low_rx {
            if let Some(cycle_state) = &detect_cycle {
                if &network.state_str() == cycle_state {
                    println!("{btn_pressed} cycle length with low at {:?}", low_in_cycle);
                    break;
                }
                low_in_cycle.push(btn_pressed);
            }
            else {
                detect_cycle = Some(network.state_str());
                println!("{btn_pressed} button presses are required.");
                btn_pressed = 0;
                low_in_cycle.push(0);
            }
        }
    }
}