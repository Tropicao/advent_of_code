#![feature(drain_filter)]
use std::io;
use std::{io::BufRead, fs::File};
use std::error::Error;


fn get_power_consumption_from_report(path: &str)-> Result<(u32, u32), Box<dyn Error>> {
    let file = File::open(path)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let inputs_count = lines.len();
    let inputs_size = lines[0].len();
    println!("{} lines to parse, inputs are {} chars long", inputs_count, inputs_size);
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in 0..lines[0].len() {
        let on_count:u32 = lines.iter().map(|x| x.chars().nth(i).unwrap().to_digit(10).unwrap()).sum();
        println!("Bit {} : {} times a one", i, on_count);
        if on_count > inputs_count as u32/2 {
            gamma_rate += 1 << (inputs_size-1-i);
        }
        else {
            epsilon_rate += 1 << (inputs_size-1-i);
            
        }
    }
    Ok((gamma_rate, epsilon_rate))
}

fn get_oxygen_generator_rating(data: &Vec<String>) -> u32
{
    let input_size = data[0].len();
    let mut input = data.clone();
    let mut i = 0;
    while input.len() > 1 && i < input_size {
        let on_count:u32 = input.iter().map(|x| x.chars().nth(i).unwrap().to_digit(10).unwrap()).sum();
        let mut c = '0';
        if on_count >= input.len() as u32 - on_count {
            c = '1';
            println!("More 1 than 0 ({} on {})", on_count, input.len());
        }
        input = input.drain_filter(|x| x.chars().nth(i) == Some(c)).collect();
        i +=1;
        println!("After filter : {:?}", input);
    }
    match input.len() {
        1 => u32::from_str_radix(&input[0], 2).unwrap(),
        _ => 0
    }
}

fn get_co_scrubber_rating(data: &Vec<String>) -> u32
{
    let input_size = data[0].len();
    let mut input = data.clone();
    let mut i = 0;
    while input.len() > 1 && i < input_size {
        let on_count:u32 = input.iter().map(|x| x.chars().nth(i).unwrap().to_digit(10).unwrap()).sum();
        let mut c = '1';
        if on_count >= input.len() as u32 - on_count {
            c = '0';
            println!("More 1 than 0 ({} on {})", on_count, input.len());
        }
        input = input.drain_filter(|x| x.chars().nth(i) == Some(c)).collect();
        i +=1;
        println!("After filter : {:?}", input);
    }
    match input.len() {
        1 => u32::from_str_radix(&input[0], 2).unwrap(),
        _ => 0
    }
}

fn get_life_support_rating_from_report(path: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let file = File::open(path)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    Ok((get_oxygen_generator_rating(&lines), get_co_scrubber_rating(&lines)))
}

fn main() -> Result<(), Box<dyn Error>>{
    let (gamma, epsilon) = get_power_consumption_from_report("inputs.txt")?;
    let (life_support, co_scrubber) = get_life_support_rating_from_report("inputs.txt")?;
    println!("G={}, E={}, P={}", gamma, epsilon, gamma*epsilon);
    println!("O={}, C={}, L={}", life_support, co_scrubber, life_support*co_scrubber);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::get_power_consumption_from_report;
    use crate::get_life_support_rating_from_report;

    #[test]
    fn check_power_consumption() {
        let result = get_power_consumption_from_report("inputs_test.txt").unwrap();
        assert_eq!(result, (22, 9));
    }

    #[test]
    fn check_life_support_rating() {
        let result = get_life_support_rating_from_report("inputs_test.txt").unwrap();
        assert_eq!(result, (23, 10));
    }
}