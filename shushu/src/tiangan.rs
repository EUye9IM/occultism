use std::fmt;

use crate::wuxing::Wuxing;
use crate::yinyang::Yinyang;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tiangan {
    Jia,
    Yi,
    Bing,
    Ding,
    Wu,
    Ji,
    Geng,
    Xin,
    Ren,
    Gui,
}

impl fmt::Display for Tiangan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tiangan::Jia => write!(f, "甲"),
            Tiangan::Yi => write!(f, "乙"),
            Tiangan::Bing => write!(f, "丙"),
            Tiangan::Ding => write!(f, "丁"),
            Tiangan::Wu => write!(f, "戊"),
            Tiangan::Ji => write!(f, "己"),
            Tiangan::Geng => write!(f, "庚"),
            Tiangan::Xin => write!(f, "辛"),
            Tiangan::Ren => write!(f, "壬"),
            Tiangan::Gui => write!(f, "癸"),
        }
    }
}

impl Tiangan {
    pub fn ordinal(self) -> usize {
        self as usize + 1
    }

    pub fn wuxing(self) -> Wuxing {
        match self {
            Tiangan::Jia | Tiangan::Yi => Wuxing::Mu,
            Tiangan::Bing | Tiangan::Ding => Wuxing::Huo,
            Tiangan::Wu | Tiangan::Ji => Wuxing::Tu,
            Tiangan::Geng | Tiangan::Xin => Wuxing::Jin,
            Tiangan::Ren | Tiangan::Gui => Wuxing::Shui,
        }
    }

    pub fn yinyang(self) -> Yinyang {
        match self {
            Tiangan::Jia | Tiangan::Bing | Tiangan::Wu | Tiangan::Geng | Tiangan::Ren => {
                Yinyang::Yang
            }
            Tiangan::Yi | Tiangan::Ding | Tiangan::Ji | Tiangan::Xin | Tiangan::Gui => {
                Yinyang::Yin
            }
        }
    }

    pub fn iter() -> impl Iterator<Item = Tiangan> {
        [
            Tiangan::Jia,
            Tiangan::Yi,
            Tiangan::Bing,
            Tiangan::Ding,
            Tiangan::Wu,
            Tiangan::Ji,
            Tiangan::Geng,
            Tiangan::Xin,
            Tiangan::Ren,
            Tiangan::Gui,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        assert_eq!(Tiangan::iter().count(), 10);
    }

    #[test]
    fn display() {
        let stems: Vec<String> = Tiangan::iter().map(|s| s.to_string()).collect();
        assert_eq!(stems.join(""), "甲乙丙丁戊己庚辛壬癸");
    }

    #[test]
    fn ordinal() {
        assert_eq!(Tiangan::Jia.ordinal(), 1);
        assert_eq!(Tiangan::Gui.ordinal(), 10);
    }

    #[test]
    fn wuxing() {
        assert_eq!(Tiangan::Jia.wuxing(), Wuxing::Mu);
        assert_eq!(Tiangan::Yi.wuxing(), Wuxing::Mu);
        assert_eq!(Tiangan::Bing.wuxing(), Wuxing::Huo);
        assert_eq!(Tiangan::Ding.wuxing(), Wuxing::Huo);
        assert_eq!(Tiangan::Wu.wuxing(), Wuxing::Tu);
        assert_eq!(Tiangan::Ji.wuxing(), Wuxing::Tu);
        assert_eq!(Tiangan::Geng.wuxing(), Wuxing::Jin);
        assert_eq!(Tiangan::Xin.wuxing(), Wuxing::Jin);
        assert_eq!(Tiangan::Ren.wuxing(), Wuxing::Shui);
        assert_eq!(Tiangan::Gui.wuxing(), Wuxing::Shui);
    }

    #[test]
    fn yinyang() {
        assert_eq!(Tiangan::Jia.yinyang(), Yinyang::Yang);
        assert_eq!(Tiangan::Yi.yinyang(), Yinyang::Yin);
        assert_eq!(Tiangan::Bing.yinyang(), Yinyang::Yang);
        assert_eq!(Tiangan::Ding.yinyang(), Yinyang::Yin);
        assert_eq!(Tiangan::Wu.yinyang(), Yinyang::Yang);
        assert_eq!(Tiangan::Ji.yinyang(), Yinyang::Yin);
        assert_eq!(Tiangan::Geng.yinyang(), Yinyang::Yang);
        assert_eq!(Tiangan::Xin.yinyang(), Yinyang::Yin);
        assert_eq!(Tiangan::Ren.yinyang(), Yinyang::Yang);
        assert_eq!(Tiangan::Gui.yinyang(), Yinyang::Yin);
    }
}
