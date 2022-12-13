use std::cmp::Ordering;
use std::convert::Infallible;
use std::iter::zip;
use std::str::FromStr;

use adventofcode2022::read_blank_line_delimited_blocks_as;

#[derive(Clone, Eq, PartialEq)]
enum PacketPart {
    List(Vec<PacketPart>),
    Number(i32),
}

fn compare(lhs: &PacketPart, rhs: &PacketPart) -> Ordering {
    use PacketPart::*;
    match (lhs, rhs) {
        (Number(l), Number(r)) if *l < *r => Ordering::Less,
        (Number(l), Number(r)) if *l > *r => Ordering::Greater,
        (Number(_), Number(_)) => Ordering::Equal,
        (Number(l), List(_)) => compare(&PacketPart::List(vec![PacketPart::Number(*l)]), rhs),
        (List(_), Number(r)) => compare(lhs, &PacketPart::List(vec![PacketPart::Number(*r)])),
        (List(l), List(r)) => {
            for (l_item, r_item) in zip(l, r) {
                match compare(l_item, r_item) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => (),
                }
            }

            if l.len() < r.len() {
                Ordering::Less
            } else if r.len() < l.len() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl std::fmt::Debug for PacketPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(v) => write!(f, "{:?}", v),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

fn parse_packet_part<'a>(mut s: &'a str, curr: &mut Vec<PacketPart>) -> &'a str {
    if s.is_empty() {
        return s;
    }
    if s.starts_with('[') {
        let mut next_vec = vec![];
        s = parse_packet_part(&s[1..s.len()], &mut next_vec);
        let next_list = PacketPart::List(next_vec);
        curr.push(next_list);
    }
    if s.starts_with(|c: char| c.is_ascii_digit()) {
        let num = s
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>();
        let parsed: i32 = num.parse().unwrap();
        curr.push(PacketPart::Number(parsed));
        return parse_packet_part(&s[num.len()..s.len()], curr);
    }
    if s.starts_with(',') {
        return parse_packet_part(&s[1..s.len()], curr);
    }
    if s.starts_with(']') {
        return &s[1..s.len()];
    }
    panic!("Unexpected state: {s}");
}

impl FromStr for PacketPart {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root_vec = vec![];
        assert!(s.starts_with('['));
        let remainder = parse_packet_part(&s[1..s.len()], &mut root_vec);
        if !remainder.is_empty() {
            panic!("Got leftover characters: {remainder}");
        }

        Ok(PacketPart::List(root_vec))
    }
}

#[derive(Clone, Debug)]
struct PacketPair(PacketPart, PacketPart);

impl FromStr for PacketPair {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();
        Ok(PacketPair(
            lines[0].parse().unwrap(),
            lines[1].parse().unwrap(),
        ))
    }
}

fn part1() {
    let input: Vec<PacketPair> = read_blank_line_delimited_blocks_as(13);
    let mut index_sum: usize = 0;
    for (idx, pair) in input.iter().enumerate() {
        if matches!(compare(&pair.0, &pair.1), Ordering::Less) {
            index_sum += idx + 1;
        }
    }
    println!("Part 1: {index_sum}");
}

fn part2() {
    let input: Vec<PacketPair> = read_blank_line_delimited_blocks_as(13);
    let mut all_packets: Vec<PacketPart> = vec![];
    for pair in input {
        all_packets.push(pair.0);
        all_packets.push(pair.1);
    }
    let key_packet_1 = PacketPart::List(vec![PacketPart::List(vec![PacketPart::Number(2)])]);
    let key_packet_2 = PacketPart::List(vec![PacketPart::List(vec![PacketPart::Number(6)])]);
    all_packets.push(key_packet_1.clone());
    all_packets.push(key_packet_2.clone());

    all_packets.sort_by(compare);

    let mut decoder_key = 1;
    for (i, packet) in all_packets.into_iter().enumerate() {
        if packet == key_packet_1 || packet == key_packet_2 {
            decoder_key *= i + 1;
        }
    }

    println!("Part 2: {decoder_key}");
}

fn main() {
    part1();
    part2();
}
