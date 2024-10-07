use std::io;


fn main() {
    // A fibonacci number is a sequence of numbers in which each number is the sum of the 
    // two preceding numbers, starting from 0 and 1

    // Prompt the user to enter the number of terms

    let mut user_input = String::new();
    println!("enter the nth value of your desired fibonacci number");
  

    io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

    let _nth: i64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };
    
println!("Fibonacci number at position {}: {}", _nth, fibonacci_iterative(_nth));


}


fn fibonacci_iterative(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}


