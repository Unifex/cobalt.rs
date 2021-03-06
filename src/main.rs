extern crate cobalt;
extern crate getopts;

use getopts::{optopt, optflag, getopts, usage};
use std::os;

fn print_version() {
    // parse this from Cargo.toml
    let version = vec!["0", "0", "1"];

    println!("{}", version.connect("."));
}

fn main() {
    let args: Vec<String> = os::args();

    let opts = [
        optopt("s", "source", "Build from example/folder", "[example/folder]"),
        optopt("d", "destination", "Build into example/folder/build", "[example/folder]"),
        optflag("h", "help", "Print this help menu"),
        optflag("v", "version", "Display version")
    ];

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let source = if matches.opt_present("s") {
        Path::new(matches.opt_str("s").unwrap())
    } else {
        Path::new(".".to_string())
    };

    let dest   = if matches.opt_present("d") {
        Path::new(matches.opt_str("d").unwrap())
    } else {
        Path::new(".".to_string())
    };

    if matches.opt_present("h") {
        println!("{}", usage("\n\tcobalt build", opts));
        return;
    }

    if matches.opt_present("v") {
        print_version();
        return;
    }

    let command = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("{}", usage("\n\tcobalt build", opts));
        return;
    };

    match command.as_slice() {
        "build" => {
            println!("building from {} into {}", source.display(), dest.display());
            match cobalt::build(&source, &dest){
                Ok(_) => {},
                Err(e) => panic!("{}", e)
            };
        },

        _ => {
            println!("{}", usage("\n\tcobalt build", opts));
            return;
        }
    }
}
