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
### Floats
### Boolean
### Characters
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
