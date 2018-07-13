use std::io;

fn main() {
    println!("Generate the nth Fibonacci Number");
    println!("Enter which number you'd like to generate:");
    // store return from our get_input function
    let n = get_input();
    println!("Input was {}", n); // check that get_input works
}

fn get_input() -> u32 {
    // init variable to store input
    let mut input = String::new();
    // read input and store in variable
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    // convert input from string to u32
    let input: u32 = match input.trim().parse() {
        // if input can be parsed into u32, return value
        Ok(num) => num,
        // if not, display an error message and
        // recall get_input function
        Err(e) => {
            println!("Please input a number ({})", e);
            get_input()
        }
    };
    input // move ownership back to main
}
