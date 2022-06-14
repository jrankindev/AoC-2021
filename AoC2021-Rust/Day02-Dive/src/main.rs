use std::io::{BufReader,BufRead};
use std::fs::File;
 
fn main() {

    //grab puzzle input and store in vector by each line
    let mut puzzle_vector = Vec::new();
    let file = File::open("pi.txt").unwrap();
    for line in BufReader::new(file).lines() {
        puzzle_vector.push(line.unwrap());
    }

    //solve part 1
    let mut depth: i32 = 0; //track depth
    let mut distance: i32 = 0; //track distance

    for i in 0..puzzle_vector.len() {
        let split = puzzle_vector[i].split(" "); //split on whitespace
        let vector_split: Vec<&str> = split.collect(); //put split into new vector with two elements
        
        //determine direction and adjust depth or distance
        if vector_split[0] == "down" {
            depth += vector_split[1].parse::<i32>().unwrap();
        } else if vector_split[0] == "up" {
            depth -= vector_split[1].parse::<i32>().unwrap();
        } else if vector_split[0] == "forward" {
            distance += vector_split[1].parse::<i32>().unwrap();
        }
    }
    
    let answer = depth * distance;

    //print part 1 question/solution
    println!("P1: Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?");
    println!("Depth * Distance: {}", answer);

    //solve part 2
    let mut depth_p2: i32 = 0; //track depth
    let mut distance_p2: i32 = 0; //track distance
    let mut aim_p2: i32 = 0; //track aim

    for i in 0..puzzle_vector.len() {
        let split = puzzle_vector[i].split(" "); //split on whitespace
        let vector_split: Vec<&str> = split.collect(); //put split into new vector with two elements
        
        //determine direction and adjust depth, distance, and aim
        if vector_split[0] == "down" {
            aim_p2 += vector_split[1].parse::<i32>().unwrap();
        } else if vector_split[0] == "up" {
            aim_p2 -= vector_split[1].parse::<i32>().unwrap();
        } else if vector_split[0] == "forward" {
            distance_p2 += vector_split[1].parse::<i32>().unwrap();
            depth_p2 = aim_p2 * vector_split[1].parse::<i32>().unwrap() + depth_p2;
        }
    }
    
    let answer_p2 = depth_p2 * distance_p2;

    //print part 1 question/solution
    println!("\nP2: Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?");
    println!("Depth * Distance: {}", answer_p2);
}