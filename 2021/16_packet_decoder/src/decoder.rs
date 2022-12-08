pub mod packet;
use bitvec::prelude::*;
use packet::{Packet, PacketType};

pub struct Decoder {
    raw_message: BitVec<Msb0, u8>,
}

impl Decoder {
    pub fn new(raw: &str) -> Self {
        let mut raw_message: BitVec<Msb0, u8> = BitVec::new();
        raw_message.resize(raw.len() * 4, false);
        for (index, value) in raw.chars().enumerate() {
            let raw_value = value.to_digit(16).unwrap() as u8;
            raw_message[index * 4..index * 4 + 4].store(raw_value);
        }
        Decoder { raw_message }
    }

    fn get_value_from_bits(bitfield: &BitSlice<Msb0, u8>) -> usize {
        let mut result = 0;
        let size = bitfield.len();
        for i in 0..size {
            if bitfield[i] {
                result += 1 << (size - 1 - i);
            }
        }
        result
    }

    fn parse_litteral(&self, i: usize, p: &mut Packet) -> usize {
        let mut i = i;
        let mut bit_result:BitVec<Msb0, u8> = BitVec::new();
        loop {
            bit_result.extend_from_bitslice(&self.raw_message[i+1..i+5]);
            i += 5;
            if !self.raw_message[i - 5] {
                break;
            }
        }
        p.update_value(Decoder::get_value_from_bits(&bit_result[..]));
        i
    }

    pub fn parse_packet(&self, version_count: &mut usize, current_offset: usize, p: &mut Packet) -> usize {
        let mut i = current_offset;

        let packet_version = Decoder::get_value_from_bits(&self.raw_message[i..i+3]);
        *version_count += packet_version as usize;
        i += 3;

        let packet_type = Decoder::get_value_from_bits(&self.raw_message[i..i+3]);
        p.set_type(match packet_type {
            4 => PacketType::LITERAL,
            0 => PacketType::SUM,
            1 => PacketType::PRODUCT,
            2 => PacketType::MIN,
            3 => PacketType::MAX,
            5 => PacketType::GT,
            6 => PacketType::LT,
            7 => PacketType::EQ,
            _ => panic!("Unknown packet type {}", packet_type)

        });
        i += 3;
        if packet_type == 4 {
            i = self.parse_litteral(i, p);
        } else {
            // Packet is an operator packet
            i += 1;
            if !self.raw_message[i - 1] {
                // Subpackets size is expressed as bits number
                // let subpackets_size = self.get_value_from_bits(i, 15);
                let subpackets_size = Decoder::get_value_from_bits(&self.raw_message[i..i+15]);
                i += 15;
                let current_offset = i;
                while i < current_offset + subpackets_size {
                    let mut new_packet = Packet::new(0, PacketType::LITERAL);
                    i = self.parse_packet(version_count, i, &mut new_packet);
                    p.append_child(new_packet);
                }
            } else {
                // Subpackets size is expressed as count
                let subpackets_count = Decoder::get_value_from_bits(&self.raw_message[i..i+11]);
                i += 11;
                for _ in 0..subpackets_count {
                    let mut new_packet = Packet::new(0, PacketType::LITERAL);
                    i = self.parse_packet(version_count, i, &mut new_packet);
                    p.append_child(new_packet);
                }
            }
        }
        i
    }

    pub fn get_versions_sum(&self) -> usize {
        let mut result = 0;
        let mut p = Packet::new(0, PacketType::LITERAL);
        self.parse_packet(&mut result, 0, &mut p);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::decoder::packet::PacketType;

    use super::Decoder;
    use super::Packet;

    #[test]
    fn test_convert_bits() {
        let decoder = Decoder::new("8A004A801A8002F478");
        assert_eq!(Decoder::get_value_from_bits(&decoder.raw_message[0..3]), 4);
        assert_eq!(Decoder::get_value_from_bits(&decoder.raw_message[3..6]), 2);
    }

    #[test]
    fn test_decode_simple() {
        let decoder = Decoder::new("8A004A801A8002F478");
        assert_eq!(decoder.get_versions_sum(), 16);
    }

    #[test]
    fn test_decode_standard() {
        let decoder = Decoder::new("620080001611562C8802118E34");
        assert_eq!(decoder.get_versions_sum(), 12);
    }

    #[test]
    fn test_decode_advanced() {
        let decoder = Decoder::new("C0015000016115A2E0802F182340");
        assert_eq!(decoder.get_versions_sum(), 23);
    }

    #[test]
    fn test_decode_complex() {
        let decoder = Decoder::new("A0016C880162017C3686B18A3D4780");
        assert_eq!(decoder.get_versions_sum(), 31);
    }
    
    #[test]
    fn test_parse_litteral() {
        let decoder = Decoder::new("D2FE28");
        let mut p = Packet::new(0, PacketType::LITERAL);
        decoder.parse_litteral(6, &mut p);
        assert_eq!(p.value(), 2021);
    }
    
    #[test]
    fn test_complete_simple() {
        let decoder = Decoder::new("C200B40A82");
        let mut p = Packet::new(0, PacketType::LITERAL);
        let mut version_count = 0;
        decoder.parse_packet(&mut version_count, 0, &mut p);
        assert_eq!(p.value(), 3);
    }
    
    #[test]
    fn test_complete_advanced() {
        let decoder = Decoder::new("04005AC33890");
        let mut p = Packet::new(0, PacketType::LITERAL);
        let mut version_count = 0;
        decoder.parse_packet(&mut version_count, 0, &mut p);
        assert_eq!(p.value(), 54);
    }
    
    #[test]
    fn test_complete_advanced_bis() {
        let decoder = Decoder::new("880086C3E88112");
        let mut p = Packet::new(0, PacketType::LITERAL);
        let mut version_count = 0;
        decoder.parse_packet(&mut version_count, 0, &mut p);
        assert_eq!(p.value(), 7);
    }
}
