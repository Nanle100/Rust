//immutable variables
fn main() {
    let  x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// shadowing
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main(){
    let  space = 3;
    println!("space: {space}");
    let space = space + 1;
    println!("space: {space}");

    {
        let space = space * 3;
        println!("space: {space}");
    }

    let space = space + 3;
    println!("space: {space}");
}


// floating-point types
// floating-point numbers
fn main() {
    //the default type for a floating number is f64
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
 
}


//Numeric Operations
fn main() {
    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");

    let truncated = -5 / 3; // Results in -1
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

}


// Boolean types
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}


//The Character Type
// The 'char' type is the language’s most primitive alphabetic type
fn main() {
   // let c = 'z';
   // let z: char = 'ℤ'; // with explicit type annotation

    let heart_eyed_cat = '😻';
         println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

}


// compund types
//two primitive compound types: tuples & arrays.

//The Tuple Type
//A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
// Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, 
// and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

// The variable tup binds to the entire tuple because a tuple is considered a single compound element. 
// To get the individual values out of a tuple, 
// we can use pattern matching to destructure a tuple value, like this:
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of x is: {x}");
}



// The Array Type
// Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. 
// Unlike arrays in some other languages, arrays in Rust have a fixed length.


use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}