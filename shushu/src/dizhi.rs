use std::fmt;

use crate::wuxing::Wuxing;
use crate::yinyang::Yinyang;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dizhi {
    Zi,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Wei,
    Shen,
    You,
    Xu,
    Hai,
}

impl fmt::Display for Dizhi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dizhi::Zi => write!(f, "子"),
            Dizhi::Chou => write!(f, "丑"),
            Dizhi::Yin => write!(f, "寅"),
            Dizhi::Mao => write!(f, "卯"),
            Dizhi::Chen => write!(f, "辰"),
            Dizhi::Si => write!(f, "巳"),
            Dizhi::Wu => write!(f, "午"),
            Dizhi::Wei => write!(f, "未"),
            Dizhi::Shen => write!(f, "申"),
            Dizhi::You => write!(f, "酉"),
            Dizhi::Xu => write!(f, "戌"),
            Dizhi::Hai => write!(f, "亥"),
        }
    }
}

impl Dizhi {
    pub fn ordinal(self) -> usize {
        self as usize + 1
    }

    pub fn wuxing(self) -> Wuxing {
        match self {
            Dizhi::Zi => Wuxing::Shui,
            Dizhi::Chou => Wuxing::Tu,
            Dizhi::Yin => Wuxing::Mu,
            Dizhi::Mao => Wuxing::Mu,
            Dizhi::Chen => Wuxing::Tu,
            Dizhi::Si => Wuxing::Huo,
            Dizhi::Wu => Wuxing::Huo,
            Dizhi::Wei => Wuxing::Tu,
            Dizhi::Shen => Wuxing::Jin,
            Dizhi::You => Wuxing::Jin,
            Dizhi::Xu => Wuxing::Tu,
            Dizhi::Hai => Wuxing::Shui,
        }
    }

    pub fn yinyang(self) -> Yinyang {
        match self {
            Dizhi::Zi | Dizhi::Yin | Dizhi::Chen | Dizhi::Wu | Dizhi::Shen | Dizhi::Xu => {
                Yinyang::Yang
            }
            Dizhi::Chou | Dizhi::Mao | Dizhi::Si | Dizhi::Wei | Dizhi::You | Dizhi::Hai => {
                Yinyang::Yin
            }
        }
    }

    pub fn shengxiao(self) -> &'static str {
        match self {
            Dizhi::Zi => "鼠",
            Dizhi::Chou => "牛",
            Dizhi::Yin => "虎",
            Dizhi::Mao => "兔",
            Dizhi::Chen => "龙",
            Dizhi::Si => "蛇",
            Dizhi::Wu => "马",
            Dizhi::Wei => "羊",
            Dizhi::Shen => "猴",
            Dizhi::You => "鸡",
            Dizhi::Xu => "狗",
            Dizhi::Hai => "猪",
        }
    }

    pub fn from_ordinal(n: usize) -> Self {
        match n % 12 {
            0 => Dizhi::Zi,
            1 => Dizhi::Chou,
            2 => Dizhi::Yin,
            3 => Dizhi::Mao,
            4 => Dizhi::Chen,
            5 => Dizhi::Si,
            6 => Dizhi::Wu,
            7 => Dizhi::Wei,
            8 => Dizhi::Shen,
            9 => Dizhi::You,
            10 => Dizhi::Xu,
            11 => Dizhi::Hai,
            _ => unreachable!(),
        }
    }

    pub fn iter() -> impl Iterator<Item = Dizhi> {
        [
            Dizhi::Zi,
            Dizhi::Chou,
            Dizhi::Yin,
            Dizhi::Mao,
            Dizhi::Chen,
            Dizhi::Si,
            Dizhi::Wu,
            Dizhi::Wei,
            Dizhi::Shen,
            Dizhi::You,
            Dizhi::Xu,
            Dizhi::Hai,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count() {
        assert_eq!(Dizhi::iter().count(), 12);
    }

    #[test]
    fn display() {
        let branches: Vec<String> = Dizhi::iter().map(|s| s.to_string()).collect();
        assert_eq!(branches.join(""), "子丑寅卯辰巳午未申酉戌亥");
    }

    #[test]
    fn ordinal() {
        assert_eq!(Dizhi::Zi.ordinal(), 1);
        assert_eq!(Dizhi::Hai.ordinal(), 12);
    }

    #[test]
    fn wuxing() {
        assert_eq!(Dizhi::Zi.wuxing(), Wuxing::Shui);
        assert_eq!(Dizhi::Chou.wuxing(), Wuxing::Tu);
        assert_eq!(Dizhi::Yin.wuxing(), Wuxing::Mu);
        assert_eq!(Dizhi::Mao.wuxing(), Wuxing::Mu);
        assert_eq!(Dizhi::Chen.wuxing(), Wuxing::Tu);
        assert_eq!(Dizhi::Si.wuxing(), Wuxing::Huo);
        assert_eq!(Dizhi::Wu.wuxing(), Wuxing::Huo);
        assert_eq!(Dizhi::Wei.wuxing(), Wuxing::Tu);
        assert_eq!(Dizhi::Shen.wuxing(), Wuxing::Jin);
        assert_eq!(Dizhi::You.wuxing(), Wuxing::Jin);
        assert_eq!(Dizhi::Xu.wuxing(), Wuxing::Tu);
        assert_eq!(Dizhi::Hai.wuxing(), Wuxing::Shui);
    }

    #[test]
    fn yinyang() {
        let yang: Vec<Dizhi> = Dizhi::iter().filter(|d| d.yinyang() == Yinyang::Yang).collect();
        let yin: Vec<Dizhi> = Dizhi::iter().filter(|d| d.yinyang() == Yinyang::Yin).collect();
        assert_eq!(yang.len(), 6);
        assert_eq!(yin.len(), 6);
    }

    #[test]
    fn shengxiao() {
        let animals: Vec<&str> = Dizhi::iter().map(|d| d.shengxiao()).collect();
        assert_eq!(animals.join(""), "鼠牛虎兔龙蛇马羊猴鸡狗猪");
    }
}
