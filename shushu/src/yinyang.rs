use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yinyang {
    Yin,
    Yang,
}

impl fmt::Display for Yinyang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Yinyang::Yin => write!(f, "阴"),
            Yinyang::Yang => write!(f, "阳"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Yinyang::Yin.to_string(), "阴");
        assert_eq!(Yinyang::Yang.to_string(), "阳");
    }
}
