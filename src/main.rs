mod spec;
mod job;
mod args;
mod command;

//use std::error::Error;
use clap::Parser;

use crate::job::Job;
use crate::args::Args;

fn main() {
    let args = Args::parse();
    let script = Job::from_args(args).generate_script();
    println!("{}", script.unwrap());
}
