//! Test fixture: Rust structs with impl blocks for symbol extraction testing.

use std::fmt;

/// A simple calculator.
pub struct Calculator {
    value: f64,
}

impl Calculator {
    /// Creates a new calculator with initial value.
    pub fn new(initial_value: f64) -> Self {
        Self { value: initial_value }
    }

    /// Adds a number to the current value.
    pub fn add(&mut self, x: f64) -> f64 {
        self.value += x;
        self.value
    }

    /// Subtracts a number from the current value.
    pub fn subtract(&mut self, x: f64) -> f64 {
        self.value -= x;
        self.value
    }

    /// Resets the calculator to zero.
    fn reset(&mut self) {
        self.value = 0.0;
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl fmt::Display for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Calculator({})", self.value)
    }
}

/// A 2D point.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Distance from origin.
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// A trait for shapes.
pub trait Shape {
    fn area(&self) -> f64;
}
