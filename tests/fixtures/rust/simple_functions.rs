//! Test fixture: Simple Rust functions for symbol extraction testing.

/// Greets a person by name.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Calculates rectangle area.
pub fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

/// Async function example.
pub async fn fetch_data(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!("Fetched from {}", url))
}

/// Generic function.
pub fn identity<T>(value: T) -> T {
    value
}

/// Function with lifetime.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
