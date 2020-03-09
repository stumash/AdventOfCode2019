use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut total_fuel = 0;
    for line in stdin.lock().lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        let fuel = fuel_for_mass(mass);
        total_fuel += fuel + fuel_for_fuel(fuel);
    }

    println!("{}", total_fuel);
}

fn fuel_for_mass(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn fuel_for_fuel(fuel: i32) -> i32 {
    let mut total_extra_fuel = 0;
    let mut curr_extra_fuel = fuel_for_mass(fuel);
    while curr_extra_fuel > 0 {
        total_extra_fuel += curr_extra_fuel;
        curr_extra_fuel = fuel_for_mass(curr_extra_fuel);
    }
    total_extra_fuel
}
