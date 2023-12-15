mod spec;
mod job;
mod arg;
mod command;

use std::error::Error;
use clap::Parser;

use crate::job::Job;
use crate::arg::Arg;

fn main() {
    let arg = Arg::parse();
    let script = Job::from_arg(arg).generate_script();
    println!("{}", script.unwrap());
}
