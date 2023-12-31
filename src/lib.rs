mod spec;
mod job;
mod args;
mod command;

//use tera::{Tera, Context};
//use std::error::Error;

//use crate::job::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
