use crate::complex_number::{ComplexNumber};

pub struct QuadraticSolver{}

impl QuadraticSolver {
    pub fn solve(a: f32, b: f32, c: f32) -> Result<(ComplexNumber, Option<ComplexNumber>), String> {
        if a == 0.0 && b == 0.0 {
            return Err(String::from("Cannot resolve!"));
        }

        if a == 0.0 {
            let x1 = ComplexNumber { real: -c / b, imaginary: 0.0 };
            return Ok((x1, None));
        }

        let discriminant: f32 = b * b - 4.0 * a * c;

        let real = -b / (2.0 * a);

        if discriminant == 0.0 {
            let x1 = ComplexNumber { real, imaginary: 0.0 };
            return Ok((x1, None));
        }

        if discriminant < 0.0 {
            let imaginary = (-discriminant).sqrt() / (2.0 * a);
            let x1 = ComplexNumber { real, imaginary };
            let conjugate = ComplexNumber::get_conjugate(&x1);
            return Ok((x1, Some(conjugate)));
        }

        let x1 = ComplexNumber { real: (-b + discriminant.sqrt()) / (2.0 * a), imaginary: 0.0 };
        let x2 = ComplexNumber { real: (-b - discriminant.sqrt()) / (2.0 * a), imaginary: 0.0 };

        return Ok((x1, Some(x2)));
    }
}