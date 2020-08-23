mod intcode;
mod nat;
mod nic;
mod read_input;

use crate::nat::Nat;
use intcode::Computer;
use nic::Nic;
use read_input::*;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let computer = Computer::init(&read_input());
    let router: Arc<Mutex<HashMap<isize, VecDeque<isize>>>> = Arc::new(Mutex::new(HashMap::new()));
    let idles: Arc<Mutex<HashMap<isize, bool>>> = Arc::new(Mutex::new(HashMap::new()));
    let process_finished: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let mut handles = vec![];

    {
        let router = router.clone();
        let idles = idles.clone();
        let process_finished = process_finished.clone();

        let handle = thread::spawn(move || {
            let mut nat = Nat::init(router, idles, process_finished);
            nat.monitor();
        });

        handles.push(handle);
    }

    for i in 0..50 {
        let computer = computer.clone();
        let router = router.clone();
        let idles = idles.clone();
        let process_finished = process_finished.clone();

        let handle = thread::spawn(move || {
            let mut nic = Nic::init(i, computer, router, idles, process_finished);
            nic.generate_packets();
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
