use rand::Rng;
use std::{sync::Arc, thread, sync::Mutex};
use std::time::Duration;

static N: u8 = 15;
static RUNTIME: u64 = 10;

pub fn vase() {
    let mut queue: Arc<Mutex<Vec<usize>>> = Arc::new( Mutex::new(Vec::with_capacity(100) ));
    let thread_identifiers: Arc<Mutex<Vec<thread::JoinHandle<()>>>> = Arc::new( Mutex::new( Vec::new() ) );
    let mut count:Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    
    // first thread initializes queue with numbers to use every couple of ms.
    let qref = Arc::clone(&queue);
    let _ = std::thread::spawn(move || {
        let mut t = rand::thread_rng();
        let mut count = 0;

        while count < 100
        {
            let rng = t.gen_range(0..N-1) as usize;
            qref.lock().unwrap().push(rng);
            count += 1;
        }
    });


    // creates threads and saves them to vector for easy referencing
    for i in 1..N {
        let q_copy = Arc::clone(&queue);
        let t_vec = Arc::clone(&thread_identifiers);
        let c_copy = Arc::clone(&count);
        // create thread and wait to be called by person
        let t = std::thread::spawn(move || {
            let num: u8 = i;
            while  1 != 0 {
                // wait until thread is called to do things
                thread::park();
                println!("Thread {} is awoken", num);

                let mut c:usize = 0;
                let mut rng = rand::thread_rng();
                c = rng.gen_range(0..100);

                let t = t_vec.lock().unwrap();
                let a = t.get(c);
                let mut z = a.expect("");
                z.thread().unpark();
            }
        });

        thread_identifiers.lock().unwrap().push(t);
    }

    
    {
        let a = Arc::clone(&thread_identifiers);
        let t = a.lock().unwrap();
        let c = t.get(0).expect("");
        c.thread().unpark();
    }
    


    // running time
    thread::sleep( Duration::from_secs(RUNTIME) );
    println!("Party ran for {} seconds", RUNTIME);
}

pub fn attempt()
{
    let counter = Arc::new(Mutex::new(0));
    let threadlist: Arc<Mutex<Vec<thread::JoinHandle<()>>>> = Arc::new( Mutex::new(Vec::new()) );
    let queue = Arc::new( Mutex::new(Vec::new() ) );

    // simulate 100 visiits
    for i in 0..100
    {
        let mut rng = rand::thread_rng();
        let mut t = queue.lock().unwrap();
        let n = rng.gen_range(0..N) as usize;
        t.push(n);
    }

    thread::sleep_ms(50);
    // create function for threads to call upon next threads
    for i in 0..N
    {
        let tRef = Arc::clone(&threadlist);
        let cRef = Arc::clone(&counter);
        let qRef = Arc::clone(&queue);
        let t = std::thread::spawn(move || {
           let num = i;
            while 1 > 0
            {
                thread::park();

                println!("Thread {} awakened", num);
                let mut c = cRef.lock().unwrap();
                let curr = qRef.lock().unwrap();
                // counter will be advanced
                *c = (*c + 1) % 100;
                let next = curr[*c];
                //println!("next: ({}, {})", *c, next );
                drop(c);
                drop(curr);

                let th = &tRef.lock().unwrap()[next];
                th.thread().unpark();
                print!("unparked");
            }
        });
        let a = threadlist.lock().unwrap().push(t);
    }

    {
        // start off running the first thread
        let a = &threadlist.lock().unwrap()[0];
        a.thread().unpark();
    }
    

    // simulate the party runnign for 10 hrs? 1hr = 1s
    thread::sleep(Duration::from_secs(10));
}
