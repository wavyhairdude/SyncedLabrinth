use rand::Rng;
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc, Mutex};
use std::thread;
use std::time::Instant;

static NUM_GUESTS: u8 = 100;
static VERBOSE: bool = false;

/// Implementation that calls threads 1 by one to go into the maze
/// Similar to the party() implementation
pub fn cupcake() {
    // reference that can only be accessed in mutual exclusion; by one person
    let cake = Arc::new(Mutex::new(true));
    let finished = Arc::new(AtomicBool::new(false));
    
    let mut thread_list = Vec::new();

    let cake_ref = Arc::clone(&cake);
    let fin_ref = Arc::clone(&finished);
    // thread that will count guests that went in or out
    let master = std::thread::spawn(move || {
        let mut counter = 1;

        while counter < NUM_GUESTS {
            let mut t = cake_ref.lock().unwrap();

            // if cake is eaten, replace and count
            if !(*t) {
                counter += 1;
                *t = true;
            }

            // release lock on cupcake and wait until called on again
            drop(t);
            thread::park();
        }
        let t = fin_ref;
        println!("All guests have entered the maze");
        t.store(true, Ordering::SeqCst);
    });

    // create threads for rest of system
    for i in 1..NUM_GUESTS {
        let pr_finished = Arc::clone(&finished);
        let c_ref = Arc::clone(&cake);
        let handle = std::thread::spawn(move || {
            let mut visited: bool = false;
            let num = i;

            // if it hasn't eaten a cupcake, check if it can take one
            // and update self state
            while !visited {
                //print!("{} = ", num);
                let mut t = c_ref.lock().unwrap();
                if *t {
                    if VERBOSE {
                        println!("Changing {}", num);
                    }
                    *t = false;
                    visited = true;
                }

                drop(t);
            }

            // even though we have visited, still pass through room but don't do anything
            // just 'park' tp show thread has finished

            while !pr_finished.load(Ordering::SeqCst) {
                thread::park();
            }
        });
        thread_list.push(handle);
    }

    let curr = Instant::now();
    let mut r = rand::thread_rng();
    while !finished.load(Ordering::SeqCst ) {
        // get random thread to go through maze
        let next = r.gen_range(0..NUM_GUESTS - 1) as usize;
        // if counter thread, call seperately
        if next == 0 {
            master.thread().unpark();
        } else {
            // unsleep the thread to 
            let t = thread_list.get(next).expect("getting Thread");
            t.thread().unpark();
        }
    }

    // wait for only the counter thread
    // after this, finishing main will force-stop the rest of the threads
    let _ = master.join();
    println!("Exited with {:?}", curr.elapsed());
}

/// Implementation that doesn't self call threads
/// All threads run at the same time for randomness
/// Whomever gets the lock first will proceed
pub fn party() {
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
            while visited {
                let _ = temp.lock().unwrap();
            }
        });
    }

    let _ = master.join();

    println!(" Time taken: {:.2?}", curr.elapsed());
}
