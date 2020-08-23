use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Read;
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

    let wires: Vec<&str> = s.lines().collect();
    let mut wire_coordinates = [HashMap::new(), HashMap::new()];

    for k in 0..=1 {
        let mut current_coord = (0, 0);
        let mut total_distance = 0;

        for leg in wires[k].split(',') {
            match &leg[0..1] {
                "R" => {
                    let distance: i32 = leg[1..leg.len()].parse().unwrap();
                    for _i in 1..=distance {
                        current_coord = (current_coord.0 + 1, current_coord.1);
                        total_distance += 1;
                        wire_coordinates[k]
                            .entry(current_coord)
                            .or_insert(total_distance);
                    }
                }
                "U" => {
                    let distance: i32 = leg[1..leg.len()].parse().unwrap();
                    for _i in 1..=distance {
                        current_coord = (current_coord.0, current_coord.1 + 1);
                        total_distance += 1;
                        wire_coordinates[k]
                            .entry(current_coord)
                            .or_insert(total_distance);
                    }
                }
                "L" => {
                    let distance: i32 = leg[1..leg.len()].parse().unwrap();
                    for _i in 1..=distance {
                        current_coord = (current_coord.0 - 1, current_coord.1);
                        total_distance += 1;
                        wire_coordinates[k]
                            .entry(current_coord)
                            .or_insert(total_distance);
                    }
                }
                "D" => {
                    let distance: i32 = leg[1..leg.len()].parse().unwrap();
                    for _i in 1..=distance {
                        current_coord = (current_coord.0, current_coord.1 - 1);
                        total_distance += 1;
                        wire_coordinates[k]
                            .entry(current_coord)
                            .or_insert(total_distance);
                    }
                }
                _ => panic!("wrong direction!"),
            }
        }
    }

    println!(
        "{}",
        wire_coordinates[0]
            .keys()
            .collect::<HashSet<&(i32, i32)>>()
            .intersection(&wire_coordinates[1].keys().collect::<HashSet<&(i32, i32)>>())
            .map(|i| wire_coordinates[0].get(i).unwrap() + wire_coordinates[1].get(i).unwrap())
            .min()
            .unwrap()
    );
}
