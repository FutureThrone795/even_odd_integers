mod even;
mod odd;
mod util;

fn main() {
    let e1 = even::Even::try_from(300).unwrap();
    let e2 = even::Even::try_from(8).unwrap();
    let o1 = odd::Odd::try_from(9).unwrap();
    let o2 = odd::Odd::try_from(101).unwrap();

    println!("Evens: ({}, {})", e1, e2);
    println!("Odds: ({}, {})", o1, o2);

    println!("Hello, world! ({})", e1 + e2);
    println!("Hello, world! ({})", o1 + o2);
}
