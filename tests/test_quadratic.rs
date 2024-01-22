#[cfg(test)]
mod tests {
    use quadratic::quadratic_solver::QuadraticSolver;

    #[test]
    fn test_quadratic() {
        let tests_cases = [
            (1.0, -2.0, 1.0, 1.0, 0.0, 0.0, 0.0),
            (1.0, 2.0, 1.0, -1.0, 0.0, 0.0, 0.0),
            (0.0, -2.0, 4.0, 2.0, 0.0, 0.0, 0.0),
            (1.0, 2.0, 3.0, -1.0, std::f32::consts::SQRT_2, -1.0, -std::f32::consts::SQRT_2),
            (1.0, 0.0, -4.0, 2.0, 0.0, -2.0, 0.0),
        ];
        for (a,b,c,x1r,x1i, x2r, x2i)  in tests_cases {
            run_test(a,b,c,x1r,x1i, x2r, x2i);
        }
    }

    fn run_test(
        a: f32,
        b: f32,
        c: f32,
        x1r: f32,
        x1i: f32,
        x2r: f32,
        x2i: f32,
    ) {
        let result = QuadraticSolver::solve(a, b, c);

        match result {
            Ok((x1, None)) => {
                assert_eq!(x1r, x1.real);
                assert_eq!(x1i, x1.imaginary);
            }
            Ok((x1, Some(x2))) => {
                assert_eq!(x1r, x1.real);
                assert_eq!(x1i, x1.imaginary);
                assert_eq!(x2r, x2.real);
                assert_eq!(x2i, x2.imaginary);
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}