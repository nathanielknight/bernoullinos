const LIMIT: usize = 13;

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        sequence_product(1, n)
    }
}

fn sequence_product(start: u64, stop: u64) -> u64 {
    if start > stop {
        return sequence_product(stop, start);
    }
    let mut result = 1;
    for i in start..=stop {
        result *= i;
    }
    result
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(4), 24);
}

/// n choose r, implemented with factorials but with some cancellations.
fn choose(n: u64, r: u64) -> f64 {
    if r == 0 || r == n || n == 0 {
        return 1.0;
    }
    if n - r > r {
        sequence_product(n - r + 1, n) as f64 / factorial(r) as f64
    } else {
        sequence_product(r + 1, n) as f64 / factorial(n - r) as f64
    }
}

#[test]
fn test_choose() {
    assert_eq!(choose(0, 0), 1.0);
    assert_eq!(choose(1, 0), 1.0);
    assert_eq!(choose(1, 1), 1.0);
    assert_eq!(choose(2, 0), 1.0);
    assert_eq!(choose(2, 1), 2.0);
    assert_eq!(choose(2, 2), 1.0);
    assert_eq!(choose(3, 0), 1.0);
    assert_eq!(choose(3, 1), 3.0);
    assert_eq!(choose(3, 2), 3.0);
    assert_eq!(choose(3, 3), 1.0);
    assert_eq!(choose(4, 0), 1.0);
    assert_eq!(choose(4, 1), 4.0);
    assert_eq!(choose(4, 2), 6.0);
    assert_eq!(choose(4, 3), 4.0);
    assert_eq!(choose(4, 4), 1.0);
}

/// Negative one to the power of x.
fn minusonepow(x: u64) -> i64 {
    if x % 2 == 0 {
        1
    } else {
        -1
    }
}

#[test]
fn test_minusonepow() {
    assert_eq!(minusonepow(0), 1);
    assert_eq!(minusonepow(1), -1);
    assert_eq!(minusonepow(2), 1);
    assert_eq!(minusonepow(3), -1);
}

fn bneg(m: usize) -> f64 {
    if m > 1 && m % 2 == 1 {
        return 0.0;
    }
    fn term(m: u64, k: u64, v: u64) -> f64 {
        minusonepow(v) as f64 * choose(k, v) * (v.pow(m as u32) as f64 / (k + 1) as f64) as f64
    }
    let mut result: f64 = 0.0;
    for k in 0..=m {
        for v in 0..=k {
            result += term(m as u64, k as u64, v as u64);
        }
    }
    result
}

fn main() {
    for m in 0..LIMIT {
        println!("B({}) = {}", m, bneg(m));
    }
}
