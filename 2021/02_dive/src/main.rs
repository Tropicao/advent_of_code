use std::io;
use std::{io::BufRead, fs::File};
use std::error::Error;

struct Coordinates {
    position: u32,
    depth: u32,
    aim:u32
}

impl Coordinates {
    fn new() -> Self {
        Coordinates {
            position: 0,
            depth: 0,
            aim: 0
        }
    }
    fn change_position(&mut self, value: u32)
    {
        self.position += value;
        self.depth += value*self.aim;
    }

    fn increase_depth(&mut self, depth:u32)
    {
        self.aim += depth;
    }

    fn decrease_depth(&mut self, depth:u32)
    {
        self.aim -= depth;
    }

}

fn main() -> Result<(), Box<dyn Error>>{
    let path = "inputs.txt";
    let file = File::open(path)?;
    let mut coordinates = Coordinates::new();
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    for line in lines {
        let components: Vec<&str> = line.split_whitespace().collect();
        match components[0] {
            "forward" => coordinates.change_position(components[1].parse()?),
            "up" => coordinates.decrease_depth(components[1].parse()?),
            "down" => coordinates.increase_depth(components[1].parse()?),
            _ => panic!("Unknown command {}", components[0])
        }
    }
    println!("Pos : {}, depth : {} (product : {})", coordinates.position, coordinates.depth, coordinates.position*coordinates.depth);
    Ok(())
}
