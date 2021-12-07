mod args;
mod install;
mod prompt;
mod run;

use std::env;

fn main() {
    let args_vec: Vec<String> = env::args().collect();
    let args = args::parse_args(&args_vec[1..]);

    run::run(&args);
}
