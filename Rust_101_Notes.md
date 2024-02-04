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
- True or False - bool
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
### Scope and Shadowing
### Suffixes and Underscores
## Challenge A
### Challenge A Overview
### Challenge A Solution
## Primitives (Compound Types) 
### Tuples
### Arrays
### Slices
