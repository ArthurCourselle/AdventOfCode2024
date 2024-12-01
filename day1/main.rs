use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    let path = "input.txt";

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut left_col: Vec<i32> = Vec::new();
    let mut right_col: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                left_col.push(left);
                right_col.push(right);
            }
        }
    }

    // left_col
    part_one(left_col.clone(), right_col.clone());
    part_two(left_col.clone(), right_col.clone());

    Ok(())
}

fn part_one(mut left_col: Vec<i32>, mut right_col: Vec<i32>) {
    left_col.sort();
    right_col.sort();

    let mut res = 0;

    for (left, right) in left_col.iter().zip(right_col.iter()) {
        res += (left - right).abs();
    }
    
    println!("part1: {} is the result.", res);
}

fn part_two(left_col: Vec<i32>, right_col: Vec<i32>) {
    let mut res = 0;
    for &elt in left_col.iter() {
        let count_elt_right =  right_col.iter().filter(|&n| *n == elt).count();
        res += elt * count_elt_right as i32;
    }

    println!("part2: {} is the result.", res);
}