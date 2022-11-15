// use std::io::{stdin, stdout, Write};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    show_primes(100000);
    let end = start.elapsed();
    println!(
        "{}.{}秒経過しました。",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn show_primes(limit: i32) {
    let mut i = 0;
    let mut cnt = 0;
    while i < limit {
        if if_prime(i) {
            // println!("{}", i);
            cnt += 1;
        }
        i += 1;
    }
    println!("cnt {}", cnt);
}

// fn use_if_prime(){
//     let mut n=String::new();
//     print!("if_prime>");
//     stdout().flush().unwrap();
//     stdin().read_line(&mut n).expect("read error");
//     let n:i32=n.trim().parse().expect("parse error");
//     println!("n>{}",n);
//     println!("n is {} prime",if_prime(n));
// }

fn if_prime(n: i32) -> bool {
    if n == 0 || n == 1 {
        return false;
    } else if n == 2 {
        return true;
    } else {
        let mut i = 2;
        while i < n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        return true;
    }
}
