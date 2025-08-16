use std::process;
use std::env;


use weegrep::{
    run,
    Config
};

fn main() {
    
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem Parsing args: {err}");
        process::exit(1);
    });
    
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


