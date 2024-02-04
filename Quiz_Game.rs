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
