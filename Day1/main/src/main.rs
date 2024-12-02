use std::fs::File;
use std::io::prelude::*;
use std::vec;

fn main() -> std::io::Result<()> {
    let mut ans: i32 = 0;
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let new_text = contents.split_whitespace();
    let mut nums: Vec<i32> = vec![];
    let mut nums1: Vec<i32> = vec![];
    let mut nums2: Vec<i32> = vec![];
    for words in new_text {
        nums.push(words.parse::<i32>().unwrap());
    }
    for i in 0..nums.len() {
        if i % 2 == 0 {
            nums1.push(nums[i]);
        }
        else {
            nums2.push(nums[i]);
        }
    }
    nums1.sort();
    nums2.sort();
    for i in 0..nums1.len() {
        ans += (nums1[i] - nums2[i]).abs();
    }
    println!("The answer for day one is: {}", ans);
    Ok(())
}