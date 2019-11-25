extern crate search;
use std::env;
use search::Config;
use search::run;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // run(config);
    if let Err(e) = run(config) {
    println!("Application error: {}", e);

    process::exit(1);
}
    // --snip--
}

// --snip--
