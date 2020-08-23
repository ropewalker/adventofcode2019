use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, Sub};
use std::path::Path;
use num::Integer;

const INPUT_PATH: &str = "input.txt";
//const TIME: usize = 1000;

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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Vector3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<x = {}, y = {}, z = {}>", self.x, self.y, self.z,)
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Asteroid {
    position: Vector3,
    velocity: Vector3,
}

impl Debug for Asteroid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos = <x = {}, y = {}, z = {}>, vel = <x = {}, y = {}, z = {}>",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z
        )
    }
}

impl Asteroid {
    fn new(position: Vector3) -> Asteroid {
        Asteroid {
            position,
            velocity: Vector3 { x: 0, y: 0, z: 0 },
        }
    }
}

fn parse_input(input: String) -> Vec<Asteroid> {
    let mut asteroids = Vec::new();

    for mut s in input.lines() {
        s = &s[1..s.len() - 1];

        let coordinates: Vec<isize> = s
            .trim()
            .split(',')
            .map(|x| {
                let y = x.trim();
                y[2..y.len()].parse::<isize>().unwrap()
            })
            .collect();

        asteroids.push(Asteroid::new(Vector3 {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        }));
    }

    asteroids
}

fn main() {
    let mut asteroids = parse_input(read_input(INPUT_PATH));
    let mut initial_state = asteroids.clone();
    let mut time_x: isize = 0;
    let mut time_y: isize = 0;
    let mut time_z: isize = 0;

    loop {
        let mut velocity_deltas = vec![Vector3{x: 0, y: 0, z: 0}; asteroids.len()];

        for i in 0..asteroids.len() {
            let asteroid1 = asteroids[i];

            for j in i + 1..asteroids.len() {
                let asteroid2 = asteroids[j];

                let x = if asteroid1.position.x < asteroid2.position.x {
                    1
                } else if asteroid1.position.x > asteroid2.position.x {
                    -1
                } else {
                    0
                };

                let d_velocity = Vector3 { x, y: 0, z: 0 };

                velocity_deltas[i] = velocity_deltas[i] + d_velocity;
                velocity_deltas[j] = velocity_deltas[j] - d_velocity;
            }
        }

        for i in 0..asteroids.len() {
            asteroids[i].velocity = asteroids[i].velocity + velocity_deltas[i];
            asteroids[i].position = asteroids[i].position + asteroids[i].velocity;
        }

        time_x += 1;

        if asteroids == initial_state {
            break;
        }
    }

    loop {
        let mut velocity_deltas = vec![Vector3{x: 0, y: 0, z: 0}; asteroids.len()];

        for i in 0..asteroids.len() {
            let asteroid1 = asteroids[i];

            for j in i + 1..asteroids.len() {
                let asteroid2 = asteroids[j];


                let y = if asteroid1.position.y < asteroid2.position.y {
                    1
                } else if asteroid1.position.y > asteroid2.position.y {
                    -1
                } else {
                    0
                };

                let d_velocity = Vector3 { x: 0, y, z: 0 };

                velocity_deltas[i] = velocity_deltas[i] + d_velocity;
                velocity_deltas[j] = velocity_deltas[j] - d_velocity;
            }
        }

        for i in 0..asteroids.len() {
            asteroids[i].velocity = asteroids[i].velocity + velocity_deltas[i];
            asteroids[i].position = asteroids[i].position + asteroids[i].velocity;
        }

        time_y += 1;

        if asteroids == initial_state {
            break;
        }
    }

    loop {
        let mut velocity_deltas = vec![Vector3{x: 0, y: 0, z: 0}; asteroids.len()];

        for i in 0..asteroids.len() {
            let asteroid1 = asteroids[i];

            for j in i + 1..asteroids.len() {
                let asteroid2 = asteroids[j];

                let z = if asteroid1.position.z < asteroid2.position.z {
                    1
                } else if asteroid1.position.z > asteroid2.position.z {
                    -1
                } else {
                    0
                };

                let d_velocity = Vector3 { x: 0, y: 0, z };

                velocity_deltas[i] = velocity_deltas[i] + d_velocity;
                velocity_deltas[j] = velocity_deltas[j] - d_velocity;
            }
        }

        for i in 0..asteroids.len() {
            asteroids[i].velocity = asteroids[i].velocity + velocity_deltas[i];
            asteroids[i].position = asteroids[i].position + asteroids[i].velocity;
        }

        time_z += 1;

        if asteroids == initial_state {
            break;
        }
    }

    println!("tx: {}", time_x);
    println!("ty: {}", time_y);
    println!("tz {}", time_z);
    println!("{}", time_z.lcm(&time_x.lcm(&time_y)));
}
