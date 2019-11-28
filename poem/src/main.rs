extern crate search;
use std::env;
use search::Config;
use search::run;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = match Config::new(env::args(), false) {
        Ok(c) => c,
        Err(e ) => {
            println!("Application error: {}", e);
            process::exit(1)
        },
    };
    // run(config);
    if let Err(e) = search::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    println!("End of insensitive search!");
    // let config2 = Config::new(&args, true);
    // // config2.sensitive = true;
    // if let Err(e) = search::run(config2) {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // }
    // --snip--
}

// --snip--
