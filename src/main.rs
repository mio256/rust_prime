use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut i = 0;
    let mut cnt = 0;
    while i < 1_000_000 {
        if if_prime(i) {
            cnt += 1;
        }
        i += 1;
    }
    println!("cnt:{}", cnt);
    let end = start.elapsed();
    println!("time:{}.{}", end.as_secs(), end.subsec_nanos() / 1_000_000);
}

fn if_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    } else if n == 2 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let sn = (n as f32).sqrt() as i32;
    let mut i = 3;
    while i <= sn {
        if n % i == 0 {
            return false;
        }
        i += 2
    }

    return true;
}
