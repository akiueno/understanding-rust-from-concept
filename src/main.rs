use std::cmp::Ordering;

enum Sign {
    Positive,
    Zero,
    Negative,
}

fn determine_sign(x: i32) -> Sign {
    match x.cmp(&0) {
        Ordering::Less => Sign::Negative,
        Ordering::Equal => Sign::Zero,
        Ordering::Greater => Sign::Positive,
    }
}

fn print_sign(s: Sign) {
    match s {
        Sign::Positive => println!("+"),
        Sign::Zero => println!("0"),
        Sign::Negative => println!("-"),
    }
}

fn main() {
    print_sign(determine_sign(10));
    print_sign(determine_sign(-2));
    print_sign(determine_sign(0));
}
