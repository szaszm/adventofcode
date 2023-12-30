#![feature(slice_group_by)]

use std::convert::identity;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use arrayvec::ArrayString;

#[derive(PartialEq)]
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

struct Hand {
    hand: ArrayString<5>,
    bet: i32,
}

impl Hand {
    fn analyze(&self) {
        let cards = self.hand.chars().map(Card::from_char).collect::<Vec<_>>();
        let groups = cards.group_by(|a, b| a == b).into_iter()
            .map(|(key, group)| (key, group.count()))
            .collect::<Vec<_>>();
        for (card, cnt) in groups {
            println!("{} {}", card, cnt);
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
        hand.analyze();
    }

    Ok(())
}