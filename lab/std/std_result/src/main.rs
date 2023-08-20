mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

fn op(x: f64, y: f64) -> f64 {
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn op_(x: f64, y: f64) -> checked::MathResult {
    let ratio = checked::div(x, y)?;
    let ln = checked::ln(ratio)?;
    checked::sqrt(ln)
}

fn main() {
    //println!("{}", op(1.0, 10.0));

    //
    match op_(1.0, 10.0) {
        Err(why) => panic!("{}", match why {
            checked::MathError::NegativeLogarithm => "logarithm of negative number",
            checked::MathError::DivisionByZero => "division by zero",
            checked::MathError::NegativeSquareRoot => "square root of negative number",
        }),
        Ok(value) => println!("{}", value),
    }
}
