
use num::{pow, range_step};
use rand::random_range;

fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}

fn miller_rabin_primality_test(n: u128, rounds: u128) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut k = 0;
    let mut temp = n - 1;

    while temp % 2 == 0 {
        temp /= 2;
        k += 1;
    }

    let m = (n - 1) / pow(2, k);

    for _ in 0..rounds {
        let a = random_range(2..n - 1);
        let mut b = mod_exp(a, m, n);

        if b == 1 || b == n - 1 {
            continue;
        }

        let mut is_witness = true;
        for _ in 1..=k-1 {
            b = mod_exp(b, 2, n);
            if b == n - 1 {
                is_witness = false;
                break;
            }
        }
        if is_witness {
            return false;
        }
    }
    true
}

fn main() {
    let start: u128 = 89719361;
    let stop: u128 = 340282366920938463463374607431768211455;
    for i in range_step(start, stop, 2) {
        if miller_rabin_primality_test(i, 5) {
            println!("Prime, {:>6}", i);
        } 
    }
}
