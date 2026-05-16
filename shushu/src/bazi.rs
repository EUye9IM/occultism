use std::fmt;

use crate::dizhi::Dizhi;
use crate::jieqi;
use crate::tiangan::Tiangan;

/// 十二时辰
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shichen {
    Zi,   // 子时 23:00-00:59
    Chou, // 丑时 01:00-02:59
    Yin,  // 寅时 03:00-04:59
    Mao,  // 卯时 05:00-06:59
    Chen, // 辰时 07:00-08:59
    Si,   // 巳时 09:00-10:59
    Wu,   // 午时 11:00-12:59
    Wei,  // 未时 13:00-14:59
    Shen, // 申时 15:00-16:59
    You,  // 酉时 17:00-18:59
    Xu,   // 戌时 19:00-20:59
    Hai,  // 亥时 21:00-22:59
}

impl fmt::Display for Shichen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shichen::Zi => write!(f, "子时"),
            Shichen::Chou => write!(f, "丑时"),
            Shichen::Yin => write!(f, "寅时"),
            Shichen::Mao => write!(f, "卯时"),
            Shichen::Chen => write!(f, "辰时"),
            Shichen::Si => write!(f, "巳时"),
            Shichen::Wu => write!(f, "午时"),
            Shichen::Wei => write!(f, "未时"),
            Shichen::Shen => write!(f, "申时"),
            Shichen::You => write!(f, "酉时"),
            Shichen::Xu => write!(f, "戌时"),
            Shichen::Hai => write!(f, "亥时"),
        }
    }
}

impl Shichen {
    pub fn from_hour_minute(hour: u8, minute: u8) -> Self {
        let _ = minute;
        match hour {
            23 | 0 => Shichen::Zi,
            1 | 2 => Shichen::Chou,
            3 | 4 => Shichen::Yin,
            5 | 6 => Shichen::Mao,
            7 | 8 => Shichen::Chen,
            9 | 10 => Shichen::Si,
            11 | 12 => Shichen::Wu,
            13 | 14 => Shichen::Wei,
            15 | 16 => Shichen::Shen,
            17 | 18 => Shichen::You,
            19 | 20 => Shichen::Xu,
            21 | 22 => Shichen::Hai,
            _ => Shichen::Zi,
        }
    }
}

/// 一柱（天干 + 地支）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pillar {
    pub stem: Tiangan,
    pub branch: Dizhi,
}

impl Pillar {
    pub fn new(stem: Tiangan, branch: Dizhi) -> Self {
        Self { stem, branch }
    }
}

impl fmt::Display for Pillar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.stem, self.branch)
    }
}

/// 四柱八字
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bazi {
    pub year_pillar: Pillar,
    pub month_pillar: Pillar,
    pub day_pillar: Pillar,
    pub hour_pillar: Pillar,
}

impl fmt::Display for Bazi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.year_pillar, self.month_pillar, self.day_pillar, self.hour_pillar
        )
    }
}

impl Bazi {
    /// 从公历日期和时间计算四柱八字
    ///
    /// `date`: 公历日期
    /// `hour`: 小时 (0-23)
    /// `minute`: 分钟 (0-59)
    pub fn from_date(date: time::Date, hour: u8, minute: u8) -> Self {
        let shichen = Shichen::from_hour_minute(hour, minute);

        // 晚子时 (23:00-23:59) 日柱按次日计算
        let day_date = if hour == 23 {
            date.next_day().unwrap_or(date)
        } else {
            date
        };

        let year_pillar = Self::calc_year_pillar(date);
        let month_pillar = Self::calc_month_pillar(date);
        let day_pillar = Self::calc_day_pillar(day_date);
        let hour_pillar = Self::calc_hour_pillar(day_pillar.stem, shichen);

        Self {
            year_pillar,
            month_pillar,
            day_pillar,
            hour_pillar,
        }
    }

