extern crate id_code_ua;
extern crate scoped_threadpool;

use id_code_ua::*;
use scoped_threadpool::Pool;

fn main() {
    let threads: usize = 100;
    let step: u32 = (100_000_000 - ID_LOWEST) / threads as u32;

    let mut pool = Pool::new(threads as u32);

    pool.scoped(|scoped| {
        for piece in 0..(threads) {
            let begin = ID_LOWEST + piece as u32 * step;
            let end = begin + step;
            println!("{}: {} - {}", piece, begin, end);

            scoped.execute(move || {
                for id in begin..end {
                    let valid = is_valid(id);
                    if valid {
                        println!("{}", id);
                    }
                }
            });
        }
    });
}
