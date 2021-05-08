fn main() {
    // Create
    let mut s = String::new();

    // Same!
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // Update
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s = String::from("lo");
    s.push('l'); // not "l"
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Error!: let s3 = s1 + s2;
    let s3 = s1 + &s2; // &s2 turns into &s2[..] (deref coercion)
    println!("{}", s3);
    // not allowed!: println!("{}", s1);
    println!("{}", s2); // OK

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // OK, but...: let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("{}", s1); // OK

    // Indexing
    let s1 = String::from("hello");
    // not allowed!: let h = s1[0];

    let hello = String::from("Hola");
    println!("{}", hello.len()); // 4
    let hello = String::from("こんにちは");
    println!("{}", hello.len()); // 15

    // Slicing strings
    let hello = "こんにちは";
    let s = &hello[0..3];
    println!("{}", s); // こ

    // panic!: let s = &hello[0..2];

    // Methods for iterating
    for c in "こんにちは".chars() {
        println!("{}", c);
    }

    for b in "こんにちは".bytes() {
        println!("{}", b);
    }
}
