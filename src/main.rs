use std::time::Instant;

fn main() {
    sieve(100_000);
    for_primes(100_000);
    sieve(1_000_000);
    for_primes(1_000_000);
    sieve(10_000_000);
    for_primes(10_000_000);
    sieve(100_000_000);
    for_primes(100_000_000);
}

fn sieve(x: usize) {
    let start = Instant::now();

    let mut flags = vec![true; x + 1];
    flags[0] = false;
    flags[1] = false;
    let sqrt_x = (f64::sqrt(x as f64) + 0.1).ceil() as usize;
    for p in 2..=sqrt_x {
        if !flags[p] {
            continue;
        }
        for mult in ((p * p)..=x).step_by(p) {
            flags[mult] = false;
        }
    }

    let mut cnt = 0;
    for v in flags.iter().enumerate() {
        if *v.1 {
            cnt += 1
        }
    }

    let end = start.elapsed();
    println!("sieve {:9}->{:8} time:{}.{}",x,cnt,end.as_secs(), end.subsec_nanos() / 1_000_000);
}

fn for_primes(n: usize) {
    let start = Instant::now();

    let mut i = 0;
    let mut cnt = 0;
    while i < n {
        if if_prime(i) {
            cnt += 1;
        }
        i += 1;
    }

    let end = start.elapsed();
    println!("for   {:9}->{:8} time:{}.{}",n,cnt,end.as_secs(), end.subsec_nanos() / 1_000_000);
}

fn if_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let sn = (f32::sqrt(n as f32) + 0.1).ceil() as usize;
    let mut i = 3;
    while i <= sn {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }

    return true;
}
