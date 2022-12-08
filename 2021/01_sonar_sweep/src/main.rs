use std::io;
use std::{fs::File, io::BufRead};
use std::error::Error;

fn basic_count(path: &str) -> Result<u32, Box<dyn Error>>
{
    let file = File::open(path)?;
    let mut previous: Option<u32> = None;
    let mut increase_count = 0;
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    for line in  lines{
        let current = line.parse::<u32>()?;
        // println!("Checking {}", current);
        if previous.is_some() && current > previous.unwrap() {
            // println!("Depth has increased");
            increase_count +=1;
        }
        previous = Some(current);
    }
    println!("Depth increased {} times", increase_count);
    Ok(increase_count)
}

fn sliding_window_count(path: &str) -> Result<u32, Box<dyn Error>>
{
    let file = File::open(path)?;
    let mut previous_sum: Option<u32> = None;
    let mut increase_count = 0;
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    for line in  lines.windows(3){
        let current_sum = line.iter().map(|x| x.parse::<u32>().unwrap()).sum();
        // println!("Checking {}", current);
        if previous_sum.is_some() && current_sum > previous_sum.unwrap() {
            // println!("Depth has increased");
            increase_count +=1;
        }
        previous_sum = Some(current_sum);
    }
    println!("Depth increased {} times", increase_count);
    Ok(increase_count)
}

fn main() -> Result<(), Box<dyn Error>>{
    println!("Basic measure : depth has increased {} times", basic_count("inputs.txt")?);
    println!("Sliding window measure : depth has increased {} times", sliding_window_count("inputs.txt")?);
    Ok(())
}
