use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(
        "Factorial of {} is {:?}",
        args[1],
        factorial_imp(args[1].parse::<u8>().unwrap())
    );
}

#[allow(dead_code)]
fn factorial_rec(n: u8) -> u64 {
    if n == 0 {
        1
    } else {
        factorial_rec(n - 1) * n as u64
    }
}

#[allow(dead_code)]
fn factorial_iter(n: u8) -> u64 {
    (1..=n as u64).fold(1, |acc, x| acc * x)
}

#[allow(dead_code)]
fn factorial_imp(n: u8) -> u64 {
    let mut res = 1;
    for i in 1..=n as u64 {
        res *= i;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_0() {
        assert_eq!(1, factorial_imp(0));
        assert_eq!(1, factorial_iter(0));
        assert_eq!(1, factorial_rec(0));
    }

    #[test]
    fn factorial_1() {
        assert_eq!(1, factorial_imp(1));
        assert_eq!(1, factorial_iter(1));
        assert_eq!(1, factorial_rec(1));
    }

    #[test]
    fn factorial_5() {
        assert_eq!(120, factorial_imp(5));
        assert_eq!(120, factorial_iter(5));
        assert_eq!(120, factorial_rec(5));
    }
}
