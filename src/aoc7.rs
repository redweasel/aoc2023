/*
--- Day 7: Camel Cards ---

Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

"Did you bring the parts?"

You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

"Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

"The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.

In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card follows this order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

    Five of a kind, where all five cards have the same label: AAAAA
    Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    High card, where all cards' labels are distinct: 23456

Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).

To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

This example shows five hands; each hand is followed by its bid amount. Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand. Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.

So, the first step is to put the hands in order of strength:

    32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
    KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
    T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.

Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5). So the total winnings in this example are 6440.

Find the rank of every hand in your set. What are the total winnings?
*/

use std::cmp::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card(pub char);

impl Card {
    pub fn value(&self) -> u32 {
        match self.0 {
            '2'..='9' => self.0.to_digit(10).unwrap() - 2,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("invalid card char"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPairs = 2,
    Pair = 1,
    None = 0
}

#[derive(Clone, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Card; 5],
}

impl Hand {
    pub fn new(s: &str) -> Hand {
        Hand { cards: s.chars().map(|c| Card(c)).collect::<Vec<Card>>().try_into().expect(&format!("{s} isn't a valid hand")) }
    }

    pub fn kind(&self) -> u32 {
        let mut c = self.cards.clone();
        c.sort();
        if c[0] == c[4] {
            6 // 5 equal
        }
        else if c[0] == c[3] || c[1] == c[4] {
            5 // 4 equal
        }
        else if c[0] == c[2] || c[2] == c[4] {
            if c[0] == c[1] && c[3] == c[4] {
                4 // full house
            }
            else {
                3 // 3 equal
            }
        }
        else if c[1] == c[3] {
            3 // 3 equal
        }
        else {
            c.windows(2).filter(|w| w[0] == w[1]).count() as u32
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Equal => {
                // same kind, so compare cards in order
                self.cards.cmp(&other.cards)
            },
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

#[test]
pub fn part1() {
    use std::io;

    let mut hands = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let (hand, bid) = input.trim().split_once(' ').expect("can't split the line in two");
        hands.push((Hand::new(hand), bid.parse::<u64>().ok().unwrap()));
    }
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    let sum: u64 = hands.into_iter().enumerate().map(|(i, (_, bid))| bid * (i as u64 + 1)).sum();
    println!("The bid sum is {sum}");
}

/*
--- Part Two ---

To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483

    32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
    KK677 is now the only two pair, making it the second-weakest hand.
    T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.

With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
*/

impl Card {
    pub fn j(&self) -> bool {
        self.0 == 'J'
    }

    pub fn cmp_j(&self, other: &Self) -> Ordering {
        if self.j() && other.j() {
            Ordering::Equal
        }
        else if self.j() {
            Ordering::Less
        }
        else if other.j() {
            Ordering::Greater
        }
        else {
            self.cmp(other)
        }
    }
}

impl Hand {
    pub fn kind_j(&self) -> u32 {
        let mut c = self.cards.clone();
        c.sort_by(|a, b| a.cmp_j(b));
        let kind = if c[3].j() {
            Kind::Five
        }
        else if c[2].j() {
            // 3 jokers -> at least 4 equal
            if c[3] == c[4] {
                Kind::Five
            }
            else {
                Kind::Four
            }
        }
        else if c[1].j() {
            // 2 jokers -> at least 3 equal, but no full house
            if c[2] == c[4] {
                Kind::Five
            }
            else if c[2] == c[3] || c[3] == c[4] {
                Kind::Four
            }
            else {
                Kind::Three
            }
        }
        else if c[0].j() {
            // 1 jokers -> at least a pair
            if c[1] == c[4] {
                Kind::Five
            }
            else if c[1] == c[3] || c[2] == c[4] {
                Kind::Four
            }
            else if c[1] == c[2] && c[3] == c[4] {
                Kind::FullHouse
            }
            else if c[1] == c[2] || c[2] == c[3] || c[3] == c[4] {
                Kind::Three
            }
            else {
                Kind::Pair
            }
        }
        else {
            if c[0] == c[4] {
                Kind::Five
            }
            else if c[0] == c[3] || c[1] == c[4] {
                Kind::Four
            }
            else if c[0] == c[2] || c[2] == c[4] {
                if c[0] == c[1] && c[3] == c[4] {
                    Kind::FullHouse
                }
                else {
                    Kind::Three
                }
            }
            else if c[1] == c[3] {
                Kind::Three
            }
            else {
                match c.windows(2).filter(|w| w[0] == w[1]).count() {
                    0 => Kind::None,
                    1 => Kind::Pair,
                    2 => Kind::TwoPairs,
                    _ => unreachable!(),
                }
            }
        };
        kind as u32
    }

    pub fn cmp_j(&self, other: &Self) -> Ordering {
        match self.kind_j().cmp(&other.kind_j()) {
            Ordering::Equal => {
                // same kind, so compare cards in order
                // tie breaking rule J < all
                self.cards.iter().zip(&other.cards).fold(Ordering::Equal, |acc, (a, b)| acc.then(a.cmp_j(b)))
                // tie breaking rule 23456789T < J < QKA
                //self.cards.cmp(&other.cards)
            },
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

#[test]
pub fn test_kinds() {
    for hand_str in ["23456", "23455", "23444", "23344", "22233", "22223", "22222"] {
        let base_hand = Hand::new(hand_str);
        for j in 0..5 {
            let mut hand = base_hand.clone();
            for i in 0..j {
                hand.cards[i] = Card('J');
            }
            let kind = hand.kind_j();
            for i0 in 0..5 {
                for i1 in 0..5 {
                    if i1 == i0 { continue; }
                    for i2 in 0..5 {
                        if i2 == i0 || i2 == i1 { continue; }
                        for i3 in 0..5 {
                            if i3 == i0 || i3 == i1 || i3 == i2 { continue; }
                            let i4 = (0 + 1 + 2 + 3 + 4) - (i0 + i1 + i2 + i3);
                            let perm_hand = Hand { cards: [hand.cards[i0], hand.cards[i1], hand.cards[i2], hand.cards[i3], hand.cards[i4]] };
                            assert_eq!(kind, perm_hand.kind_j(), "failed on hand {:?}", perm_hand.cards);
                        }
                    }
                }
            }
        }
    }
}

#[test]
pub fn part2() {
    use std::io;

    // story here:
    // I got this incorrect...
    // with the tie breaking rule J < all
    // I got 249897120 (too high)
    // with the tie breaking rule 23456789T < J < QKA
    // I got 250208959 (too high)
    // then I started to implement kind_j in a different way to have something to compare
    // turns out it was wrong somehow...
    // now that it's correct, with tie breaking rule J < all
    // I got 246894760 (finally correct)

    let mut hands = vec![];
    loop {
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.len() <= 1 || read_bytes == 0 {
            break;
        }
        let (hand, bid) = input.trim().split_once(' ').expect("can't split the line in two");
        hands.push((Hand::new(hand), bid.parse::<u64>().ok().unwrap()));
    }
    hands.sort_by(|a, b| a.0.cmp_j(&b.0));
    let sum: u64 = hands.into_iter().enumerate().map(|(i, (_, bid))| bid * (i as u64 + 1)).sum();
    println!("The bid sum is {sum}");
}