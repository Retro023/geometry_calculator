use std::io;

#[derive(Debug)]
enum Shape {
    Square,
    Rectangle,
    Triangle,
    Circle,
}

impl Shape {
    fn from_u32(n: u32) -> Option<Shape> {
        match n {
            1 => Some(Shape::Square),
            2 => Some(Shape::Rectangle),
            3 => Some(Shape::Triangle),
            4 => Some(Shape::Circle),
            _ => None,
        }
    }
}

fn main() {
    // Select the shape
    println!("Choose the shape you wish to calculate the area for:");
    println!("1. Square");
    println!("2. Rectangle");
    println!("3. Triangle");
    println!("4. Circle");

    let shape = get_user_input()
        .trim()
        .parse::<u32>()
        .ok()
        .and_then(Shape::from_u32);

    match shape {
        Some(Shape::Square) => handle_square(),
        Some(Shape::Rectangle) => handle_rectangle(),
        Some(Shape::Triangle) => handle_triangle(),
        Some(Shape::Circle) => handle_circle(),
        None => println!("Invalid choice! Please select 1, 2, 3, or 4."),
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn get_user_input_as_f64(prompt: &str) -> Result<f64, String> {
    println!("{}", prompt);
    let input = get_user_input();
    input
        .trim()
        .parse::<f64>()
        .map_err(|_| "Invalid number".to_string())
}

fn handle_square() {
    match get_user_input_as_f64("Please enter the length of the square:") {
        Ok(side) => {
            let area = side * side;
            println!("The dimensions of the square:");
            println!("|------------------------------------------------------------|");
            println!(" The side length of the square: {:.2}", side);
            println!(" The area of the square is: {:.2} square units", area);
            println!("
                                                                 ########
                                                                 #      #
                                                                 #      #
                                                                 ########
            ");
            println!("|--------------------------------------------------------------|");
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn handle_rectangle() {
    let width = get_user_input_as_f64("Please enter the width of the rectangle:");
    let length = get_user_input_as_f64("Please enter the length of the rectangle:");

    match (width, length) {
        (Ok(width), Ok(length)) => {
            let area = width * length;
            println!("The dimensions of the rectangle:");
            println!("|---------------------------------------------------------|");
            println!(" Width: {:.2}", width);
            println!(" Length: {:.2}", length);
            println!(" The area of the rectangle is: {:.2} square units", area);
            println!("
                                                             ########
                                                             #      #
                                                             #      #
                                                             #      #
                                                             ########
            ");
            println!("|----------------------------------------------------------|");
        }
        _ => println!("Error: Invalid input"),
    }
}

fn handle_triangle() {
    let base = get_user_input_as_f64("Please enter the base length of the triangle:");
    let height = get_user_input_as_f64("Please enter the height of the triangle:");

    match (base, height) {
        (Ok(base), Ok(height)) => {
            let area = 0.5 * base * height;
            println!("The dimensions of the triangle:");
            println!("|-----------------------------------------------------------------|");
            println!(" Base: {:.2}", base);
            println!(" Height: {:.2}", height);
            println!(" The area of the triangle is: {:.2} square units", area);
            println!("
                                                        /\\
                                                       /  \\
                                                      /    \\
                                                     /______\\
            ");
            println!("|-----------------------------------------------------------------|");
        }
        _ => println!("Error: Invalid input"),
    }
}

fn handle_circle() {
    match get_user_input_as_f64("Please enter the radius of the circle:") {
        Ok(radius) => {
            let area = std::f64::consts::PI * radius * radius;
            println!("The dimensions of the circle:");
            println!("|--------------------------------------------------------------|");
            println!(" Radius: {:.2}", radius);
            println!(" The area of the circle is: {:.2} square units", area);
            println!("
                                                               ____  
                                                             /      \\ 
                                                            |        |
                                                             \\______/ 
            ");
            println!("|--------------------------------------------------------------|");
        }
        Err(e) => println!("Error: {}", e),
    }
}
