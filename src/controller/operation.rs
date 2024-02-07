use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operation {
    pub fn operate(&self, first: i64, second: i64) -> i64 {
        match self {
            Self::Plus => first + second,
            Self::Minus => first - second,
            Self::Divide => first / second,
            Self::Multiply => first * second,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
        }
    }
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Multiply => write!(f, "Multiply"),
            Self::Divide => write!(f, "Divide"),
        }
    }
}