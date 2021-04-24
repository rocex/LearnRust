use std::fs;
use std::io::prelude::*;

fn main() {
    let a = "hello rust world";

    println!("Hello, world! {}", a);

    let b = "rust world";
    println!("Hello, world! {}", b);
    println!("Hello, world!");

    let c = 1;
    println!("Hello, world! {}", c);

    let ss = a.len();
    for i in 0..10 {
        println!("{2} '{0}' length is {1}", a, ss, i);
    }

    let text = fs::read_to_string("D:/DataHub/StudyRust/src/test.d").unwrap();
    println!("{}", text);
}
