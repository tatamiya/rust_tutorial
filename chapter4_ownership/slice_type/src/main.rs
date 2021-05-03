fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let w1 = first_word(&s);
    // s.clear(); Error!
    println!("{}", w1);

    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("{}", word);
    let word = improved_first_word(&my_string[..]);
    println!("{}", word);
    let word = improved_first_word(&my_string);
    println!("{}", word);

    let my_string_literal= "hello world"; // &str, immutable reference
    let word = improved_first_word(&my_string_literal[..]);
    println!("{}", word);
    let word = improved_first_word(&my_string_literal);
    println!("{}", word);
    // Error!: let word = first_word(&my_string_literal);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
// more general
fn improved_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}