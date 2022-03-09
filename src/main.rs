use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::{Builder, ThreadPool};

struct Ptr(*mut [i32]);

unsafe impl Send for Ptr {}

const NUM_ELEMENTS: usize = 100_000_000;
const NUM_THREADS: usize = 16;
const NUM_ELS_PER_SLICE: usize = NUM_ELEMENTS / NUM_THREADS;
const THREAD_STACK_SIZE: usize = 1_000_000;

fn main() {
    // time
    let start = Instant::now();
    let vec = run_safe();
    println!("Safe run time: {:?}", start.elapsed());
    assert_eq!(vec, vec![1; NUM_ELEMENTS]);
    let start = Instant::now();
    let vec = run_unsafe();
    println!("Unsafe run time: {:?}", start.elapsed());
    assert_eq!(vec, vec![1; NUM_ELEMENTS]);
}

fn run_safe() -> Vec<i32> {
    let vec = vec![0; NUM_ELEMENTS];
    let mutex = Arc::new(Mutex::new(vec));
    let pool = ThreadPool::new(NUM_THREADS);

    let mut ranges = Vec::<(usize, usize)>::new();
    let mut last_end = 0;
    for i in 0..NUM_THREADS {
        let start = last_end;
        let mut end = start + NUM_ELS_PER_SLICE;
        if (NUM_ELEMENTS - last_end) % (NUM_THREADS - i) != 0 {
            end += 1;
        }
        let range = (start, end);
        ranges.push(range);
        last_end = end;
    }

    for range in ranges {
        let mutex_clone = mutex.clone();
        pool.execute(move || {
            let mut vec = mutex_clone.lock().unwrap();
            for i in range.0..range.1 {
                vec[i] += 1 as i32;
            }
        });
    }

    pool.join();

    let vec = mutex.lock().unwrap().to_vec();
    vec
}

fn run_unsafe() -> Vec<i32> {
    let mut vec = vec![0; NUM_ELEMENTS];
    let pool = Builder::new()
        .num_threads(NUM_THREADS)
        .thread_stack_size(THREAD_STACK_SIZE)
        .build();

    let mut slices = Vec::<(Ptr, usize)>::new();
    let mut last_end = 0;
    for i in 0..NUM_THREADS {
        let start = last_end;
        let mut end = start + NUM_ELS_PER_SLICE;
        if (NUM_ELEMENTS - last_end) % (NUM_THREADS - i) != 0 {
            end += 1;
        }
        let slice = Ptr(&mut vec[start..end]);
        let size = end - start;
        slices.push((slice, size));
        last_end = end;
    }

    for mutex in slices {
        pool.execute(move || {
            let slice = mutex.0;
            let size = mutex.1;
            for i in 0..size {
                unsafe {
                    (*slice.0)[i] += 1 as i32;
                }
            }
        });
    }

    pool.join();

    vec
}
