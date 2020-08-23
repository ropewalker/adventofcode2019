use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn path_to_sun<'a>(start: &'a str, orbits: &'a HashMap<&'a str, &'a str>) -> HashSet<&'a str> {
    let mut k = start;
    let mut path = HashSet::new();

    while orbits.contains_key(k) {
        k = orbits.get(k).unwrap();
        path.insert(k);
    }

    path
}

fn read_input() -> String {
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

    s
}

fn main() {
    let map = read_input();
    let orbits: HashMap<&str, &str> = map
        .lines()
        .map(|x| {
            let pair: Vec<&str> = x.split(')').collect();
            (pair[1], pair[0])
        })
        .collect();

    //    let mut count: i32 = 0;
    //
    //    for &i in orbits.keys() {
    //        let mut k = i;
    //
    //        while orbits.contains_key(k) {
    //            count += 1;
    //            k = orbits.get(k).unwrap();
    //
    //            println!("k = {}", k);
    //        }
    //    }
    //
    //    println!("{}", count);

    let path_from_me_to_sun: HashSet<&str> = path_to_sun("YOU", &orbits);
    let path_from_santa_to_sun: HashSet<&str> = path_to_sun("SAN", &orbits);

    println!(
        "{}",
        path_from_me_to_sun
            .symmetric_difference(&path_from_santa_to_sun)
            .collect::<HashSet<_>>()
            .len()
    );
}
