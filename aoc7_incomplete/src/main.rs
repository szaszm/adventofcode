#![feature(slice_group_by)]

use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use arrayvec::ArrayString;

#[derive(Eq, Ord, PartialOrd, Hash, PartialEq, Copy, Clone, Debug)]
enum Card {
    _2,_3,_4,_5,_6,_7,_8,_9,T,J,Q,K,A
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::_2,
            '3' => Card::_3,
            '4' => Card::_4,
            '5' => Card::_5,
            '6' => Card::_6,
            '7' => Card::_7,
            '8' => Card::_8,
            '9' => Card::_9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!()
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Card::_2 => "2",
            Card::_3 => "3",
            Card::_4 => "4",
            Card::_5 => "5",
            Card::_6 => "6",
            Card::_7 => "7",
            Card::_8 => "8",
            Card::_9 => "9",
            Card::T => "T",
            Card::J => "J",
            Card::Q => "Q",
            Card::K => "K",
            Card::A => "A"
        };
        formatter.write_fmt(format_args!("[{}]", string))
    }
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card)
}

struct Hand {
    hand: ArrayString<5>,
    bet: i32,
}

impl Hand {
    fn group(&self) -> HashMap::<Card, i32> {
        let cards = self.hand.chars().map(Card::from_char).collect::<Vec<_>>();
        let mut groups = HashMap::<Card, i32>::new();
        for c in cards {
            *groups.entry(c).or_insert(0) += 1;
        }
        return groups;
    }

    fn group_flip(&self) -> Vec::<(i32, Card)> {
        let groups = self.group();
        let mut result = Vec::<(i32, Card)>::new();
        for (card, cnt) in groups {
            result.push((cnt, card));
        }
        result.sort_by(|(acnt, acard), (bcnt, bcard)| {
            if bcnt == acnt {
                return bcard.cmp(acard);
            }
            bcnt.cmp(acnt)
        });
        return result;
    }

    fn hand_type(hand: &Vec::<(i32, Card)>) -> HandType {
        match hand[0].0 {
            5 => HandType::FiveOfAKind(hand[0].1),
            4 => HandType::FourOfAKind(hand[0].1),
            3 => if hand[1].0 == 2 {
                HandType::FullHouse(hand[0].1, hand[1].1)
            } else {
                HandType::ThreeOfAKind(hand[0].1)
            },
            2 => if hand[1].0 == 2 {
                HandType::TwoPair(hand[0].1, hand[1].1)
            } else {
                HandType::OnePair(hand[0].1)
            },
            1 => HandType::HighCard(hand[0].1),
            _ => panic!()
        }
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_fmt(format_args!("Hand '{}' bet {}", self.hand.as_str(), self.bet))
    }
}

fn main() -> io::Result<()> {
    let file = File::open("example.txt")?;
    let reader = BufReader::new(file);
    let hands = reader.lines().map(|l| l.unwrap())
        .map(|l| {
            let spl: Vec<_> = l.split(" ").collect();
            Hand{
                hand: ArrayString::from(spl[0]).unwrap(),
                bet: spl[1].parse::<i32>().unwrap()
            }
        }).collect::<Vec<_>>();
    for hand in hands {
        println!("{}", hand);
        let groups = hand.group_flip();
        for (cnt, card) in &groups {
            println!("{} {}", card, cnt);
        }
        println!("hand type: {:?}", Hand::hand_type(&groups));
    }

    Ok(())
}
