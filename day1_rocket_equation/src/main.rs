use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why.description())
    }

    let total_mass = s
        .lines()
        .map(|module_mass| {
            let module_mass: i32 = module_mass.parse().unwrap();
            let mut fuel_mass: i32 = 0;
            let mut fuel_delta: i32 = module_mass / 3 - 2;

            while fuel_delta > 0 {
                fuel_mass += fuel_delta;
                fuel_delta = fuel_delta / 3 - 2;
            }

            fuel_mass
        })
        .sum::<i32>();

    print!("{}", total_mass);
}
