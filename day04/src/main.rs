use std::env;
use std::fs;

fn main() {
    // Load the input file
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("input.dat")
    };
    println!("Reading input file '{}'", file_path);
    let contents = fs::read_to_string(file_path).expect("Failed to read input file");

    // Define solution variables as mutable integers
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    // Create a matrix to hold the input data
    let n = contents.split("\n").into_iter().count();
    let mut matrix = vec![vec![0; n]; n];

    // Loop over codes in the input file
    for (i, entry) in contents.split("\n").enumerate() {
        match entry {
            "" => {} // Skip any empty strings
            _ => {
                for (j, c) in entry.chars().enumerate() {
                    // Set entry of array to 1 if @ and 0 if .
                    match c {
                        '@' => matrix[i][j] = 1,
                        '.' => matrix[i][j] = 0,
                        _ => panic!("Unexpected character in input"),
                    }
                }
            }
        }
    }

    // DEBUG: Print the matrix
    for row in &matrix {
        println!("{:?}", row);
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2)
}