    /// 计算年柱
    ///
    /// 年柱以立春为界，立春前属上一年
    fn calc_year_pillar(date: time::Date) -> Pillar {
        let (chinese_year, _) = jieqi::month_index_for_date(date);

        let stem_idx = (chinese_year - 4).rem_euclid(10) as usize;
        let branch_idx = (chinese_year - 4).rem_euclid(12) as usize;

        Pillar {
            stem: Tiangan::from_ordinal(stem_idx),
            branch: Dizhi::from_ordinal(branch_idx),
        }
    }

    /// 计算月柱（五虎遁）
    fn calc_month_pillar(date: time::Date) -> Pillar {
        let (chinese_year, month_idx) = jieqi::month_index_for_date(date);
        // month_idx: 1=寅月 ... 12=丑月

        // 五虎遁：月干起算
        // 甲己之年丙作首，乙庚之岁戊为头
        // 丙辛之年寻庚上，丁壬壬寅顺水流
        // 若问戊癸何处起，甲寅之上好追求
        let year_stem_idx = (chinese_year - 4).rem_euclid(10) as usize;
        let start_stem_idx = (year_stem_idx * 2 + 2) % 10;
        let stem_idx = (start_stem_idx + month_idx - 1) % 10;

        // 月支：寅=2, 卯=3, ..., 丑=1
        let branch_idx = (month_idx + 1) % 12;

        Pillar {
            stem: Tiangan::from_ordinal(stem_idx),
            branch: Dizhi::from_ordinal(branch_idx),
        }
    }

    /// 计算日柱
    ///
    /// 基准：1900-01-01 = 甲戌日（六十甲子索引 10）
    fn calc_day_pillar(date: time::Date) -> Pillar {
        let ref_date = time::Date::from_calendar_date(1900, time::Month::January, 1)
            .expect("valid reference date");
        let days = (date - ref_date).whole_days();
        // 1900-01-01 对应甲戌，索引 10（甲子=0）
        let index = days.rem_euclid(60) as usize + 10;
        let index = index % 60;

        let stem_idx = index % 10;
        let branch_idx = index % 12;

        Pillar {
            stem: Tiangan::from_ordinal(stem_idx),
            branch: Dizhi::from_ordinal(branch_idx),
        }
    }

