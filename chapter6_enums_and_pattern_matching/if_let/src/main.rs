fn main() {
    let some_u8_value = Some(0u8);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // Same and more concise
    if let Some(3) = some_u8_value {
        println!("three");
    }
    println!("Hello, world!");
}
