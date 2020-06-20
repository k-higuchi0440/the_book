use std::fs::File;
use std::io::{ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let new_file = File::create("hello.txt");
                match new_file {
                    Ok(f) => f,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };

    let mut f = f;
    let mut s = String::new();
    let read_result = match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    };
    println!("read result: {}", read_result.unwrap_or_default());

    // we can use ? operator in a function returns Result (any types implements std::ops::Try)
    File::open("")?;

    Ok(())
}
