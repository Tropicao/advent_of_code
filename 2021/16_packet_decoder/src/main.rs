mod decoder;
use decoder::Decoder;
use decoder::packet::{Packet, PacketType};
use std::fs::File;
use std::io::{BufReader, BufRead};
fn main() {
    let mut reader = BufReader::new(File::open("inputs.txt").unwrap());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let decoder = Decoder::new(&input);
    println!("Versions sum : {}", decoder.get_versions_sum());
    let mut p = Packet::new(0, PacketType::LITERAL);
    let mut version_count = 0;
    decoder.parse_packet(&mut version_count, 0, &mut p);
    println!("Final packet value {}", p.value());
}
