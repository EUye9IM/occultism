use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wuxing {
    Jin,
    Mu,
    Shui,
    Huo,
    Tu,
}

impl fmt::Display for Wuxing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Wuxing::Jin => write!(f, "金"),
            Wuxing::Mu => write!(f, "木"),
            Wuxing::Shui => write!(f, "水"),
            Wuxing::Huo => write!(f, "火"),
            Wuxing::Tu => write!(f, "土"),
        }
    }
}

impl Wuxing {
    pub fn sheng(self, other: Wuxing) -> bool {
        matches!(
            (self, other),
            (Wuxing::Mu, Wuxing::Huo)
                | (Wuxing::Huo, Wuxing::Tu)
                | (Wuxing::Tu, Wuxing::Jin)
                | (Wuxing::Jin, Wuxing::Shui)
                | (Wuxing::Shui, Wuxing::Mu)
        )
    }

    pub fn ke(self, other: Wuxing) -> bool {
        matches!(
            (self, other),
            (Wuxing::Mu, Wuxing::Tu)
                | (Wuxing::Tu, Wuxing::Shui)
                | (Wuxing::Shui, Wuxing::Huo)
                | (Wuxing::Huo, Wuxing::Jin)
                | (Wuxing::Jin, Wuxing::Mu)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(Wuxing::Jin.to_string(), "金");
        assert_eq!(Wuxing::Mu.to_string(), "木");
        assert_eq!(Wuxing::Shui.to_string(), "水");
        assert_eq!(Wuxing::Huo.to_string(), "火");
        assert_eq!(Wuxing::Tu.to_string(), "土");
    }

    #[test]
    fn sheng_cycle() {
        assert!(Wuxing::Mu.sheng(Wuxing::Huo));
        assert!(Wuxing::Huo.sheng(Wuxing::Tu));
        assert!(Wuxing::Tu.sheng(Wuxing::Jin));
        assert!(Wuxing::Jin.sheng(Wuxing::Shui));
        assert!(Wuxing::Shui.sheng(Wuxing::Mu));
    }

    #[test]
    fn ke_cycle() {
        assert!(Wuxing::Mu.ke(Wuxing::Tu));
        assert!(Wuxing::Tu.ke(Wuxing::Shui));
        assert!(Wuxing::Shui.ke(Wuxing::Huo));
        assert!(Wuxing::Huo.ke(Wuxing::Jin));
        assert!(Wuxing::Jin.ke(Wuxing::Mu));
    }
}
