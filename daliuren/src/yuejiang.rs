use std::fmt;

use shushu::Dizhi;
use shushu::Wuxing;
use shushu::Yinyang;
use shushu::jieqi::{self, Jieqi};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yuejiang {
    Dengming,
    Hekui,
    Congkui,
    Chuansong,
    Xiaoji,
    Shengguang,
    Taiyi,
    Tiangang,
    Taichong,
    Gongcao,
    Daji,
    Shenhou,
}

impl fmt::Display for Yuejiang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Yuejiang::Dengming => write!(f, "登明"),
            Yuejiang::Hekui => write!(f, "河魁"),
            Yuejiang::Congkui => write!(f, "从魁"),
            Yuejiang::Chuansong => write!(f, "传送"),
            Yuejiang::Xiaoji => write!(f, "小吉"),
            Yuejiang::Shengguang => write!(f, "胜光"),
            Yuejiang::Taiyi => write!(f, "太乙"),
            Yuejiang::Tiangang => write!(f, "天罡"),
            Yuejiang::Taichong => write!(f, "太冲"),
            Yuejiang::Gongcao => write!(f, "功曹"),
            Yuejiang::Daji => write!(f, "大吉"),
            Yuejiang::Shenhou => write!(f, "神后"),
        }
    }
}

impl Yuejiang {
    pub fn dizhi(self) -> Dizhi {
        match self {
            Yuejiang::Dengming => Dizhi::Hai,
            Yuejiang::Hekui => Dizhi::Xu,
            Yuejiang::Congkui => Dizhi::You,
            Yuejiang::Chuansong => Dizhi::Shen,
            Yuejiang::Xiaoji => Dizhi::Wei,
            Yuejiang::Shengguang => Dizhi::Wu,
            Yuejiang::Taiyi => Dizhi::Si,
            Yuejiang::Tiangang => Dizhi::Chen,
            Yuejiang::Taichong => Dizhi::Mao,
            Yuejiang::Gongcao => Dizhi::Yin,
            Yuejiang::Daji => Dizhi::Chou,
            Yuejiang::Shenhou => Dizhi::Zi,
        }
    }

    pub fn wuxing(self) -> Wuxing {
        self.dizhi().wuxing()
    }

    pub fn yinyang(self) -> Yinyang {
        self.dizhi().yinyang()
    }

    pub fn ordinal(self) -> usize {
        match self {
            Yuejiang::Dengming => 1,
            Yuejiang::Hekui => 2,
            Yuejiang::Congkui => 3,
            Yuejiang::Chuansong => 4,
            Yuejiang::Xiaoji => 5,
            Yuejiang::Shengguang => 6,
            Yuejiang::Taiyi => 7,
            Yuejiang::Tiangang => 8,
            Yuejiang::Taichong => 9,
            Yuejiang::Gongcao => 10,
            Yuejiang::Daji => 11,
            Yuejiang::Shenhou => 12,
        }
    }

    pub fn from_dizhi(d: Dizhi) -> Self {
        match d {
            Dizhi::Hai => Yuejiang::Dengming,
            Dizhi::Xu => Yuejiang::Hekui,
            Dizhi::You => Yuejiang::Congkui,
            Dizhi::Shen => Yuejiang::Chuansong,
            Dizhi::Wei => Yuejiang::Xiaoji,
            Dizhi::Wu => Yuejiang::Shengguang,
            Dizhi::Si => Yuejiang::Taiyi,
            Dizhi::Chen => Yuejiang::Tiangang,
            Dizhi::Mao => Yuejiang::Taichong,
            Dizhi::Yin => Yuejiang::Gongcao,
            Dizhi::Chou => Yuejiang::Daji,
            Dizhi::Zi => Yuejiang::Shenhou,
        }
    }

    pub fn from_ordinal(n: usize) -> Self {
        match n % 12 {
            0 => Yuejiang::Shenhou,
            1 => Yuejiang::Dengming,
            2 => Yuejiang::Hekui,
            3 => Yuejiang::Congkui,
            4 => Yuejiang::Chuansong,
            5 => Yuejiang::Xiaoji,
            6 => Yuejiang::Shengguang,
            7 => Yuejiang::Taiyi,
            8 => Yuejiang::Tiangang,
            9 => Yuejiang::Taichong,
            10 => Yuejiang::Gongcao,
            11 => Yuejiang::Daji,
            _ => unreachable!(),
        }
    }

