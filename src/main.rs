use std::io;
use quadratic::complex_number::ComplexNumber;
use quadratic::quadratic_solver::QuadraticSolver;


fn main() {
    // Create variables to store the user inputs
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    // Create a new instance of the standard input stream
    let stdin = io::stdin();

    println!("This is the quadratic solver for the equation:");
    println!("  ax^2 + bx + c = 0");
    println!("Enter the value for 'a': ");
    stdin.read_line(&mut a).expect("Failed to read input");

    println!("Enter the value for 'b': ");
    stdin.read_line(&mut b).expect("Failed to read input");

    println!("Enter the value for 'c': ");
    stdin.read_line(&mut c).expect("Failed to read input");

    let a: f32 = a.trim().parse().expect("Failed to parse 'a'");
    let b: f32 = b.trim().parse().expect("Failed to parse 'b'");
    let c: f32 = c.trim().parse().expect("Failed to parse 'c'");

    println!("You entered {}x^2 + {}x + {} = 0", a, b, c);
    let result: Result<(ComplexNumber, Option<ComplexNumber>), String> = QuadraticSolver::solve(a, b, c);

    match result {
        Ok((x1, None)) => {
            println!("Root: {}", x1);
        }
        Ok((x1, Some(x2))) => {
            println!("Root1: {}", x1);
            println!("Root2: {}", x2);
        }
        Err(error) => println!("Error: {}", error),
    }

}

