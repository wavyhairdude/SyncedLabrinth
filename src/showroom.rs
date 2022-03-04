use rand::Rng;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

static N: u8 = 4;
static RUNTIME: u64 = 10;

pub fn vase() {
    let mut queue = Vec::new();
    //let mut thread_list = Vec::new();

    // loop through each person
    for i in 1..N {
        let num: u8 = i;
        let keep_alive: bool = true;
        // create thread and wait
        let t = std::thread::spawn(move || {
            while true {
                println!("Thread {} is spawned", num);
                thread::park();
            }
        });
        queue.push(t);
    }

    // add 20 random people that are waiting
    for i in 1..20 {
        let mut rng = rand::thread_rng();
        let num: usize = rng.gen_range(0..N - 1) as usize;

        let t = queue[num].thread().unpark();
        thread::sleep_ms(2000);
    }

    // running time
    //thread::sleep( Duration::from_secs(RUNTIME) );
}
