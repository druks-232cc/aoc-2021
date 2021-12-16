use std::panic;

#[derive(Debug)]
enum Packet {
    Literal(Literal),
    Operator(Operator),
}

#[derive(Debug)]
struct Literal {
    version: u8,
    value: u64,
}

#[derive(Debug)]
struct Operator {
    version: u8,
    operator: u8,
    subpackets: Vec<Packet>,
}

impl Packet {
    fn get_version_sum(&self) -> u32 {
        match self {
            Packet::Operator(p) => p.get_version_sum(),
            Packet::Literal(p) => p.version as u32,
        }
    }

    fn calculate(&self) -> u64 {
        match self {
            Packet::Operator(p) => p.calculate(),
            Packet::Literal(p) => p.value,
        }
    }
}

impl Operator {
    fn get_version_sum(&self) -> u32 {
        self.subpackets
            .iter()
            .fold(self.version as u32, |acc, s| acc + s.get_version_sum())
    }

    fn calculate(&self) -> u64 {
        let v = self
            .subpackets
            .iter()
            .map(|p| p.calculate())
            .collect::<Vec<u64>>();
        match self.operator {
            0 => v.iter().sum(),
            1 => v.iter().product(),
            2 => *v.iter().min().unwrap(),
            3 => *v.iter().max().unwrap(),
            5 => (v[0] > v[1]) as u64,
            6 => (v[0] < v[1]) as u64,
            7 => (v[0] == v[1]) as u64,
            _ => 0,
        }
    }
}

fn from_bs(bs: &mut String, n: usize) -> u64 {
    u64::from_str_radix(&bs.drain(..n).collect::<String>(), 2).unwrap()
}

fn get_literal(bs: &mut String) -> u64 {
    let mut literal = 0;

    while from_bs(bs, 1) == 1 {
        literal = (literal + from_bs(bs, 4)) << 4;
    }

    literal + from_bs(bs, 4)
}

fn get_single_packet(bs: &mut String) -> Packet {
    let version = from_bs(bs, 3) as u8;
    let p_type = from_bs(bs, 3) as u8;
    match p_type {
        4 => Packet::Literal(Literal {
            version: version,
            value: get_literal(bs),
        }),
        _ => Packet::Operator(Operator {
            version: version,
            operator: p_type,
            subpackets: get_subpackets(bs),
        }),
    }
}

fn get_subpackets(bs: &mut String) -> Vec<Packet> {
    let len_type = from_bs(bs, 1);
    let mut subpackets = vec![];

    match len_type {
        0 => {
            let len = from_bs(bs, 15) as usize;
            let until = bs.len() - len;

            while bs.len() > until {
                subpackets.push(get_single_packet(bs));
            }
        }
        1 => {
            let nb_sub = from_bs(bs, 11);

            for _p in 0..nb_sub {
                subpackets.push(get_single_packet(bs));
            }
        }
        _ => panic!("not possible"),
    }

    subpackets
}

fn parse_input(input: &str) -> Packet {
    let mut bs: String;

    bs = input
        .trim()
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect();

    get_single_packet(&mut bs)
}

fn run(input: &str) -> Option<(u32, u64)> {
    let starting_packet = parse_input(input);

    let p1 = starting_packet.get_version_sum();

    let p2 = starting_packet.calculate();

    Some((p1, p2))
}

fn main() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    println!("Day16 p1 : {}", p1);
    println!("Day16 p2 : {}", p2);
}

#[test]
fn test_input() {
    let (p1, p2) = run(include_str!("input.txt")).unwrap();
    assert_eq!(883, p1);
    assert_eq!(1675198555015, p2);
}

#[test]
fn test_input_sample_1() {
    let (p1, _p2) = run("8A004A801A8002F478").unwrap();
    assert_eq!(16, p1);
}

#[test]
fn test_input_sample_2() {
    let (p1, _p2) = run("620080001611562C8802118E34").unwrap();
    assert_eq!(12, p1);
}

#[test]
fn test_input_sample_3() {
    let (p1, _p2) = run("C0015000016115A2E0802F182340").unwrap();
    assert_eq!(23, p1);
}

#[test]
fn test_input_sample_4() {
    let (p1, _p2) = run("A0016C880162017C3686B18A3D4780").unwrap();
    assert_eq!(31, p1);
}

#[test]
fn test_input_sample_5() {
    let (_p1, p2) = run("C200B40A82").unwrap();
    assert_eq!(3, p2);
}

#[test]
fn test_input_sample_6() {
    let (_p1, p2) = run("04005AC33890").unwrap();
    assert_eq!(54, p2);
}

#[test]
fn test_input_sample_7() {
    let (_p1, p2) = run("880086C3E88112").unwrap();
    assert_eq!(7, p2);
}

#[test]
fn test_input_sample_8() {
    let (_p1, p2) = run("CE00C43D881120").unwrap();
    assert_eq!(9, p2);
}

#[test]
fn test_input_sample_9() {
    let (_p1, p2) = run("D8005AC2A8F0").unwrap();
    assert_eq!(1, p2);
}

#[test]
fn test_input_sample_10() {
    let (_p1, p2) = run("F600BC2D8F").unwrap();
    assert_eq!(0, p2);
}

#[test]
fn test_input_sample_11() {
    let (_p1, p2) = run("9C005AC2F8F0").unwrap();
    assert_eq!(0, p2);
}

#[test]
fn test_input_sample_12() {
    let (_p1, p2) = run("9C0141080250320F1802104A08").unwrap();
    assert_eq!(1, p2);
}

#[test]
fn test_input_sample_13() {
    let (_p1, p2) = run("02008180210420C4200").unwrap();
    assert_eq!(10, p2);
}

#[test]
fn test_input_sample_14() {
    let (_p1, p2) = run("D2FE28").unwrap();
    assert_eq!(2021, p2);
}
