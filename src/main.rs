use std::sync::{Arc, Mutex};
use std::time::Instant;

static NUM_GUESTS: u8 = 12;
static VERBOSE: bool = false;

fn main() {
    // cupcake is initially present, set to true
    let cupcake = Arc::new(Mutex::new(true));

    // get current instance time for benchmarking
    let curr = Instant::now();

    // master thread that will count other threads status. So initialize this first
    // since we will be passing owneship, create reference copy
    let ms_cake = Arc::clone(&cupcake);
    let master = std::thread::spawn(move || {
        // count the number of switches from false to true that were performed
        let mut count: u8 = 1;

        // loop thread until all guests have visited
        while count < NUM_GUESTS {
            let mut cake = ms_cake.lock().unwrap();
            // if there is no cupcake, place another; count another visitor
            if !(*cake) {
                count += 1;
                *cake = true;
            }
        }

        print!(
            "\tAll {} guests have been processed through the maze",
            NUM_GUESTS
        );
    });

    // for the rest of the threads, use different function
    for i in 1..NUM_GUESTS {
        // copy reference to muteX
        let temp = Arc::clone(&cupcake);
        let _ = std::thread::spawn(move || {
            let mut visited: bool = false;

            // keep checking for cupcake if it hasn't taken one yet
            while !visited {
                if VERBOSE {
                    println!("thread {}", i);
                }

                let mut cake = temp.lock().unwrap();
                
                // if there is a cupcake, take it. set to false
                // for future loops 
                if *cake {
                    *cake = false;
                    visited = true;
                }
            }
            if VERBOSE {
                println!("visited thread {} ", i);
            }
            
            // have the persons visit the labrynth but do nothing
            while visited
            {
                let _ = temp.lock().unwrap();
            }
        });
    }

    let _ = master.join();
    
    println!(" Time taken: {:.2?}", curr.elapsed()); 
}
