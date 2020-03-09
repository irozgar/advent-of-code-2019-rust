use std::fs::File;
use std::io::{BufRead, BufReader};

fn basic_module_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn module_fuel(mass: i32) -> i32 {
    let fuel = basic_module_fuel(mass);
    if fuel <= 0 {
        0
    } else {
        let fuel_for_fuel = module_fuel(fuel);
        fuel + fuel_for_fuel
    }
}

fn main() {
    let f = File::open("data/day1/input.txt").expect("Could not open input file");
    let fuel: i32 = BufReader::new(f)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            match line.parse() {
                Ok(m) => module_fuel(m),
                Err(_) => 0,
            }
        })
        .sum();
    println!("Fuel needed: {}", fuel);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_module_fuel() {
        assert_eq!(basic_module_fuel(12), 2);
        assert_eq!(basic_module_fuel(14), 2);
        assert_eq!(basic_module_fuel(1969), 654);
        assert_eq!(basic_module_fuel(100756), 33583);
    }

    #[test]
    fn test_module_fuel() {
        assert_eq!(module_fuel(14), 2);
        assert_eq!(module_fuel(1969), 966);
        assert_eq!(module_fuel(100756), 50346);
    }
}
