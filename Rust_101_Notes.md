## Installing Rust
### Installation 
* For Linux: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* For Windows: [Exe file download](https://www.rust-lang.org/tools/install)
### Documentation 
* [Rust Book](https://doc.rust-lang.org/stable/book/)
* [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)
## Creating Our First Project
* Make the project: In a Terminal in VSCode: `cargo new project_name_here`
* Enter into the project: Go to `File` and then find your project's folder and open it
* Run the project: `cargo run` / just the run button
## Hello World
### Main Function & Macros
* Main function must be in every Rust program, it is where your code starts pretty much
* The `main` function has the following signature:
```rust
fn main() {
  // code goes here
}
```
* Takes no arguments and returns no value
* Macros = built in functions like `println`
  * `println` automatically add /n to your string, but just `print` does not
  * Hello world main function example:
  ```rust
  fn main() {
    println!("Hello, world!");
  }
  ```
### Comments
* 3 types of comments: line, block and documentation
  * Line comments are single line
    ```rust
    // This is a line comment

    let x = 42;  // This line is commented out
    ```
  * Block commnets can be multiple lines
    ```rust
    /*
    This is a block comment
    It can span multiple lines
    */

    let x = 42;

    /*
    let y = 13;
    This block is commented out
    */
    ```
  * Documentation comments can be use with `rustdoc` to generate documentation for Rust code
    ```rust
    /// This function adds two numbers
    ///
    /// # Examples
    ///
    ///
    /// let x = 1;
    /// let y = 2;
    /// assert_eq!(add(x, y), 3);
    ///

    fn add(x: i32, y: i32) -> i32 {
    x + y
    }
    ```
### Errors
* Can see our errors in the Terminal and also in the VSCode IDE before even running it (for certain errors)
    * Get error code and line number
* Can run `rustc` commands to explain the errors to you
    * Example: `rustc —explain E0762`
## Primitives (Scalar Types)
### Integers
- Integers = whole numbers
    - Usage: counting, indexing, and arithmetic operations, etc
- Common types
    - Signed Integers, can be negative or positive (if you don’t declare this is the default: `i32`)
        - The numbers in their names represents the number of bits they use to store their values.
        - `i8`, `i16`, `i32`, and `i64`
    - Unsigned Integers, can only be positive
        - The number in their names represents the number of bits they use to store their values (again)
        - `u8`, `u16`, `u32`, and `u64`
    - `isize` & `usize`: signed and unsigned integers that can represent the size of a pointer
    on the current platform
        - Aka 32 bits long on 32-bit platforms, and 64 bits long on 64 bit platforms
- Integers can be created using literals, such as `42` or `0xff`, or by using expressions and variables.
- Math can be performed with these integers of course
- Bitwise operations can also performed like bitwise and (`&`), bitwise or (`|`), and bitwise not (`!`).
- Rust can check for overflow at runtime and you can also handle overflows explicitly like by using the `Wrapping` struct arithmetic operations with wrapping behavior instead of panicking
- Literal int - decimals, binary, hex, etc
```rust
println!("Max size of a u32: {}", u32::MAX); // Print out the max size of a u32 int | 4294967295
println!("Max size of a u64: {}", u64::MAX); // Print out the max size of a u64 int | 18446744073709551615

println!("Max size of a i32: {}", i32::MAX); // Print out the max size of a i32 int | 2147483647
println!("Max size of a i64: {}", i64::MAX); // Print out the max size of a i64 int | 9223372036854775807
```
### Floats
- Decimal values: 1000.1, 3.14, etc (.14 is not valid, you need to do 0.14)
- f32 and f64 are the only types (for bit architecture)
  - f64 is the default (can be slower on less than 64 bit architecture)
```rust
println!("Max size of a f32: {}", f32::MAX); // Print out the max size of a f32 int | 340282350000000000000000000000000000000
println!("Max size of a f64: {}", f64::MAX); // Print out the max size of a f64 int | 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```
### Boolean
- True or False - `bool`
- For conditional statements
  - Can use `and`, `not`, and `or` with these of course
```rust
let x = 15; 
if x > 10 { 
  println!("x is greater than 10");
} 
```
### Characters
- Syntax is `char`
- Is 4 bytes
```rust
println!('A') // can be letters like this, but also could be emojis, unicode, and lots of other things (anything 4 bytes in size)
// These are NOT strings
```
## Variables
### Types of Variables
- Three keywords to declare variables
    - `let`, `let mut`, and `const`
```rust
let x = 42; // immutable variable
let mut x = 42; x = 43; // mutable variable
const PI: f32 = 3.14159; // constant variable, not mutable and can never be mutable | have to be called in screamcase (all caps)
const NUMBER: i32 = 17; // Have to declare the anotation type (i32 in this case) instead of there just being a default / autopopulate option
// const can be placed outside of a function, they are global variables, and they are very fast
```
### Scope and Shadowing
- Scope = where you can access a variable
- Variables declared inside a block / function have limited scope (only within that given block of function)
- Variables declared outside of a block / function have a global scope
```rust
let x = 42; // Global scope
{
  let y = 43; // Limited scope
  println!("x: {}, y: {}", x, y); 
}
println!("x: {}", x);
```
- Shadowing allows you to declare a new variable with the same name as an existing variable in the same scope
  - Good for changing the value of a varaible without having to change it's name
```rust
let x = 42; // First value
let x = "hello"; // Shadowed value 
println!("x: {}", x); // prints "hello"
```
- Can use this to convert a mutable variable to an inmutable one
```rust
let mut x = 42;
x = 43;
let x = x;
println!("x: {}", x);
```
- Can remove certain errors/warnings from your terminal output if you want
```rust
#![allow(unused)] // allow unused variables
```
### Suffixes and Underscores
- Suffixes are used to specify the type of a numeric literal
  - Like `u32`, or `i32`
```rust
let x: u32 = 42u32;
let x: i32 = -42i32; 
```
- The u and i represent unsigned and signed numbers respectively
  - The number is the number of bits in its integer
- Underscores in Rust can be used to separate large numeric literals for better readability
  - These underscores have no effect on values
```rust
fn main() { 
  let x = 1_000_000;
  let y = 0.000_000_001;
  println!("x = {}, y = {}", x, y); // output is x = 1000000, y = 0.000000001
}

let x: i32 = 42_i32; // output is 42
```
## Challenge A
### Challenge A Overview
```rust
fn main() {
    /* Challenge 1 - Build a program that has the following:

    1) Has a global constant integer named 'birthday' with a value of 1
    2) Has a local string variable named 'my_name' with your name as the value
    3) Has a local string variable named 'my_birthday' with your birth month/day (no year) as the value
    4) Has a local mutable integer variable named 'age' with your current age as the value
    5) Has a local integer variable named 'new_age' with your age after your birthday as the value
    6) Prints out 'My name is X and I am X years old. I will turn X on X' 

    */
}
```
### Challenge A Solution
```rust
#![allow(unused)]

const BIRTHDAY: i32 = 1; 

fn main() {
    let my_name = "Hiro";
    let my_birthday = "June 31st";
    let mut age = 99;
    let new_age = age+BIRTHDAY;

    println!("My name is {} and I am {} years old. I will turn {} on {}.", my_name, age, new_age, my_birthday);
}
```
## Primitives (Compound Types) 
### Tuples
- Fixed size collection of elements of different types
  - Used to group related data
  - Declared with parentheses ()
  - Max values that can be stored is 12
```rust
let person = ("John", 42, 1.72); // Basic Tuple
let (name, age, height) = person; // Can be used for pattern matching
println!("Name: {}, Age: {}, Height: {}", name, age, height);

let person = ("John", 42, 1.72);
println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2); // Can also be indexed with dot notation

// Can also assign items within the tuples to variables in a more basic way like this
let person = ("John", 42, 1.72);
let name = person.0;
let age = person.1;
let height = person.2;
```
### Arrays
- Fixed-size collection of elements of the same type
  - Declared with brackets []
  - The length of the arry is part of it's type and therefore it must be known at compile time
  - Can store up to 32 values
  - Faster than Tuples
```rust
let numbers = [1, 2, 3, 4, 5]; // An array with 5 elements
println!("The first number is {}", numbers[0]);  // Can be access with indexing notation

let numbers = [1, 2, 3, 4, 5];
for number in numbers.iter() { println!("{}", number); // Can be iterated over in a for loop
``` 
#### Slices
- Lightweight view into a contiguous sequence of elements in a collection (like array or vectors)
  - Can be created with an existing collection by specifying a range of indices
```rust
let arr = [1, 2, 3, 4, 5]; // Make the array
let slice = &arr[1..3]; // Make the slice (ending at one before the number you enter after the ..)
println!("{:?}", slice); // prints "[2, 3]" 

// Slices can be used to pass a subset of elements in a collection to a function
fn sum(slice: &[i32]) -> i32 {
  let mut total = 0;
  for &x in slice {
    total += x;
  }
  total
}

let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3]; // 2 & 3 
let total = sum(slice);
println!("The sum is {}", total); // prints "The sum is 5"

// Slices can be used to pass a subset of elements in a collection to a function
let mut arr = [1, 2, 3, 4, 5]; // make a mutable array 
let slice = &mut arr[1..3]; // call the mutable array
slice[0] = 6; // Change the values of the slices
slice[1] = 7; // Change the values of the slices
println!("{:?}", arr); // prints "[1, 6, 7, 4, 5]"
```
## Strings
### Strings Overview
- Three main types: `str`, `&str`, and `String`
  - `str` is a primitive type that represents a sequence of Unicode scalar values
    - Often used as a slice type to represent borrowed string data
    - Not really commonly used in programs though
  - `&str` is a string slice type that represents a borrowed view into a sequence of Unicode scalar values
    - Read-only type | no modification
    - Often used to pass string data between functions or to extract substrings from a larger string
  - `String` is a dynamically-sized string type that represents a sequence of Unicode scalar values
    - Mutable
    - Often used to represent user input, file data, or other dynamic text content
```rust
// &str type
fn main() {
  let message = "hello, world!";
  let first_word = &message[0..5];
  println!("The first word is {}", first_word);
}

// String type
fn main() {
  let mut message = String::from("hello");
  message.push_str(", world!");
  println!("{}", message);
}
```
### Escaping
- Used to represent special characters in string literals and character literals
- Common escaped characters
  - `\\` backslash
  - `\"` double quote
  - `\'` single quote
  - `\n` newline
  - `\r` carriage return
  - `\t` tab
  - `\0` null character
  - `\xNN` hexadecimal escape sequence, where NN is a two-digit hexadecimal number that represents a Unicode scalar value
  - `\u{NNNN}` Unicode escape sequence, where NNNN is a four-digit hexadecimal number that represents a Unicode scalar value
```rust
let message = "Hello, \"world\"!\n"; // Escaped the double quotes and newline
println!("{}", message);

let heart = '\u{2764}'; // Escaped the Unicode scalar value for a heart symbol
println!("{}", heart);

// Raw string literals can be used to include backslashes and special characters without escaping them
// Declared using the r#"..."# syntax
let message = r#"Hello, "world"!\n"#;
println!("{}", message); 
```
## User Input
### Modules and Libraries
- Need to use the standard I/O library
  - The [documentation](https://doc.rust-lang.org/std/io/index.html) for this is good to look at
```rust
use std::io; // import library
```
### User Input
- User input allows us to take input from the user and then use it in our program
- To read user input from the command line, the `std::env` module can be used to access the command line arguments passed to the program
```rust
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect(); // get user input
  println!("The first argument is {}", args[1]); // print to screen
} 
```
- To read user input from the standard input stream, the `std::io::stdin()` function can be used to obtain a handle to the input stream
  - `std::io::BufRead` trait provides functions for reading lines of input from this stream
```rust
use std::io::{self, BufRead}; 

fn main() {
  let stdin = io::stdin(); // obtain a handle to the standard input stream
  for line in stdin.lock().lines() { //  obtain a locked reference to the input stream
    println!("{}", line.unwrap()); // print it out
  }
} 
```
## Math
### Math Operators
- `+`: addition
- `-`: subtraction
- `*`: multiplication
- `/`: division
- `%`: remainder
```rust
let x = 10;
let y = 3;
let sum = x + y;
let diff = x - y;
let product = x * y;
let quotient = x / y;
let remainder = x % y;
```
## Dependencies 
- Stored in the Cargo.toml file
```rust
[dependencies]
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
```
- Requires  package name and version number
  - "0.8": exact version number
  - ">=0.8,<1.0": version range
  - "*": wildcard
- Can go to crates.io to see more package information / find packages
```rust
// Rand package usage example
use rand::Rng;

fn main() {
  let mut rng = rand::thread_rng();
  let random_number = rng.gen::<u32>();
  println!("Random number: {}", random_number);
}
```
## Challenge B
### Challenge B Overview
```rust
fn main() {
    /* Build a simple calculator that takes two user inputs
       then calculates the addition, subtraction, multiplication, and division
       of those two inputs.
    */
}
```
### Challenge B Solution
```rust
#![allow(unused)]

use std::io;

fn main() {
    println!("Give me a value for x");
    let mut input_x = String::new();
    io::stdin().read_line(&mut input_x);

    let x: i32 = input_x.trim().parse().expect("Entry was not an integer!");
    let float_x = x as f64;

    println!("Give me a value for y");
    let mut input_y = String::new();
    io::stdin().read_line(&mut input_y);

    let y: i32 = input_y.trim().parse().expect("Entry was not an integer!");
    let float_y = y as f64;

    println!("The result of {} + {} is {}", x, y, x+y);
    println!("The result of {} - {} is {}", x, y, x-y);
    println!("The result of {} * {} is {}", x, y, x*y);
    println!("The result of {} / {} is {}", x, y, float_x/float_y);
}
```
## Control Flow
### Comparison Operators and Truth Tables
- Truth tables and operators using boolean values (true or false)
- Operators
  - `==`: equality
  - `!=`: inequality
  - `<`: less than
  - `<=`: less than or equal to
  - `>`: greater than
  - `>=`: greater than or equal to
```rust
fn main() {
    let a = 5;
    let b = 10;
    let c = true;
    let d = false;

    println!("a > b: {}", a > b); // false
    println!("a >= b: {}", a >= b); // false
    println!("a < b: {}", a < b); // true
    println!("a <= b: {}", a <= b); // true
    println!("a == b: {}", a == b); // false
    println!("a != b: {}", a != b); // true
    println!("True or False: {}", c || d); //true
    println!("True or True: {}", c || c); //true
    println!("False or False: {}", d || d); //false
    println!("True and False: {}", c && d); //false
    println!("True and True: {}", c && c); //true
    println!("False and False: {}", d && d); //false
}
```
- Not the whole truth table (can google that), these are just the really important ones
- Logical operations
  - `&&` logical AND
  - `||` logical OR
  - `!` logical NOT
### Conditional Statements
- Used to control the flow of execution based on Boolean conditions
- `if` statements
  - Can also have one or more `else if` statement
  - And an optional `else` statement for everything else
```rust
let x = 10;
if x > 0 {
  println!("x is positive");
} else if x < 0 {
  println!("x is negative");
} else {
  println!("x is zero");
}
```
- Larger code block example
```rust
#![allow(unused)]

use std::io;
use rand::Rng;

fn main() {
    //if, else if, else

    println!("How much money do you have?");
    let mut input_money = String::new();
    io::stdin().read_line(&mut input_money);

    let money: i32 = input_money.trim().parse().expect("Entry was not an integer");

    println!("How old are you?");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age);

    let age: i32 = input_age.trim().parse().expect("Entry was not an integer");

    if (age >= 21) && (money >= 5) {
        println!("We're getting a drink!")
    } else if (age >=21) && (money < 5) {
        println!("Come back with more money!")
    } else if (age < 21) && (money >= 5) {
        println!("Nice try, kid!")
    } else {
        println!("You're too young and too poor.")
    };

}
```
### Match
- The second type of conditional statements (`if` statements are the first type)
- Used to match a value against a set of patterns and execute the corresponding code for the first matching pattern
  - Can have one or more `arm`s, which consist of a pattern and the corresponding code to execute
  - All possible values must be covered 
```rust
let candidacy_age: i32 = 24;

match candidacy_age {
  1..=24 => println!("Cannot hold office."),
  25..=29 => println!("Can run for the House"),
  30..=34 => println!("Can run for the Senate"),
  35..=i32::MAX => println!("Can run for President"),
  _ => println!("Are you an infant?") // Catch all for everything else 
};
```
```rust
// Can also use Ordering 
let my_age: i32 = 33;
let drinking_age: i32 = 21;
match my_age.cmp(&drinking_age){
  Ordering::Less => println!("Cannot drink"),
  Ordering::Equal => println!("Woo, you can drink!"),
  Ordering::Greater => println("Can drink!"),
};
```
### Loops
- `Loop` statements are used to repeat a block of code indefinitely until a `break` statement is encountered
```rust
// Counts up and prints each number out until ten, at which point the break statement is hit
let mut count = 0;
loop {
  count += 1;
  println!("Count: {}", count);
  if count == 10 {
    break;
  }
}
```
- `While` statements are used to repeat an action while a certain condition is true
```rust
// loop terminates when the counter reaches a value of 10 and the condition becomes false
let mut count = 0;
while count < 10 {
  count += 1;
  println!("Count: {}", count);
}
```
- `For` statement is used to iterate over a range, an iterator, or a collection of elements
```rust
// iterate over a range of values from 1 to 10, and print each value
for i in 1..=10 {
  println!("Count: {}", i);
}

let numbers = vec![1, 2, 3, 4, 5];

// iterate over a vector of numbers and print each number
for num in numbers {
  println!("Number: {}", num);
} 
```
## Functions
- Used to encapsulate a block of code that performs a specific task and can be called from other parts of the program
  - `fn` keyword
- Call them later on, reuse them multiple times
- Can be anywhere outside of `main` (above, below, doesn't matter)
```rust
fn greet(name: &str) { // function named greet, one parameter called name, which is of the &str type
  println!("Hello, {}!", name);
} 
```
- Can also use the `->` syntax
```rust
fn add(x: i32, y: i32) -> i32 { // function named add, two paramters of type i32
  x + y // add them together
} 
```
- Functions can also have default parameter values, which are used when no value is provided
```rust
fn repeat(word: &str, count: u32) -> String {
  word.repeat(count as usize)
} 

fn main() { 
  println!("{}", repeat("hello", 3)); // prints "hellohellohello"
  println!("{}", repeat("world", 5)); // prints "worldworldworldworldworld"
  println!("{}", repeat("bye", 1)); // prints "bye"
} 
```
- Functions can also use the keyword `ruturn` to explicitly return a value from the function
```rust
fn divide(x: f32, y: f32) -> Result<f32, String> { 
  if y == 0.0 { 
    return Err(String::from("Division by zero")); // return error output
  }
  Ok(x / y)
}

fn main() { 
  match divide(10.0, 2.0) { 
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err), 
  }
  match divide(10.0, 0.0) { 
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err), 
  }
} 
```
## Challenge C
### Challenge C Overview
```rust
fn main() {
    // Create a calculator that takes three user inputs (x, y, and operator)
    // Create functions for +, -, *, /
    // Use if/else or Match for operator
    // Might take a little research!
}
```
### Challenge C Solution
```rust
use std::io;

fn main() {
    //Opening lines
    println!("Hiro's Rust Calculator");
    println!("You must select two values (x and y) and an operator.");

    //Receive a value for X
    println!("Please give me a value for X.");

    let mut x = String::new();
    io::stdin().read_line(&mut x);
    let x: i32 = x.trim().parse().expect("Entry was not an integer.");
    let float_x = x as f64;

    //Receive a value for Y
    println!("Please give me a value for Y.");

    let mut y = String::new();
    io::stdin().read_line(&mut y);
    let y: i32 = y.trim().parse().expect("Entry was not an integer.");
    let float_y = y as f64;

    //Receive an operator
    println!("Choose an operator: +, -, *, /");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator);
    let operator_slice = operator.trim();

    //Match operator
    match operator_slice {
        "+" => {
            add(x,y);
        }
        "-" => {
            subtract(x,y);
        }
        "*" => {
            multiply(x,y);
        }
        "/" => {
            divide(float_x,float_y);
        }
        &_ => {
            println!("Invalid entry. Exiting program.");
        }
    }


}

//Math functions
fn add(x: i32, y:i32) {
    println!("The result of {} + {} = {}",x,y,x+y);
}
fn subtract(x: i32, y:i32) {
    println!("The result of {} - {} = {}",x,y,x-y);
}
fn multiply(x: i32, y:i32) {
    println!("The result of {} * {} = {}",x,y,x*y);
}
fn divide(x: f64, y:f64) {
    println!("The result of {} / {} = {}",x,y,x/y);
}
```
## Other Items
### Vectors
### Structures
### Enums
### Generics
### Traits
## Memory Management
### Ownership
### References & Borrowing
## File Input & Output
## Error Handling
## Tying It All Together
### SHA256 Password Cracker
https://github.com/HiroNewf/TCM-Security-Courses-Code-and-Notes/blob/main/SHA256_Password_Cracker.rs
```rust
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid arguments!");
        println!(">> {} <sha256sum>", args[0]);
        exit(1);
    }

    let wanted_hash = &args[1];
    let password_file = "src/rockyou.txt";
    let mut attempts = 1;

    println!("Attempting to back: {}!\n", wanted_hash);

    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);

    for line in reader.lines() {
        let line = line.unwrap();
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}", Sha256::digest(&password));
        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
        if &password_hash == wanted_hash {
            println!("Password hash found after {} attempts! {} hashes to {}!", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
        attempts += 1;
    }

    println!("Password hash not found!");
}
```
### URL Chortener
https://github.com/HiroNewf/TCM-Security-Courses-Code-and-Notes/blob/main/URL_Shortener.rs
```rust
#![allow(unused)]

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let mapping_path = "src/mapping.txt";
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: ./url_shortener <url>");
        return;
    }

    let url = &args[1];

    if url.starts_with("http") {
        // Convert long-form URL to short-form URL.
        let mut rng = rand::thread_rng();
        let short_url: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric) as char)
            .take(8)
            .collect();

        println!("Long URL: {}", url);
        println!("Short URL: {}", short_url);

        // Store the mapping between the short URL and the long URL in a file.
        let mut mapping_file = match OpenOptions::new().write(true).append(true).open(mapping_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Error opening mapping file");
                return;
            }
        };
        let mapping = format!("{},{}\n", short_url, url);
        if let Err(_) = mapping_file.write_all(mapping.as_bytes()) {
            println!("Error writing to mapping file");
            return;
        }
    } else {
        // Read the mapping file and redirect the user to the corresponding long URL.
        let mapping_file = match File::open(mapping_path) {
            Ok(file) => file,
            Err(_) => {
                println!("Error opening mapping file");
                return;
            }
        };
        let reader = BufReader::new(mapping_file);
        for line in reader.lines() {
            let mapping = match line {
                Ok(line) => line,
                Err(_) => {
                    println!("Error reading mapping file");
                    continue;
                }
            };
            let parts: Vec<&str> = mapping.split(',').collect();
            if parts.len() != 2 {
                continue;
            }
            let short = parts[0];
            let long = parts[1];
            if short == url {
                println!("Redirecting to {}", long);
                return;
            }
        }
        println!("Short URL not found");
    }
}
```
### Quiz Game
https://github.com/HiroNewf/TCM-Security-Courses-Code-and-Notes/blob/main/Quiz_Game.rs
```rust
#![allow(unused)]

use std::io::{self, Write};

fn main() {
    let mut correct_answers = 0;

    //Welcome message
    println!("Welcome to our Quiz Game!");
    println!("Please select the correct answer for each question.");

    //Question 1
    println!("1. What is the capital city of France?");
    println!("A. London");
    println!("B. Paris");
    println!("C: Rome");
    print!("Your answer: ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();

    if answer.trim().to_ascii_uppercase() == "B" {
        println!("Correct!");
        correct_answers += 1;
    } else {
        println!("Incorrect.  The correct answer is B.");
    }

    //Question 2
    println!("\n2. What is the largest country in the world by area?");
    println!("A. Russia");
    println!("B. Canada");
    println!("C: China");
    print!("Your answer: ");
    io::stdout().flush().unwrap();

    answer.clear();
    io::stdin().read_line(&mut answer).unwrap();

    if answer.trim().to_ascii_uppercase() == "A" {
        println!("Correct!");
        correct_answers += 1;
    } else {
        println!("Incorrect.  The correct answer is A.");
    }

    // Question 3
    println!("\n3. Who is credited with inventing the World Wide Web?");
    println!("A. Bill Gates");
    println!("B. Tim Berners-Lee");
    println!("C. Steve Jobs");
    print!("Your answer: ");
    io::stdout().flush().unwrap();

    answer.clear();
    io::stdin().read_line(&mut answer).unwrap();

    if answer.trim().to_ascii_uppercase() == "B" {
        println!("Correct!");
        correct_answers += 1;
    } else {
        println!("Incorrect. The correct answer is B.");
    }

    //Calculate final score
    let total_questions = 3;
    let percentage = (correct_answers as f32 / total_questions as f32) * 100.0;
    
    println!("\nYou got {} out of {} questions correct ({:.2})%", correct_answers, total_questions, percentage);

}
```
