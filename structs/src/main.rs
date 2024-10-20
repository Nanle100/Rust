//A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. 
//If you’re familiar with an object-oriented language, a struct is like an object’s data attributes


//to use a struct, we must first of all create an instance of the struct like this 
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//then we can create an instance of that struct by specifying concrete values for each of the fields
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

   user1.email = String::from("anotheremail@example.com");
   let print = user1.active;
    println!("User 1 email: {}", user1.email); // Output: User 1 email: anotheremail@example.com
    println!("User 1 active: {}", print); // Output: User 1 active: true
}


//It makes sense to name the function parameters with the same name as the struct fields, 
//but having to repeat the email and username field names and variables is a bit tedious.
// If the struct had more fields, repeating each name would get even more annoying.

// this is where Using the Field Init Shorthand comes in
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user = build_user("user@example.com".to_string(), "username".to_string());

    println!("{:?}", user);
}



// Creating Instances from Other Instances with Struct Update Syntax

fn call() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("the values of user2: {:?}", user2);
}

//Using Tuple Structs Without Named Fields to Create Different Types

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(23, 11, 230);

    println!("the values of black: {:?}", black);
    println!("the values of origin: {:?}", origin);
}


// Unit-like structs

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
//Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 