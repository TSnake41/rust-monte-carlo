use std::{env, thread};
use std::thread::JoinHandle;
use fastrand;

#[inline]
fn dans_cercle(x: f32, y: f32) -> bool {
    let a = x - 0.5;
    let b = y - 0.5;

    (a * a + b * b) <= 0.25
}

fn pi_thread(n: u32) -> u32 {
    let rng = fastrand::Rng::new();
    let mut n_dans_cercle = 0u32;

    for _ in 0..n {
        if dans_cercle(rng.f32(), rng.f32()) {
            n_dans_cercle += 1;
        }
    }

    n_dans_cercle
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: ./pi n t");
        return
    }

    let n: u32 = args[1].parse().unwrap();
    let t: u32 = args[2].parse().unwrap();

    println!("{} points avec {} threads ({} points par thread)", n, t, n / t);

    assert!(t > 0);

    let mut threads: Vec<JoinHandle<u32>> = Vec::with_capacity(t as usize);

    for _ in 0..t {
        threads.push(thread::spawn(move || { pi_thread(n / t) }));
    }
    
    let n_dans_cercle_total: u32 = threads
        .into_iter()
        .map(|thrd| thrd.join().unwrap())
        .sum();
    
    let pi = 4.0f32 * ((n_dans_cercle_total as f32) / (n as f32));

    println!("PI = {}", pi);
}
