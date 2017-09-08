extern crate openssl;
use openssl::bn::*;

use std::io::{stdin, stdout, Write};
use std::thread::spawn;

extern crate num_cpus;

fn main() {
    print!("Base number: ",);
    stdout().flush().unwrap();
    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();
    n.pop();
    n.push_str(&"0".repeat(21));
    n.push('1');
    let mut n = match BigNum::from_dec_str(&n) { Ok(bn) => bn, _ => { println!("Bad number: {}", n); return }};
    for c in 0 .. num_cpus::get() {
        let n = n.to_vec();
        let delta = (num_cpus::get() + c) as u32;
        spawn(move || {
            let mut n = BigNum::from_slice(&n).unwrap();
            let mut ctx = BigNumContext::new().unwrap();
            let mut i = delta;
            loop {
                if n.is_prime_fasttest(10, &mut ctx, true).unwrap() {
                    println!("[{:02}] {}: Passed fast test", c, i);
                    if n.is_prime(20, &mut ctx).unwrap() {
                        println!("[{:02}] {}: Passed slow test", c, i);
                        if n.is_prime(100, &mut ctx).unwrap() {
                            println!("Got prime: {}", n.to_dec_str().unwrap());
                            ::std::process::exit(0);
                        } else { println!("[{:02} {}: Failed final test", c, i); continue }
                    } else { println!("[{:02} {}: Failed slow test", c, i); continue }
                }
                n.add_word(delta).unwrap();
                i += delta;
            }
        });
    }
    loop {}
}
