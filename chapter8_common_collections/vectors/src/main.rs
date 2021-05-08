fn main() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3]; // inferred as Vec<i32>

    // Updating a Vector
    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // Reading elements
    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &v4[0];
    // not allowed here!: v4.push(6);
    println!("The first element is {}", first);
    v4.push(6); // OK

    // Iterating
    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }

    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
    }
    for i in &v6 {
        println!("{}", i);
    }

    // Using an Enum to Store Multiple Types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
