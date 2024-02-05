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
### Arrays
#### Slices
## Strings
### Strings Overview
### Escaping
## User Input
### Modules and Libraries
### User Input
## Math
### Math Operators
## Dependencies 
## Challenge B
### Challenge B Overview
### Challenge B Solution
## Control Flow
### Comparison Operators and Truth Tables
### Conditional Statements
### Match
### Loops
## Functions
## Challenge C
### Challenge C Overview
### Challenge C Solution
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
