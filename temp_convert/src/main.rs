use std::io;

fn main () {
    // celcius to fahrenheit = fahrenheit - 32 * 5/9
    // fahrenheit to celsius = celsius * 9/5 + 32


    // This is to know the temperature conversion the user is using

    //this is to know the temperature conversion type. when the user choose 1 it means they want a conversion from celsius to fahrenheit
    // If they choose 2 they want to convert from fahrenheit to celsius
    let mut user_input = String::new();

    println!("Choose what temperature you want to convert to");
    println!("Type 1. for Celsius to Fahrenheit");
    println!("Type 2. for Fahrenheit to Celsius");

      // Read the user input from standard input
      io::stdin()
      .read_line(&mut user_input)
      .expect("Failed to read line");

        // Trim whitespace and parse the input to i64
        // Note that in parsing, a number value is return hence, it is expected to state the data type.
    let trimmed_input = user_input.trim();
    let _choice: i64 = match trimmed_input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! Please enter a valid choice.");
            return;
        }

    };

    // Check if the user input is either 1 or 2, else display an error message
    if _choice != 1 && _choice != 2 {
        println!("Invalid choice! Please enter 1 or 2.");
    } else {
        println!("You chose this: {_choice}");
    }


    // Based on the user's choice, convert the temperature from Celsius to Fahrenheit or vice versa
    if _choice == 1 {
        println!("Enter the temperature in Celsius:");

        //this is the number in celsius to be converted to fahrenheit
        let mut temperature_celcius = String::new();

        io::stdin()
        .read_line(&mut temperature_celcius)
        .expect("Failed to read line");

        let trimmed_celcius = temperature_celcius.trim();
        let temp_celsius: i64 = match trimmed_celcius.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid temperature.");
                return;
            }
        };

        // this is where the actual conversion is done
        let temp_fahrenheit = temp_celsius * 9/5 + 32;
        println!("The temperature in Fahrenheit is: {}F", temp_fahrenheit);

    } else {
        println!("Enter the temperature in Fahrenheit:");

        //this is the number in celsius to be converted to fahrenheit
         let mut temperature_fahrenheit = String::new();

        io::stdin()
        .read_line(&mut temperature_fahrenheit)
        .expect("Failed to read line");

        let trimmed_fahrenheit = temperature_fahrenheit.trim();
        let temp_fahrenheit: i64 = match trimmed_fahrenheit.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a valid temperature.");
                return;
            }
        };

        // this is where the actual conversion is done
        let temp_celsius = (temp_fahrenheit - 32) * 5/9;
        println!("The temperature in Celsius is: {}C", temp_celsius);
    }

}
