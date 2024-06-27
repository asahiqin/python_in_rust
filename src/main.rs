#![feature(let_chains)]
extern crate core;

use crate::test::test;
mod ast;
mod test;
mod tools;

fn main() {
    test()
}
