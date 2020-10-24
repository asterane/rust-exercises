use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    // panic!("crash and burn");

    let f = File::open("hello.txt");

    // Longer, with several match expressions
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Shorter, using library methods
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Real short, showing only unwrap, then expect
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = File::open("hello.txt")?;
}

// Verbose error propagation example
fn read_username_from_file_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

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

// Error propagation using the ? operator
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Really brief error propagation
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// One line
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
