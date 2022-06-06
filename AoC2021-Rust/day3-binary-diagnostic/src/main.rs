use std::io::{BufReader,BufRead};
use std::fs::File;
 
fn main() {

    //grab puzzle input and store in vector by each line
    let mut puzzle_vector = Vec::new();
    let file = File::open("pie.txt").unwrap();
    for line in BufReader::new(file).lines() {
        puzzle_vector.push(line.unwrap());
    }

    //solve part 1 (power consumption = gamma rate * epsilon rate)
    //  general idea is to add all the 1s together for a total count of them and see if it is
    //  greater than half of the total number of reports, if it is then gamma is 1, if not then gamma is 0
    let binary_line_len: usize = puzzle_vector.iter().max().unwrap().len();
    let half_binary_reports: usize = puzzle_vector.len() / 2;
    let mut gamma_rate_vector: Vec<&str> = vec!["0"; binary_line_len];
    let mut epsilon_rate_vector: Vec<&str> = vec!["0"; binary_line_len];

    let mut gamma_tracker: usize = 0; //track 1s for gamma

    //loop based on total binary bits
    for i in 0..binary_line_len {
        //loop through all the vector elements
        for x in 0..puzzle_vector.len() {
            if puzzle_vector[x].chars().nth(i) == "1".chars().nth(0) { //if the current bit is a 1 then update the gamma tracker
                gamma_tracker += 1;
            }
        }

        //check to see if the 1s are more than half the total diag reports and set gamma/epsilon vector accordingly
        if gamma_tracker > half_binary_reports { 
            gamma_rate_vector[i] = "1";
            epsilon_rate_vector[i] = "0";
        } else {
            gamma_rate_vector[i] = "0";
            epsilon_rate_vector[i] = "1";
        }

        //reset gamma tracking for next iteration
        gamma_tracker = 0;
    }

    //concatenate the gamma and epsilon vectors into single string
    let concat_gamma: &str = &gamma_rate_vector.concat();
    let concat_epsilon = &epsilon_rate_vector.concat();
    //convert strings to decimals from binary number
    let decimal_gamma = isize::from_str_radix(concat_gamma, 2).unwrap();
    let decimal_epsilon = isize::from_str_radix(concat_epsilon, 2).unwrap();
    //calculate power consumption from decimal values
    let power_consumption = decimal_gamma * decimal_epsilon;


    //print part 1 question/solution
    println!("\nP1: Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine? (Be sure to represent your answer in decimal, not binary.)");
    println!("\nAnswer: {}\n", power_consumption);


    //solve part 2 (life support = oxygen generator rating * CO2 scrubber rating)

    for i in 0..puzzle_vector.len() {
        println!("{}", puzzle_vector[i]);
    }

    let bittest = isize::from_str_radix(&puzzle_vector[0], 2).unwrap() << 4;
    println!("Bittest: {}", bittest);


    //print part 2 question/solution
    println!("\nP2: Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and CO2 scrubber rating, then multiply them together. What is the life support rating of the submarine? (Be sure to represent your answer in decimal, not binary.)");
    println!("\nAnswer");
}