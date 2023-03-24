mod args;
mod searcher;

use std::{env, process};

use args::Args;
use searcher::search;

fn main() {
    let args = Args::build(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let res = search(args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    for ele in res {
        println!("{}", ele);
    }
}
