fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let mut s2 = String::from("hello");
    // Error! Cannot borrow a mutable more than once at a time!
    // let r1 = &mut s2;
    // let r2 = &mut s2;
    // println!("{}, {}", r1, r2);

    // Error! This is also not allowed!
    // Cannot have mutable reference while immutable ones are in scope!
    // let r1 = &s2;
    // let r2 = &s2;
    // let r3 = &mut s2;
    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s2;
    let r2 = &s2;
    println!("{} and {}", r1, r2);
    let r3 = &mut s2;

    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world"); cannot modify what is borrowed!
    s.len()
}

// mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Dangling reference is not allowed
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//} s is dropped here