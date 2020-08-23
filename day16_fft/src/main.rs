mod read_input;

use read_input::*;

const INPUT_PATH: &str = "input.txt";
const Q: usize = 10_000;
const OFFSET_SIZE: usize = 7;
const PHASES: usize = 100;

////((((n + 1) / k) as f64 * PI) / 2.0).sin() as isize)
//fn almost_sin(n: usize, k: usize) -> isize {
//    let base_pattern: Vec<isize> = vec![0, 1, 0, -1];
//    let period = base_pattern.len();
//
//    base_pattern[(n + 1) / (k + 1) % period]
//}

fn main() {
    let s = read_input(INPUT_PATH);

    let mut signal: Vec<isize> = Vec::new();

    for c in s.chars() {
        signal.push(c.to_digit(10).unwrap() as isize);
    }

    let p = signal.len();

    let offset = signal
        .iter()
        .take(OFFSET_SIZE)
        .fold(0, |o: usize, &i| o * 10 + i as usize);

    if offset * 2 < p {
        unimplemented!()
    }

    signal = signal
        .into_iter()
        .cycle()
        .take(p * Q)
        .skip(offset)
        .collect();

    for _phase in 0..PHASES {
        for i in (0..signal.len() - 1).rev() {
            signal[i] = (signal[i] + signal[i + 1]) % 10;
        }
    }

    for i in 0..8 {
        print!("{}", signal[i]);
    }

    println!();
}
