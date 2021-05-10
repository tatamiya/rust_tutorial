use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("world.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("world.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("world.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap();
    // panic: let f = File::open("world.txt").unwrap();
    // panic: let f = File::open("world.txt").expect("Failed to open world.txt");

    // Propagating Errors
    let username = match read_username_from_file() {
        Ok(username) => username,
        Err(_) => String::from("John Doe"),
    };
    println!("{}", username);

    let f = File::open("world.txt");
    Ok(())
}
