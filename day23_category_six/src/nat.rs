use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Nat {
    address: isize,
    router: Arc<Mutex<HashMap<isize, VecDeque<isize>>>>,
    idles: Arc<Mutex<HashMap<isize, bool>>>,
    process_finished: Arc<Mutex<bool>>,
    last_sent_y: Option<isize>,
}

impl Nat {
    pub fn init(
        router: Arc<Mutex<HashMap<isize, VecDeque<isize>>>>,
        idles: Arc<Mutex<HashMap<isize, bool>>>,
        process_finished: Arc<Mutex<bool>>,
    ) -> Nat {
        Nat {
            address: 255,
            router,
            idles,
            process_finished,
            last_sent_y: None,
        }
    }

    pub fn is_network_idle(&self) -> bool {
        let mut network_is_idle = true;

        {
            let idles = &mut *self.idles.lock().unwrap();
            let router = &mut *self.router.lock().unwrap();

            for i in 0..50 {
                let idle = idles.entry(i).or_insert(false);
                network_is_idle = network_is_idle && *idle;
            }

            for i in 0..50 {
                let inputs = router.entry(i).or_insert_with(VecDeque::new);
                network_is_idle = network_is_idle && inputs.is_empty();
            }
        }

        network_is_idle
    }

    pub fn monitor(&mut self) {
        loop {
            if self.is_network_idle() {
                let router = &mut *self.router.lock().unwrap();
                let inputs = router.entry(self.address).or_insert_with(VecDeque::new);

                let y = inputs.pop_front().unwrap();
                let x = inputs.pop_front().unwrap();
                inputs.clear();

                let dest_inputs = router.entry(0).or_insert_with(VecDeque::new);

                dest_inputs.push_front(x);
                dest_inputs.push_front(y);

                println!("last Y sent: {:?}", &self.last_sent_y);
                println!("Y sent: {}", y);

                if let Some(last_y) = self.last_sent_y {
                    if last_y == y {
                        println!("Second y: {}", y);

                        let process_finished = &mut *self.process_finished.lock().unwrap();
                        *process_finished = true;

                        break;
                    } else {
                        self.last_sent_y = Some(y);
                    }
                } else {
                    self.last_sent_y = Some(y);
                }
            }
        }
    }
}
