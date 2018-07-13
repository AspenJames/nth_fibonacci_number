use std::io;

fn main() {
    println!("Generate the nth Fibonacci Number");
    println!("Enter which number you'd like to generate:");
    // store return from our get_input function
    let n = get_input();
    let fib_num = generate_fib_num(&n);
    println!("The Fibonacci Number at position {} is {}", n, fib_num);
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
            println!("Please input a positive, whole number ({})", e);
            get_input()
        }
    };
    input // move ownership back to main
}

fn generate_fib_num(n: &u32) -> u32 {
    if *n <= 1 {
        // return n if it is less than or equal to 1
        *n
    } else {
        // recursively add preceeding two values for n > 1
        generate_fib_num(&(n-1)) + generate_fib_num(&(n-2))
    }
}
