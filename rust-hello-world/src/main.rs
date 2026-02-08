use colored::*;
use std::io::{self, Write};

fn main() {
    println!("{}", "=== Welcome to Rust! ===".bright_cyan().bold());
    println!();
    
    // Get user's name
    print!("What's your name? ");
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    
    // Personalized greeting
    println!();
    println!("{} {}!", "Hello".green().bold(), name);
    println!();
    
    // Show some Rust features
    println!("{}", "🦀 Cool Rust Facts:".yellow());
    
    let facts = vec![
        "Rust is memory-safe without garbage collection",
        "Rust's mascot is Ferris the Crab 🦀",
        "Rust has been 'Most Loved Language' 8 years running",
    ];
    
    for (i, fact) in facts.iter().enumerate() {
        println!("  {}. {}", i + 1, fact);
    }
    
    println!();
    println!("{}", "Happy coding! 🚀".bright_magenta());
}
