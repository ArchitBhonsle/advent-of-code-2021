use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
pub struct Packet {
    version: usize,
    type_id: usize,
    contents: PacketContents,
}

impl Packet {
    pub fn new(bits: &str) -> (Packet, &str) {
        let (version, rest) = bits.split_at(3);
        let version = usize::from_str_radix(version, 2).expect("could not parse version");

        let (type_id, rest) = rest.split_at(3);
        let type_id = usize::from_str_radix(type_id, 2).expect("could not parse type_id");

        let (contents, rest) = match type_id {
            4 => PacketContents::new_literal(rest),
            _ => PacketContents::new_operator(rest),
        };

        let packet = Packet {
            version,
            type_id,
            contents,
        };

        (packet, rest)
    }

    pub fn value(&self) -> usize {
        match self.contents {
            PacketContents::Literal(n) => n,
            PacketContents::Operator(ref v) => match self.type_id {
                0 => v.iter().map(|p| p.value()).sum(),
                1 => v.iter().map(|p| p.value()).product(),
                2 => v
                    .iter()
                    .map(|p| p.value())
                    .min()
                    .expect("could not find minimum"),
                3 => v
                    .iter()
                    .map(|p| p.value())
                    .max()
                    .expect("could not find maximum"),
                5 => {
                    if v[0].value() > v[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if v[0].value() < v[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if v[0].value() == v[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug)]
pub enum PacketContents {
    Literal(usize),
    Operator(Vec<Packet>),
}

impl PacketContents {
    pub fn new_literal(bits: &str) -> (PacketContents, &str) {
        let mut rest = bits;
        let mut res = Vec::new();

        loop {
            let (chunk, rest_inner) = rest.split_at(5);
            let (first, remaining) = chunk.split_at(1);
            rest = rest_inner;
            res.push(remaining);

            if first == "0" {
                break;
            }
        }

        let literal = usize::from_str_radix(&res.join(""), 2).expect("literal could not be parsed");

        (PacketContents::Literal(literal), rest)
    }

    pub fn new_operator(bits: &str) -> (PacketContents, &str) {
        let (length_type_id, rest) = bits.split_at(1);

        match length_type_id {
            "0" => PacketContents::new_operator_0(rest),
            "1" => PacketContents::new_operator_1(rest),
            _ => unreachable!(),
        }
    }

    fn new_operator_0(bits: &str) -> (PacketContents, &str) {
        let (total_length, rest) = bits.split_at(15);
        let total_length =
            usize::from_str_radix(total_length, 2).expect("could not parse total_length");

        let (mut packet_bits, rest) = rest.split_at(total_length);

        let mut packets = Vec::new();
        while packet_bits.len() > 0 {
            let (packet, packet_bits_inner) = Packet::new(packet_bits);
            packet_bits = packet_bits_inner;
            packets.push(packet);
        }

        (PacketContents::Operator(packets), rest)
    }

    fn new_operator_1(bits: &str) -> (PacketContents, &str) {
        let (number_of_packets, mut rest) = bits.split_at(11);
        let number_of_packets =
            usize::from_str_radix(number_of_packets, 2).expect("could not parse total_length");

        let mut packets = Vec::new();
        for _ in 0..number_of_packets {
            let (packet, rest_inner) = Packet::new(rest);
            rest = rest_inner;
            packets.push(packet);
        }

        (PacketContents::Operator(packets), rest)
    }
}

fn hex_to_bin_char(x: char) -> String {
    match x {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
    .to_string()
}

fn hex_to_bin(input: &str) -> String {
    input.chars().map(|c| hex_to_bin_char(c)).join("")
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Packet {
    Packet::new(&hex_to_bin(input)).0
}

#[aoc(day16, part1)]
pub fn solver_1(packet: &Packet) -> usize {
    fn version_sum(pkt: &Packet) -> usize {
        return pkt.version
            + match &pkt.contents {
                PacketContents::Literal(_) => 0,
                PacketContents::Operator(pkts) => pkts.iter().map(|p| version_sum(p)).sum(),
            };
    }

    version_sum(packet)
}

#[aoc(day16, part2)]
pub fn solver_2(packet: &Packet) -> usize {
    packet.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "8A004A801A8002F478";
    const INPUT_2: &str = "620080001611562C8802118E34";
    const INPUT_3: &str = "C0015000016115A2E0802F182340";
    const INPUT_4: &str = "A0016C880162017C3686B18A3D4780";

    #[test]
    fn test_parsing() {
        fn check(input: &str, version: usize, type_id: usize, rem: &str) {
            let input = &hex_to_bin(input);
            let (packet, rest) = Packet::new(input);

            assert_eq!(packet.version, version);
            assert_eq!(packet.type_id, type_id);
            assert_eq!(rest, rem);

            dbg!(packet);
        }

        check("D2FE28", 6, 4, "000");
        check("38006F45291200", 1, 6, "0000000");
        check("EE00D40C823060", 7, 3, "00000");
    }

    #[test]
    fn test_1() {
        assert_eq!(solver_1(&generator(INPUT_1)), 16);
        assert_eq!(solver_1(&generator(INPUT_2)), 12);
        assert_eq!(solver_1(&generator(INPUT_3)), 23);
        assert_eq!(solver_1(&generator(INPUT_4)), 31);
    }
}
