mod magic_card;
mod read_input;
mod shuffle_process;

use crate::magic_card::MagicCard;
use crate::shuffle_process::Process;
use modinverse::*;
use num::Integer;
use std::collections::HashSet;

fn run_many(a: i128, b: i128, e: isize, l: i128) -> (i128, i128) {
    if e == 1 {
        return (a, b);
    } else if e % 2 == 0 {
        return run_many((a * a) % l, (a * b + b) % l, e / 2, l);
    } else {
        let (c, d) = run_many(a, b, e - 1, l);
        return ((a * c) % l, (a * d + b) % l);
    }
}

fn main() {
    let mut card = MagicCard::init(2019, 119_315_717_514_047);
    //    let mut card = MagicCard::init(2020, 119_315_717_514_047);
    let process = Process::init();

    let quick_process = card.quck_process(&process);
    println!("{:?}", &quick_process);
    println!(
        "{:?}",
        (quick_process.0 * 2020 + quick_process.1) % 119_315_717_514_047
    );

//    let a = modinverse(quick_process.0, card.deck_size).unwrap() as i128;
    let a = quick_process.0 as i128;
    let b = quick_process.1 as i128;
    let l = card.deck_size as i128;

    println!("a: {} b: {} l: {}", a, b, l);

    //    for i in 1..101_741_582_076_661 {
    //        result = (result + b) * a % l;
    //    }

    let (a, b) = run_many(a, b, 101_741_582_076_661, l);

    let mi_a = modinverse(a, card.deck_size as i128).unwrap();

    println!("{}", (2020 - b) * mi_a % l + l);
}