    pub fn from_jieqi(jq: Jieqi) -> Option<Self> {
        match jq {
            Jieqi::Yushui => Some(Yuejiang::Dengming),
            Jieqi::Chunfen => Some(Yuejiang::Hekui),
            Jieqi::Guyu => Some(Yuejiang::Congkui),
            Jieqi::Xiaoman => Some(Yuejiang::Chuansong),
            Jieqi::Xiazhi => Some(Yuejiang::Xiaoji),
            Jieqi::Dashu => Some(Yuejiang::Shengguang),
            Jieqi::Chushu => Some(Yuejiang::Taiyi),
            Jieqi::Qiufen => Some(Yuejiang::Tiangang),
            Jieqi::Shuangjiang => Some(Yuejiang::Taichong),
            Jieqi::Xiaoxue => Some(Yuejiang::Gongcao),
            Jieqi::Dongzhi => Some(Yuejiang::Daji),
            Jieqi::Dahan => Some(Yuejiang::Shenhou),
            _ => None,
        }
    }

    pub fn from_date(date: time::Date) -> Self {
        let date_jdn = jieqi::julian_day_number(date.year(), date.month() as u8, date.day());
        let y = date.year();

        let mut zhongqi: Vec<(Jieqi, i32)> = Vec::with_capacity(36);
        for year_offset in [-1i32, 0, 1] {
            let terms = jieqi::all_jieqi(y + year_offset);
            for (jq, y2, m, d) in terms {
                if !jq.is_jie() {
                    zhongqi.push((jq, jieqi::julian_day_number(y2, m, d)));
                }
            }
        }

        let best = zhongqi
            .iter()
            .filter(|(_, jdn)| *jdn <= date_jdn)
            .max_by_key(|(_, jdn)| *jdn)
            .expect("at least one zhongqi before any date");

        Self::from_jieqi(best.0).expect("zhongqi always maps to yuejiang")
    }

