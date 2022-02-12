use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m !=0 && n != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    n
}

fn main() {
    let mut numbers = Vec::new();
    
    for arg in std::env::args().skip(1) {
        numbers.push(
            u64::from_str(&arg).expect("error parsing argument")
        );
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The gratest common divisor of {:?} is {}", numbers, d);
}
