mod util;
mod even;
mod odd;
mod mixed_ops;

fn main() {
    let e1 = even::Even::try_from(300).unwrap();
    let e2 = even::Even::try_from(8).unwrap();
    let o1 = odd::Odd::try_from(9).unwrap();
    let o2 = odd::Odd::try_from(101).unwrap();

    println!("Evens: ({}, {})", e1, e2);
    println!("Odds: ({}, {})", o1, o2);

    println!();
    println!("{} + {} = ({})", e1, e2, e1 + e2);
    println!("{} + {} = ({})", o1, o2, o1 + o2);
    
    println!();

    println!("{} + {} = ({})", o1, e2, o1 + e2);
    println!("{} + {} = ({})", e1, o2, e1 + o2);

    println!();

    println!("{} * {} = ({})", e1, e2, e1 * e2);
    println!("{} * {} = ({})", o1, o2, o1 * o2);

    println!();
    
    println!("{} * {} = ({})", o1, e2, o1 * e2);
    println!("{} * {} = ({})", e1, o2, e1 * o2);
}