    pub fn iter() -> impl Iterator<Item = Yuejiang> {
        [
            Yuejiang::Dengming,
            Yuejiang::Hekui,
            Yuejiang::Congkui,
            Yuejiang::Chuansong,
            Yuejiang::Xiaoji,
            Yuejiang::Shengguang,
            Yuejiang::Taiyi,
            Yuejiang::Tiangang,
            Yuejiang::Taichong,
            Yuejiang::Gongcao,
            Yuejiang::Daji,
            Yuejiang::Shenhou,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u8, d: u8) -> time::Date {
        let month = match m {
            1 => time::Month::January,
            2 => time::Month::February,
            3 => time::Month::March,
            4 => time::Month::April,
            5 => time::Month::May,
            6 => time::Month::June,
            7 => time::Month::July,
            8 => time::Month::August,
            9 => time::Month::September,
            10 => time::Month::October,
            11 => time::Month::November,
            12 => time::Month::December,
            _ => panic!("invalid month"),
        };
        time::Date::from_calendar_date(y, month, d).expect("valid date")
    }

    #[test]
    fn count() {
        assert_eq!(Yuejiang::iter().count(), 12);
    }

    #[test]
    fn display() {
        let names: Vec<String> = Yuejiang::iter().map(|y| y.to_string()).collect();
        assert_eq!(
            names.join(""),
            "登明河魁从魁传送小吉胜光太乙天罡太冲功曹大吉神后"
        );
    }

    #[test]
    fn ordinal() {
        assert_eq!(Yuejiang::Dengming.ordinal(), 1);
        assert_eq!(Yuejiang::Shenhou.ordinal(), 12);
    }

    #[test]
    fn dizhi_mapping() {
        assert_eq!(Yuejiang::Dengming.dizhi(), Dizhi::Hai);
        assert_eq!(Yuejiang::Hekui.dizhi(), Dizhi::Xu);
        assert_eq!(Yuejiang::Congkui.dizhi(), Dizhi::You);
        assert_eq!(Yuejiang::Chuansong.dizhi(), Dizhi::Shen);
        assert_eq!(Yuejiang::Xiaoji.dizhi(), Dizhi::Wei);
        assert_eq!(Yuejiang::Shengguang.dizhi(), Dizhi::Wu);
        assert_eq!(Yuejiang::Taiyi.dizhi(), Dizhi::Si);
        assert_eq!(Yuejiang::Tiangang.dizhi(), Dizhi::Chen);
        assert_eq!(Yuejiang::Taichong.dizhi(), Dizhi::Mao);
        assert_eq!(Yuejiang::Gongcao.dizhi(), Dizhi::Yin);
        assert_eq!(Yuejiang::Daji.dizhi(), Dizhi::Chou);
        assert_eq!(Yuejiang::Shenhou.dizhi(), Dizhi::Zi);
    }

    #[test]
    fn from_dizhi_roundtrip() {
        for d in Dizhi::iter() {
            assert_eq!(Yuejiang::from_dizhi(d).dizhi(), d);
        }
    }

    #[test]
    fn from_ordinal_roundtrip() {
        for n in 1..=12 {
            assert_eq!(Yuejiang::from_ordinal(n).ordinal(), n);
        }
    }

    #[test]
    fn from_ordinal_wrap() {
        assert_eq!(Yuejiang::from_ordinal(13).ordinal(), 1);
        assert_eq!(Yuejiang::from_ordinal(0).ordinal(), 12);
    }

    #[test]
    fn from_jieqi_jie_returns_none() {
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Lichun), None);
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Jingzhe), None);
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Qingming), None);
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Lixia), None);
    }

    #[test]
    fn from_jieqi_qi_returns_some() {
        assert_eq!(
            Yuejiang::from_jieqi(Jieqi::Yushui),
            Some(Yuejiang::Dengming)
        );
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Chunfen), Some(Yuejiang::Hekui));
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Guyu), Some(Yuejiang::Congkui));
        assert_eq!(
            Yuejiang::from_jieqi(Jieqi::Xiaoman),
            Some(Yuejiang::Chuansong)
        );
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Xiazhi), Some(Yuejiang::Xiaoji));
        assert_eq!(
            Yuejiang::from_jieqi(Jieqi::Dashu),
            Some(Yuejiang::Shengguang)
        );
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Chushu), Some(Yuejiang::Taiyi));
        assert_eq!(
            Yuejiang::from_jieqi(Jieqi::Qiufen),
            Some(Yuejiang::Tiangang)
        );
        assert_eq!(
            Yuejiang::from_jieqi(Jieqi::Shuangjiang),
            Some(Yuejiang::Taichong)
        );
        assert_eq!(
            Yuejiang::from_jieqi(Jieqi::Xiaoxue),
            Some(Yuejiang::Gongcao)
        );
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Dongzhi), Some(Yuejiang::Daji));
        assert_eq!(Yuejiang::from_jieqi(Jieqi::Dahan), Some(Yuejiang::Shenhou));
    }

    #[test]
    fn from_jieqi_all_qi_covered() {
        let mut covered = 0usize;
        for jq in Jieqi::iter() {
            if Yuejiang::from_jieqi(jq).is_some() {
                covered += 1;
            }
        }
        assert_eq!(covered, 12, "exactly 12 zhongqi should map to yuejiang");
    }

    #[test]
    fn wuxing() {
        assert_eq!(Yuejiang::Dengming.wuxing(), Wuxing::Shui);
        assert_eq!(Yuejiang::Hekui.wuxing(), Wuxing::Tu);
        assert_eq!(Yuejiang::Shengguang.wuxing(), Wuxing::Huo);
        assert_eq!(Yuejiang::Gongcao.wuxing(), Wuxing::Mu);
        assert_eq!(Yuejiang::Shenhou.wuxing(), Wuxing::Shui);
    }

    #[test]
    fn yinyang() {
        assert_eq!(Yuejiang::Dengming.yinyang(), Yinyang::Yin);
        assert_eq!(Yuejiang::Hekui.yinyang(), Yinyang::Yang);
        assert_eq!(Yuejiang::Shenhou.yinyang(), Yinyang::Yang);
    }

    #[test]
    fn yinyang_balance() {
        let yang: Vec<_> = Yuejiang::iter()
            .filter(|y| y.yinyang() == Yinyang::Yang)
            .collect();
        let yin: Vec<_> = Yuejiang::iter()
            .filter(|y| y.yinyang() == Yinyang::Yin)
            .collect();
        assert_eq!(yang.len(), 6);
        assert_eq!(yin.len(), 6);
    }

    #[test]
    fn from_date_at_zhongqi() {
        let year = 2025;
        let terms = jieqi::all_jieqi(year);
        let zhongqi: Vec<_> = terms
            .into_iter()
            .filter(|(jq, _, _, _)| !jq.is_jie())
            .collect();

        assert_eq!(zhongqi.len(), 12, "should have 12 zhongqi");

        for (jq, y, m, d) in &zhongqi {
            let zq_date = date(*y, *m, *d);
            let yue = Yuejiang::from_date(zq_date);
            let expected = Yuejiang::from_jieqi(*jq).unwrap();
            assert_eq!(yue, expected, "at {:?} {}/{}/{}", jq, y, m, d);
        }
    }

    #[test]
    fn from_date_mid_summer() {
        let mid_summer = date(2025, 7, 1);
        let yue = Yuejiang::from_date(mid_summer);
        assert!(
            yue == Yuejiang::Xiaoji || yue == Yuejiang::Shengguang,
            "mid summer should be Xiaoji or Shengguang, got {yue:?}"
        );
    }

    #[test]
    fn from_date_winter() {
        let mid_winter = date(2025, 12, 25);
        let yue = Yuejiang::from_date(mid_winter);
        assert!(
            yue == Yuejiang::Daji || yue == Yuejiang::Shenhou,
            "winter should be Daji or Shenhou, got {yue:?}"
        );
    }

    #[test]
    fn all_yuejiang_appear_in_year() {
        let mut seen = [false; 12];
        let year = 2025;

        for m in 1..=12 {
            let d = date(year, m as u8, 15);
            let yue = Yuejiang::from_date(d);
            seen[yue.ordinal() - 1] = true;
        }

        assert!(
            seen.iter().all(|&s| s),
            "all 12 yuejiang should appear within a year"
        );
    }
}