    /// 计算时柱（五鼠遁）
    ///
    /// day_stem: 日柱天干
    /// shichen: 时辰
    fn calc_hour_pillar(day_stem: Tiangan, shichen: Shichen) -> Pillar {
        let day_stem_idx = day_stem as usize;
        let shichen_idx = shichen as usize;

        // 五鼠遁：时干起算
        // 甲己还加甲，乙庚丙作初
        // 丙辛从戊起，丁壬庚子居
        // 戊癸何方发，壬子是真途
        let start_stem_idx = (day_stem_idx * 2) % 10;
        let stem_idx = (start_stem_idx + shichen_idx) % 10;

        Pillar {
            stem: Tiangan::from_ordinal(stem_idx),
            branch: Dizhi::from_ordinal(shichen_idx),
        }
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
    fn shichen_from_hour() {
        assert_eq!(Shichen::from_hour_minute(23, 30), Shichen::Zi);
        assert_eq!(Shichen::from_hour_minute(0, 15), Shichen::Zi);
        assert_eq!(Shichen::from_hour_minute(3, 0), Shichen::Yin);
        assert_eq!(Shichen::from_hour_minute(15, 30), Shichen::Shen);
    }

    #[test]
    fn pillar_display() {
        let p = Pillar::new(Tiangan::Jia, Dizhi::Zi);
        assert_eq!(p.to_string(), "甲子");
        let p = Pillar::new(Tiangan::Gui, Dizhi::Hai);
        assert_eq!(p.to_string(), "癸亥");
    }

    #[test]
    fn bazi_display() {
        let bazi = Bazi {
            year_pillar: Pillar::new(Tiangan::Bing, Dizhi::Wu),
            month_pillar: Pillar::new(Tiangan::Ren, Dizhi::Chen),
            day_pillar: Pillar::new(Tiangan::Ren, Dizhi::Shen),
            hour_pillar: Pillar::new(Tiangan::Wu, Dizhi::Shen),
        };
        assert_eq!(bazi.to_string(), "丙午 壬辰 壬申 戊申");
    }

    // 已知参照: 2026-04-27 15:30 排盘
    // 年: 丙午 (2026, 立春后)
    // 月: 壬辰 (清明后, 立夏前) [丙年五虎遁庚寅起, 辰月=壬辰]
    // 日: 辛未 (JDN 2461158, 从1900-01-01 甲戌日推算)
    // 时: 申时, 辛日五鼠遁戊子起, 申=丙申
    #[test]
    fn known_date_2026_04_27() {
        let d = date(2026, 4, 27);
        let bazi = Bazi::from_date(d, 15, 30);
        assert_eq!(bazi.year_pillar.to_string(), "丙午");
        assert_eq!(bazi.month_pillar.to_string(), "壬辰");
        assert_eq!(bazi.day_pillar.to_string(), "辛未");
        assert_eq!(bazi.hour_pillar.to_string(), "丙申");
    }

    // 甲辰年（2024）立春后验证
    #[test]
    fn known_date_2024_03_15() {
        let d = date(2024, 3, 15);
        let bazi = Bazi::from_date(d, 10, 0);
        // 2024 甲辰年, 甲年寅月起丙寅, 卯月丁卯
        assert_eq!(bazi.year_pillar.to_string(), "甲辰");
        // 3月15日: 惊蛰(3/6)后, 清明(4/5)前 → 卯月
        assert_eq!(bazi.month_pillar.to_string(), "丁卯");
    }

    // 立春前为上年
    #[test]
    fn before_lichun_uses_previous_year() {
        // 计算 2026 年的立春实际日期
        let near = jieqi::julian_day_number(2026, 2, 1) as f64;
        let jd = jieqi::jieqi_jd_near(jieqi::Jieqi::Lichun, near);
        let (_, _, lichun_day, _, _) = jieqi::julian_to_gregorian(jd);
        // 立春前一天
        let d = date(2026, 2, lichun_day - 1);
        let bazi = Bazi::from_date(d, 12, 0);
        assert_eq!(bazi.year_pillar.to_string(), "乙巳");
        // 丑月: 乙年五虎遁戊寅起 → 丑月 = 己丑
        assert_eq!(bazi.month_pillar.to_string(), "己丑");
    }

    // 立春后为当年
    #[test]
    fn after_lichun_uses_current_year() {
        let near = jieqi::julian_day_number(2026, 2, 1) as f64;
        let jd = jieqi::jieqi_jd_near(jieqi::Jieqi::Lichun, near);
        let (_, _, lichun_day, _, _) = jieqi::julian_to_gregorian(jd);
        // 立春当日
        let d = date(2026, 2, lichun_day);
        let bazi = Bazi::from_date(d, 12, 0);
        assert_eq!(bazi.year_pillar.to_string(), "丙午");
        // 寅月: 丙年五虎遁庚寅起 → 寅月 = 庚寅
        assert_eq!(bazi.month_pillar.to_string(), "庚寅");
    }

    // 月柱节气边界验证
    #[test]
    fn month_boundary_jingzhe() {
        // 计算 2026 年惊蛰实际日期
        let near = jieqi::julian_day_number(2026, 3, 1) as f64;
        let jd = jieqi::jieqi_jd_near(jieqi::Jieqi::Jingzhe, near);
        let (_, _, jz_day, _, _) = jieqi::julian_to_gregorian(jd);
        // 惊蛰前一天 → 寅月
        let before = date(2026, 3, jz_day - 1);
        let bazi_before = Bazi::from_date(before, 12, 0);
        assert_eq!(bazi_before.month_pillar.to_string(), "庚寅");
        // 惊蛰当日 → 卯月
        let after = date(2026, 3, jz_day);
        let bazi_after = Bazi::from_date(after, 12, 0);
        assert_eq!(bazi_after.month_pillar.to_string(), "辛卯");
    }

    // 子月跨年边界
    #[test]
    fn zi_month_cross_year() {
        // 2026年小寒实际日期
        let near = jieqi::julian_day_number(2026, 1, 1) as f64;
        let jd = jieqi::jieqi_jd_near(jieqi::Jieqi::Xiaohan, near);
        let (_, _, xh_day, _, _) = jieqi::julian_to_gregorian(jd);
        // 小寒前两天 → 子月
        let d = date(2026, 1, xh_day - 2);
        let bazi = Bazi::from_date(d, 12, 0);
        assert_eq!(bazi.year_pillar.to_string(), "乙巳");
        // 乙年: 戊寅起, 子月(11) = 戊子
        assert_eq!(bazi.month_pillar.to_string(), "戊子");
    }

    // 丑月跨年边界
    #[test]
    fn chou_month() {
        // 2026年小寒实际日期
        let near = jieqi::julian_day_number(2026, 1, 1) as f64;
        let jd = jieqi::jieqi_jd_near(jieqi::Jieqi::Xiaohan, near);
        let (_, _, xh_day, _, _) = jieqi::julian_to_gregorian(jd);
        // 小寒当日 → 丑月
        let d = date(2026, 1, xh_day);
        let bazi = Bazi::from_date(d, 12, 0);
        assert_eq!(bazi.year_pillar.to_string(), "乙巳");
        assert_eq!(bazi.month_pillar.to_string(), "己丑");
    }

    // 晚子时 (23:00-23:59) 日柱+时柱按次日排
    #[test]
    fn late_zi_hour() {
        // 2026-04-27 23:30 → 日柱按4月28日算
        let d = date(2026, 4, 27);
        let bazi = Bazi::from_date(d, 23, 30);
        // 年柱/月柱不变: 丙午 壬辰
        assert_eq!(bazi.year_pillar.to_string(), "丙午");
        assert_eq!(bazi.month_pillar.to_string(), "壬辰");
        // 日柱应为 4月28日: 壬申（辛未次日）
        assert_eq!(bazi.day_pillar.to_string(), "壬申");
        // 时柱: 壬日五鼠遁庚子起, 子时 = 庚子
        assert_eq!(bazi.hour_pillar.to_string(), "庚子");
    }

    // 早子时 (00:00-00:59) 日柱按当日算
    #[test]
    fn early_zi_hour() {
        let d = date(2026, 4, 28);
        let bazi = Bazi::from_date(d, 0, 30);
        assert_eq!(bazi.day_pillar.to_string(), "壬申");
        // 壬日, 子时 → 庚子
        assert_eq!(bazi.hour_pillar.to_string(), "庚子");
    }

    // 甲子日验证: 1900-01-01 为甲戌日, 推算甲子日
    #[test]
    fn jia_zi_day() {
        // 甲子日: 1900-01-01 (甲戌, index 10) 的前10天
        // 即 1899-12-22
        let d = date(1899, 12, 22);
        let bazi = Bazi::from_date(d, 12, 0);
        assert_eq!(bazi.day_pillar.to_string(), "甲子");
    }

    // 六十甲子循环验证: 甲子日60天后又是甲子日
    #[test]
    fn sexagenary_cycle_day() {
        let d1 = date(2024, 1, 1);
        let bazi1 = Bazi::from_date(d1, 12, 0);
        let d2 = date(2024, 3, 1); // 60天后 (2024是闰年, 1月31天+2月29天=60)
        let bazi2 = Bazi::from_date(d2, 12, 0);
        // 60天后日柱相同
        assert_eq!(bazi1.day_pillar.to_string(), bazi2.day_pillar.to_string());
    }

    // 年柱六十甲子循环验证
    #[test]
    fn sexagenary_cycle_year() {
        let d1 = date(2024, 6, 1);
        let bazi1 = Bazi::from_date(d1, 12, 0);
        let d2 = date(2084, 6, 1); // 60年后
        let bazi2 = Bazi::from_date(d2, 12, 0);
        assert_eq!(bazi1.year_pillar.to_string(), "甲辰");
        assert_eq!(bazi2.year_pillar.to_string(), "甲辰");
    }

    #[test]
    fn shichen_display() {
        assert_eq!(Shichen::Zi.to_string(), "子时");
        assert_eq!(Shichen::Wu.to_string(), "午时");
    }
}
