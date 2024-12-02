use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let mut input = File::open("./input.txt")?;
    let mut contents = String::new();
    let mut split_content: Vec<&str> = vec![];
    let mut answer = 0;
    input.read_to_string(&mut contents)?;
    for words in contents.split("\n") {
        if words.is_empty() {
            break;
        }
        split_content.push(words);
    }
    for lists in split_content.clone() {
        if check_if_sorted_increasing(lists) == true || 
        check_if_sorted_decreasing(lists) == true {
            answer += 1;
            println!("{:?}", lists);
        }
    }

    println!("{:?}", answer);
    Ok(())
}

fn check_if_sorted_increasing(input: &str) -> bool {
    let mut temp_list: Vec<i32> = vec![];
    for nums in input.split(" ") {
        temp_list.push(nums.parse::<i32>().unwrap());
    }
    for nums in 0..temp_list.len() {
        if nums == temp_list.len() - 1 {
            break;
        }
        if temp_list[nums] < temp_list[nums+1] 
        && (temp_list[nums] - temp_list[nums+1]).abs() <= 3 {
            if nums == temp_list.len() - 2 {
                return true;
            }
        }
        else { break; }
    }
    return false;
}

fn check_if_sorted_decreasing(input: &str) -> bool {
    let mut temp_list: Vec<i32> = vec![];
    for nums in input.split(" ") {
        temp_list.push(nums.parse::<i32>().unwrap());
    }
    for nums in 0..temp_list.len() {
        if nums == temp_list.len() - 1 {
            break;
        }
        if temp_list[nums] > temp_list[nums+1] 
        && (temp_list[nums] - temp_list[nums+1]).abs() <= 3 {
            if nums == temp_list.len() - 2 {
                return true;
            }
        } else {break;}
    }
    return false;
}
