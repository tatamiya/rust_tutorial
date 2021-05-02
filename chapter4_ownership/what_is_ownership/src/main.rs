fn main() {
    //let mut s = "hello";
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); s1 was moved into s2
    println!("{}, world!", s2);

    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    takes_ownership(s2);
    // println!("{}, world!", s2); s2 was moved
    println!("{}, world!", s3);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // OK because integers have Copy trait

    makes_copy(y);
    println!("x = {}, y = {}", x, y); // OK

    let s4 = gives_ownership();
    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5);
    println!("{}, world!", s4);
    // println!("{}, world!", s5); moved
    println!("{}, world!", s6);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
