use rand::prelude::*;

const ITERS: usize = 10_000_000;

// fn main() {
// println!("Hello, world!");
// }

fn main() {
    println!("{}", ITERS);
    let mut rng = rand::thread_rng();
    let mut sum: f64 = 0.0;
    for _ in 0..ITERS {
        sum += rng.gen::<f64>();
    }
    println!("{}", sum);
}

