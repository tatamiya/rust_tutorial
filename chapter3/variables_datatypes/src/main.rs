use std::io;

fn main() {
    //  let x = 5; compile error!
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    // let mut spaces = "   ";
    // spaces = spaces.len(); compile error!
    println!("The value of spaces is: {}", spaces);

    // 3.2 Data Types
    let integer: u32 = "42".parse().expect("Not a number!");
    println!("The value of integer is: {}", integer);

    let float32: f32 = "3.0".parse().expect("Not a number!");
    println!("The value of float is: {}", float32);

    let boolean: bool = "true".parse().expect("Not a boolean!");
    println!("The value of boolean is: {}", boolean);

    let character: char = "a".parse().expect("Not a character!");
    println!("The value of character is: {}", character);

    let character: char = "あ".parse().expect("Not a character!");
    println!("The value of character is: {}", character);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: ({}, {}, {})", x, y, z);
    println!("The 0th value of tuple is: {}", tup.0);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The first value of array is: {}", first);
    println!("The second value of array is: {}", second);

    //let b = [1, 2, 3.6] type error: every element of an array must have the same type

    let a: [f32; 3] = [3.3, 2.1, 5.0];
    println!("The first element of array is: {}", a[0]);

    let a = [3; 3];
    println!(
        "The first, socond, third elements of array are: {}, {}, {}",
        a[0], a[1], a[2]
    );

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    // 3.3 Functions
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
