use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

fn main() {
    const NUM_ELEMENTS: usize = 9;
    const NUM_THREADS: usize = 8;
    const NUM_ELS_PER_SLICE: usize = NUM_ELEMENTS / NUM_THREADS;

    let mut vec = Vec::<i32>::from([0; NUM_ELEMENTS]);
    let pool = ThreadPool::new(NUM_THREADS);


    let mut slices = Vec::<*mut [i32]>::new();
    for i in 0..NUM_THREADS {
        let start = i * NUM_ELS_PER_SLICE;
        let end = start + NUM_ELS_PER_SLICE;
        let slice = &mut vec[start..end] as *mut [i32];
        slices.push(slice);
    }

    unsafe {
        let mut count = 0;
        for slice in slices {
            // print slice
            if count == NUM_THREADS - 1 {
                println!("{}", (*slice)[1]);
            }
            count += 1;
        }
    }
    // unsafe {
    //     (*raw)[1] = 1;
    //     println!("{:?}", vec);
    // }
}

// fn main() {
//     let mutex = Arc::new(Mutex::new(vec![0; 10]));
//     let mut threads = vec![];

//     let mutex1 = mutex.clone();
//     threads.push(std::thread::spawn(move || {
//         let mut vec = mutex1.lock().unwrap();
//         vec[0] = 1;
//     }));
//     let mutex2 = mutex.clone();
//     threads.push(std::thread::spawn(move || {
//         let mut vec = mutex2.lock().unwrap();
//         vec[1] = 1;
//     }));
    
//     for thread in threads {
//         thread.join().unwrap();
//     }
//     println!("{:?}", mutex.lock().unwrap());
// }