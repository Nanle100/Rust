//The Slice Type
// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
// A slice is a kind of reference, so it does not have ownership.

// To do task
// Here’s a small programming problem: write a function that takes a 
// string of words separated by spaces and returns the first word it
// finds in that string. If the function doesn’t find a space in the string,
// the whole string must be one word, so the entire string should be returned.

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
   
//     //we create an iterator over the array of bytes using the iter method:
//     //know that iter is a method that returns each element in a collection and 
//     //that enumerate wraps the result of iter and returns each element as part of a tuple instead
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn main(){
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word will get the value 5

//     println!("Show me the value of word: {}", word);

//     s.clear(); // this empties the String, making it equal to ""

//     // word still has the value 5 here, but there's no more string that
//     // we could meaningfully use the value 5 with. word is now totally invalid!
// }


fn main (){
    let s = String::from("hello");

 let len = s.len();

let slice = &s[3..len];
 let slice = &s[3..];


println!("the value of slice: {}", slice);
}