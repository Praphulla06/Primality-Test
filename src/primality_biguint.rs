use num_bigint::BigUint;
use num_traits::ToPrimitive;
use num_traits::{One, Zero};

fn mod_exp(mut base: BigUint, mut exp: BigUint, modulus: &BigUint) -> BigUint {
    let mut result = BigUint::one();
    base %= modulus;

    while !exp.is_zero() {
        if &exp & BigUint::one() == BigUint::one() {
            result = (result * &base) % modulus;
        }
        base = (&base * &base) % modulus;
        exp >>= 1;
    }
    result
}

fn miller_rabin_primality_test(n: &BigUint, rounds: u32) -> bool {
    if *n < BigUint::from(2u32) {
        return false;
    }
    if *n == BigUint::from(2u32) || *n == BigUint::from(3u32) {
        return true;
    }
    if n % 2u32 == BigUint::zero() {
        return false;
    }

    let mut k = 0u32;
    let mut temp = n - BigUint::one();
    while &temp % 2u32 == BigUint::zero() {
        temp /= 2u32;
        k += 1;
    }
    let m = temp;

    for _ in 0..rounds {
        // Keep your RNG logic as is
        let a: BigUint =
            BigUint::from(rand::random::<u128>() % (*&n - 3u32).to_u128().unwrap() + 2);
        let mut b = mod_exp(a.clone(), m.clone(), n);

        if b == BigUint::one() || b == (n - BigUint::one()) {
            continue;
        }

        let mut is_witness = true;
        for _ in 1..k {
            b = mod_exp(b.clone(), BigUint::from(2u32), n);
            if b == (n - BigUint::one()) {
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
    let mut i = BigUint::from(170261731u64);
    let step = BigUint::from(2u32);
    let mut prime_count = 0;
    let interval = 100_000;

    loop {
        if miller_rabin_primality_test(&i, 5) {
            prime_count += 1;
            if prime_count % interval == 0 {
                println!("Prime #{}: {}", prime_count, i);
            }
        }
        i += &step;
    }
}
