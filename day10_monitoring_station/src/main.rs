use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const INPUT_PATH: &str = "input.txt";

fn read_input(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why.description())
    }

    String::from(s.trim())
}

fn main() {
    let s = read_input(INPUT_PATH);

    let mut asteroids: Vec<(i32, i32)> = Vec::new();

    for (x, line) in s.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push((y as i32, x as i32));
            }
        }
    }

    let mut visible_per_asteroid: HashMap<(i32, i32), i32> = HashMap::new();

    for asteroid in &asteroids {
        let mut lines: HashSet<(i32, i32)> = HashSet::new();

        'label: for another_asteroid in &asteroids {
            if *another_asteroid != *asteroid {
                let line_of_sight = (
                    another_asteroid.0 - asteroid.0,
                    another_asteroid.1 - asteroid.1,
                );

                for other_line in lines.iter() {
                    let pseudo_scalar =
                        line_of_sight.0 * other_line.1 - line_of_sight.1 * other_line.0;
                    let scalar = line_of_sight.0 * other_line.0 + line_of_sight.1 * other_line.1;

                    if pseudo_scalar == 0 && scalar >= 0 {
                        continue 'label;
                    }
                }

                lines.insert(line_of_sight);

                let count = visible_per_asteroid.entry(*asteroid).or_insert(0);
                *count += 1;
            }
        }
    }

    let base: (i32, i32) = visible_per_asteroid
        .into_iter()
        .max_by_key(|&(_x, count)| count)
        .unwrap()
        .0;

    asteroids.retain(|&x| x != base);

    asteroids = asteroids
        .iter()
        .map(|&(x, y)| (x - base.0, y - base.1))
        .collect();

    asteroids.sort_by(|&a, &b| {
        let pseudo_scalar = a.0 * b.1 - a.1 * b.0;
        let scalar = a.0 * b.0 + a.1 * b.1;

        let result = if pseudo_scalar == 0 && scalar >= 0 {
            let len_a_squared = a.0 * a.0 + a.1 * a.1;
            let len_b_squared = b.0 * b.0 + b.1 * b.1;

            len_a_squared.cmp(&len_b_squared)
        } else if a.0 * b.0 <= 0 {
            b.0.cmp(&a.0)
        } else if pseudo_scalar > 0 {
            Less
        } else {
            Greater
        };

        result
    });

    let (mut x, mut y) = (asteroids[0].0, asteroids[0].1);
    let mut lines: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut line: Vec<(i32, i32)> = Vec::new();
    line.push((x, y));
    lines.push(line);

    for asteroid in asteroids.iter().skip(1) {
        let pseudo_scalar = asteroid.0 * y - asteroid.1 * x;
        let scalar = asteroid.0 * x + asteroid.1 * y;

        if pseudo_scalar != 0 || scalar < 0 {
            x = asteroid.0;
            y = asteroid.1;

            let mut line: Vec<(i32, i32)> = Vec::new();
            line.push((x, y));
            lines.push(line);
        } else {
            let mut line = lines.remove(lines.len() - 1);
            line.push((asteroid.0, asteroid.1));
            lines.push(line);
        }
    }

    let mut k = 0;
    let mut i = 0;

    'label1: while i <= 200 {
        while k < lines.len() {
            if !lines[k].is_empty() {
                let (x, y) = lines[k].remove(0);
                i += 1;

                if i == 200 {
                    println!("{}", (x + base.0) * 100 + y + base.1);
                    break 'label1;
                }
            }

            k += 1;
        }
    }
}
