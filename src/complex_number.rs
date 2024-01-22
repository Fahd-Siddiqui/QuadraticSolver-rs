use std::fmt;

pub struct ComplexNumber {
    pub real: f32,
    pub imaginary: f32,
}

impl ComplexNumber {
    pub fn get_conjugate(number: &ComplexNumber) -> ComplexNumber {
        return ComplexNumber { real: number.real, imaginary: -number.imaginary };
    }
}

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = format!("{}", self.real);

        if self.imaginary < 0.0 {
            string = format!("{} {}i", string, self.imaginary);
        } else if self.imaginary > 0.0 {
            string = format!("{} +{}i", string, self.imaginary);
        }

        write!(f, "{}", string)
    }
}