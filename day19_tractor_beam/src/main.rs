mod intcode;
mod read_input;

use intcode::*;
use read_input::*;

fn calculate(computer: &mut Computer, x: isize, y: isize) -> isize {
    computer.set_input(vec![x, y]);
    computer.compute();
    let output = computer.read_output();
    computer.reset();

    output
}

fn main() {
    let mut computer = Computer::init(&read_input(), Vec::new());

    calculate(&mut computer, 1, 1);

    //    for y in 0..100 {
    //        for x in 0..100 {
    //            print!("{}", match calculate(&mut computer, x, y) {
    //                0 => '.',
    //                1 => '#',
    //                _ => '?'
    //            });
    //        }
    //        println!();
    //    }

    let mut x = 99;
    let mut y = 0;

    while calculate(&mut computer, x, y) == 0 {
        y += 1;
    }

    'label: loop {
        println!("({}, {})", x, y);

        while calculate(&mut computer, x, y) == 1 {
            if calculate(&mut computer, x - 99, y + 99) == 1 {
                println!("{}", (x - 99) * 10000 + y);
                break 'label;
            }

            x += 1;
        }

        y += 1;
    }
}
