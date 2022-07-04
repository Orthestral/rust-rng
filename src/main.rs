// imports
use rand::Rng;
use clap::Parser;
use std::process;

// All CLI parameters
#[derive(Parser)]
struct Cli {
    // The minimum value
    #[clap(value_parser)]
    low: String,
    // The maximum value
    #[clap(value_parser)]
    high: String,
    // If defined, how many times to generate numbers
    #[clap(default_value_t = String::from("1"), short, long)]
    count: String,
    // Whether or not the results should be unique
    #[clap(short, long, action)]
    unique: bool,
    // If defined, make floats instead of ints, and round to the value given
    #[clap(short, long)]
    float: Option<String>,
}

fn main() {
    // Parse arguments from the user
    let args = Cli::parse();

    if !args.count.parse::<i32>().is_ok() {
        eprintln!("Count must be an integer!");
        process::exit(1);
    }
    // Declare count and convert it to integer
    let count = args.count.parse::<i32>().unwrap();

    // If user hasn't invoked the float option
    if args.float == None {
        if !args.low.parse::<i32>().is_ok() || !args.high.parse::<i32>().is_ok() {
            eprintln!("Both ranges must be integers!");
            process::exit(1);
        }
        // Declare min/max and convert them to integers
        let low = args.low.parse::<i32>().unwrap();
        let high = args.high.parse::<i32>().unwrap();

        // Instantiate vector to hold previously generated numbers
        let mut nums_generated = Vec::<i32>::new();

        // Print random integers to the command line to the amount count is set
        for _ in 0..count {
            // Put generated number in variable
            let mut num = int_rng(low, high);

            // If user has invoked the unique flag
            if args.unique {
                // If specified count is over the total possible values
                if count > ((high - low) + 1) {
                    eprintln!("Count must not be over the total possible values!");
                    process::exit(1);
                }
                // If number isn't unique, pick a new one
                while !check_i32_for_uniqueness(&num, &nums_generated) {
                    num = int_rng(low, high);
                }
            }

            // Add number to list of previously generated numbers
            nums_generated.push(num);
            // Display number
            println!("{}", num);
        }
    } else {
        if !args.low.parse::<f64>().is_ok() || !args.high.parse::<f64>().is_ok() {
            eprintln!("Both ranges must be numbers!");
            process::exit(1);
        }
        // Declare min/max and convert them to floats
        let low = args.low.parse::<f64>().unwrap();
        let high = args.high.parse::<f64>().unwrap();

        // Instantiate vector to hold previously generated numbers
        let mut nums_generated = Vec::<f64>::new();

        // Declare decimal place and convert it to integer
        let round_to = args.float.unwrap().parse::<u32>().unwrap();

        // Print random floats to the command line to the amount count is set
        for _ in 0..count {
            // Put generated number in variable
            let mut num = round(float_rng(low, high), round_to);

            // If user has invoked the unique flag
            if args.unique {
                // If number isn't unique, pick a new one
                while !check_f64_for_uniqueness(&num, &nums_generated) {
                    num = round(float_rng(low, high), round_to);
                }
            }
            
            // Add number to list of previously generated numbers
            nums_generated.push(num);
            // Display number
            println!("{}", num);
        }
    }
}

// Return a random integer within a range
fn int_rng(low: i32, high: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low, high + 1)
}

// Return a random float within a range
fn float_rng(low: f64, high: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low, high)
}

// Round float to specified decimal place
fn round(num: f64, round_to: u32) -> f64 {
    let y = 10i32.pow(round_to) as f64;
    (num * y).round() / y
}

// Check if i32 value isn't in a vector
fn check_i32_for_uniqueness(num: &i32, array: &Vec<i32>) -> bool {
    for i in array {
        if i == num {
        return false;
        }
    }
    true
}

// Check if f64 value isn't in a vector
fn check_f64_for_uniqueness(num: &f64, array: &Vec<f64>) -> bool {
    for i in array {
        if i == num {
            return false;
        }
    }
    true
}