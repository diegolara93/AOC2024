use std::fs::File;
use std::io::prelude::*;
use std::vec;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let new_text = contents.split_whitespace();
    let mut nums: Vec<i32> = vec![];
    for words in new_text {
        println!("{}", words);
        nums.push(words.parse::<i32>().unwrap());
    }
    print!("{}", nums.len());
    Ok(())
}