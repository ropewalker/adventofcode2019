use crate::intcode::{Computer, State};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

pub struct Nic {
    address: isize,
    computer: Computer,
    router: Arc<Mutex<HashMap<isize, VecDeque<isize>>>>,
    idles: Arc<Mutex<HashMap<isize, bool>>>,
    process_finished: Arc<Mutex<bool>>,
    packet: VecDeque<isize>,
}

impl Nic {
    pub fn init(
        address: isize,
        mut computer: Computer,
        router: Arc<Mutex<HashMap<isize, VecDeque<isize>>>>,
        idles: Arc<Mutex<HashMap<isize, bool>>>,
        process_finished: Arc<Mutex<bool>>,
    ) -> Nic {
        computer.provide_input(address);

        Nic {
            address,
            computer,
            router,
            idles,
            process_finished,
            packet: VecDeque::new(),
        }
    }

    pub fn generate_packets(&mut self) {
        let mut state = self.computer.compute();

        while state != State::Finished {
            {
                let process_finished = &mut *self.process_finished.lock().unwrap();
                if *process_finished {
                    break;
                }
            }

            match state {
                State::AwaitingInput => {
                    let idles = &mut *self.idles.lock().unwrap();
                    let idle = idles.entry(self.address).or_insert(false);

                    let router = &mut *self.router.lock().unwrap();
                    let inputs = router.entry(self.address).or_insert_with(VecDeque::new);

                    if inputs.is_empty() {
                        *idle = true;
                        self.computer.provide_input(-1);
                    } else {
                        *idle = false;
                        while !inputs.is_empty() {
                            self.computer.provide_input(inputs.pop_back().unwrap());
                        }
                    }
                }
                State::ProducedOutput => {
                    self.packet.push_front(self.computer.read_output().unwrap());

                    if self.packet.len() == 3 {
                        let router = &mut *self.router.lock().unwrap();

                        let dest_address = self.packet.pop_back().unwrap();
                        let x = self.packet.pop_back().unwrap();
                        let y = self.packet.pop_back().unwrap();

                        let inputs = router.entry(dest_address).or_insert_with(VecDeque::new);

                        inputs.push_front(x);
                        inputs.push_front(y);
                    }
                }
                _ => panic!("Wrong state!"),
            }

            state = self.computer.compute();
        }
    }
}
