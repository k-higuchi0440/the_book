use std::env;
use std::process;
use the_book::lib_12;
use the_book::lib_12::Config;

fn main() {
    // chapeter 12
    // let args: Vec<String> = env::args().collect();
    //     let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // chapter 13 refactored code
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = lib_12::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
