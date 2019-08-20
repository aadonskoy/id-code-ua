extern crate id_code_ua;
extern crate scoped_threadpool;

use id_code_ua::*;
use scoped_threadpool::Pool;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    let threads: usize = 100;
    let step: u32 = (100_000_000 - ID_LOWEST) / threads as u32;
    let res = Arc::new(Mutex::new(vec![]));

    let mut pool = Pool::new(threads as u32);
    pool.scoped(|scoped| {
        for ind in 0..threads {
            let clone = Arc::clone(&res);
            let begin = ID_LOWEST + ind as u32 * step;
            let end = begin + step;
            println!("{}: {} - {}", ind, begin, end);

            scoped.execute(move || {
                let mut wrkr = clone.lock().unwrap();
                for id in begin..end {
                    let valid = is_valid(id);
                    if !valid {
                        wrkr.push(id);
                    }
                }
            });
        }
    });
    let mut file = File::create("codes.txt").unwrap();
    let vv: Vec<u32> = res.clone().lock().unwrap().to_vec();
    for i in &vv {
        write!(file, "{}\n", i).expect("Can't write");
    }
}
