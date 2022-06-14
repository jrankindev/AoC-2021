use std::io::{BufReader,BufRead};
use std::fs::File;
 
fn main() {

    //grab puzzle input and store in vector by each line
    let file = File::open("pi.txt").unwrap();
    let puzzle_vector: Vec<u32> = BufReader::new(file).lines().map(|line| line.unwrap().trim().parse().unwrap()).collect();

    //solve part 1
    let mut p1_count: i32 = 0;
    for i in 1..puzzle_vector.len() {
        if puzzle_vector[i] > puzzle_vector[i - 1] {
            p1_count += 1;
        }
    }

    //print part 1 question/solution
    println!("P1: How many measurements are larger than the previous measurement?");
    println!("{}", p1_count);

    //solve part 2
    let mut p2_count: i32 = 0;
    for i in 3..puzzle_vector.len() {
        if puzzle_vector[i] > puzzle_vector[i - 3] {
            p2_count += 1;
        }
    }

    //print part 2 question/solution
    println!("\nP2: Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum?");
    println!("{}", p2_count);

}