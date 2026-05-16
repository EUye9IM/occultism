use std::fmt;

/// 二十四节气
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jieqi {
    Lichun,      // 立春 315°
    Yushui,      // 雨水 330°
    Jingzhe,     // 惊蛰 345°
    Chunfen,     // 春分 0°
    Qingming,    // 清明 15°
    Guyu,        // 谷雨 30°
    Lixia,       // 立夏 45°
    Xiaoman,     // 小满 60°
    Mangzhong,   // 芒种 75°
    Xiazhi,      // 夏至 90°
    Xiaoshu,     // 小暑 105°
    Dashu,       // 大暑 120°
    Liqiu,       // 立秋 135°
    Chushu,      // 处暑 150°
    Bailu,       // 白露 165°
    Qiufen,      // 秋分 180°
    Hanlu,       // 寒露 195°
    Shuangjiang, // 霜降 210°
    Lidong,      // 立冬 225°
    Xiaoxue,     // 小雪 240°
    Daxue,       // 大雪 255°
    Dongzhi,     // 冬至 270°
    Xiaohan,     // 小寒 285°
    Dahan,       // 大寒 300°
}

impl fmt::Display for Jieqi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Jieqi::Lichun => write!(f, "立春"),
            Jieqi::Yushui => write!(f, "雨水"),
            Jieqi::Jingzhe => write!(f, "惊蛰"),
            Jieqi::Chunfen => write!(f, "春分"),
            Jieqi::Qingming => write!(f, "清明"),
            Jieqi::Guyu => write!(f, "谷雨"),
            Jieqi::Lixia => write!(f, "立夏"),
            Jieqi::Xiaoman => write!(f, "小满"),
            Jieqi::Mangzhong => write!(f, "芒种"),
            Jieqi::Xiazhi => write!(f, "夏至"),
            Jieqi::Xiaoshu => write!(f, "小暑"),
            Jieqi::Dashu => write!(f, "大暑"),
            Jieqi::Liqiu => write!(f, "立秋"),
            Jieqi::Chushu => write!(f, "处暑"),
            Jieqi::Bailu => write!(f, "白露"),
            Jieqi::Qiufen => write!(f, "秋分"),
            Jieqi::Hanlu => write!(f, "寒露"),
            Jieqi::Shuangjiang => write!(f, "霜降"),
            Jieqi::Lidong => write!(f, "立冬"),
            Jieqi::Xiaoxue => write!(f, "小雪"),
            Jieqi::Daxue => write!(f, "大雪"),
            Jieqi::Dongzhi => write!(f, "冬至"),
            Jieqi::Xiaohan => write!(f, "小寒"),
            Jieqi::Dahan => write!(f, "大寒"),
        }
    }
}

impl Jieqi {
    /// 太阳视黄经（角度）
    pub fn solar_angle(self) -> f64 {
        match self {
            Jieqi::Lichun => 315.0,
            Jieqi::Yushui => 330.0,
            Jieqi::Jingzhe => 345.0,
            Jieqi::Chunfen => 0.0,
            Jieqi::Qingming => 15.0,
            Jieqi::Guyu => 30.0,
            Jieqi::Lixia => 45.0,
            Jieqi::Xiaoman => 60.0,
            Jieqi::Mangzhong => 75.0,
            Jieqi::Xiazhi => 90.0,
            Jieqi::Xiaoshu => 105.0,
            Jieqi::Dashu => 120.0,
            Jieqi::Liqiu => 135.0,
            Jieqi::Chushu => 150.0,
            Jieqi::Bailu => 165.0,
            Jieqi::Qiufen => 180.0,
            Jieqi::Hanlu => 195.0,
            Jieqi::Shuangjiang => 210.0,
            Jieqi::Lidong => 225.0,
            Jieqi::Xiaoxue => 240.0,
            Jieqi::Daxue => 255.0,
            Jieqi::Dongzhi => 270.0,
            Jieqi::Xiaohan => 285.0,
            Jieqi::Dahan => 300.0,
        }
    }

    /// 是否为"节"（12 节为月柱分界，"气"为月中）
    pub fn is_jie(self) -> bool {
        (self as usize) % 2 == 0
    }

    /// 节气序号（立春=0 ... 大寒=23）
    pub fn ordinal(self) -> usize {
        self as usize
    }

    /// 对应的月支索引（寅月=1 ... 丑月=12），仅对"节"有效
    pub fn month_index(self) -> usize {
        self as usize / 2 + 1
    }

    pub fn iter() -> impl Iterator<Item = Jieqi> {
        [
            Jieqi::Lichun,
            Jieqi::Yushui,
            Jieqi::Jingzhe,
            Jieqi::Chunfen,
            Jieqi::Qingming,
            Jieqi::Guyu,
            Jieqi::Lixia,
            Jieqi::Xiaoman,
            Jieqi::Mangzhong,
            Jieqi::Xiazhi,
            Jieqi::Xiaoshu,
            Jieqi::Dashu,
            Jieqi::Liqiu,
            Jieqi::Chushu,
            Jieqi::Bailu,
            Jieqi::Qiufen,
            Jieqi::Hanlu,
            Jieqi::Shuangjiang,
            Jieqi::Lidong,
            Jieqi::Xiaoxue,
            Jieqi::Daxue,
            Jieqi::Dongzhi,
            Jieqi::Xiaohan,
            Jieqi::Dahan,
        ]
        .into_iter()
    }
}

/// 计算给定儒略日的太阳视黄经（度，归一化到 [0, 360)）
///
/// 使用 Meeus《Astronomical Algorithms》的低精度公式，含章动和光行差修正。
pub fn solar_longitude(jd: f64) -> f64 {
    // 儒略世纪数（J2000.0 = JD 2451545.0）
    let t = (jd - 2451545.0) / 36525.0;

    // 太阳平黄经
    let l0 = (280.46646 + 36000.76983 * t + 0.0003032 * t * t).rem_euclid(360.0);

    // 太阳平近点角
    let m = (357.52911 + 35999.05029 * t - 0.0001537 * t * t).rem_euclid(360.0);
    let m_rad = m.to_radians();

    // 中心差
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * m_rad.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * m_rad).sin()
        + 0.000289 * (3.0 * m_rad).sin();

    // 真黄经
    let true_lon = l0 + c;

    // 月球升交点黄经
    let omega = 125.04 - 1934.136 * t;
    let omega_rad = omega.to_radians();

    // 章动与光行差修正后的视黄经
    let apparent = true_lon - 0.00569 - 0.00478 * omega_rad.sin();

    apparent.rem_euclid(360.0)
}

/// 计算儒略日（整数，公历日期）
pub fn julian_day_number(year: i32, month: u8, day: u8) -> i32 {
    let a = (14 - month as i32) / 12;
    let y = year + 4800 - a;
    let m = month as i32 + 12 * a - 3;
    day as i32 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

/// 在参考儒略日附近搜索某个节气的儒略日
///
/// 从 `near_jd` 出发用牛顿法收敛到该节气，保证找到最近一次发生。
pub fn jieqi_jd_near(jq: Jieqi, near_jd: f64) -> f64 {
    let angle = jq.solar_angle();
    let mut jd = near_jd;

    for _ in 0..15 {
        let current = solar_longitude(jd);
        let mut diff = angle - current;

        if diff > 180.0 {
            diff -= 360.0;
        } else if diff < -180.0 {
            diff += 360.0;
        }

        if diff.abs() < 1e-8 {
            break;
        }

        jd += diff / 0.985647;
    }

    jd
}

/// 儒略日 → 公历日期 (年, 月, 日, 时, 分)
///
/// 仅适用于 Gregorian 历（1582-10-15 之后）。
pub fn julian_to_gregorian(jd: f64) -> (i32, u8, u8, u8, u8) {
    let z = (jd + 0.5).floor();
    let f = jd + 0.5 - z;
    let z = z as i64;

    let alpha = ((z as f64 - 1867216.25) / 36524.25) as i64;
    let a = z + 1 + alpha - alpha / 4;
    let b = a + 1524;
    let c = ((b as f64 - 122.1) / 365.25).floor() as i64;
    let d = (365.25 * c as f64).floor() as i64;
    let e = ((b - d) as f64 / 30.6001).floor() as i64;

    let day = (b - d - (30.6001 * e as f64).floor() as i64 + f.floor() as i64) as u8;
    let month_e = e as i32;
    let month = if month_e < 14 {
        month_e - 1
    } else {
        month_e - 13
    } as u8;
    let year = if month > 2 { c - 4716 } else { c - 4715 } as i32;

    let frac = f - f.floor();
    let hour = (frac * 24.0).floor() as u8;
    let minute = ((frac * 24.0 - hour as f64) * 60.0).floor() as u8;

    (year, month, day, hour, minute)
}

/// 获取给定日期的月支索引（寅月=1 ... 丑月=12），返回(农历年, 月序号)
///
/// 使用天文算法串行链式搜索 12 个"节"的精确日期。
pub fn month_index_for_date(date: time::Date) -> (i32, usize) {
    let y = date.year();
    let md = (date.month() as u8, date.day());

    let jie_list = [
        Jieqi::Lichun,
        Jieqi::Jingzhe,
        Jieqi::Qingming,
        Jieqi::Lixia,
        Jieqi::Mangzhong,
        Jieqi::Xiaoshu,
        Jieqi::Liqiu,
        Jieqi::Bailu,
        Jieqi::Hanlu,
        Jieqi::Lidong,
        Jieqi::Daxue,
        Jieqi::Xiaohan,
    ];

    // 先找当年立春，确定农历年
    let near_jd = julian_day_number(y, 2, 1) as f64;
    let lichun_jd = jieqi_jd_near(Jieqi::Lichun, near_jd);
    let (_, lm, ld, _, _) = julian_to_gregorian(lichun_jd);
    let chinese_year = if is_before(md, (lm, ld)) { y - 1 } else { y };

    // 从农历年的 2 月 1 日起重新链式搜索 12 个节
    let mut cur_jd = julian_day_number(chinese_year, 2, 1) as f64;
    let mut jie_jdns: Vec<i32> = Vec::with_capacity(12);

    for &jie in &jie_list {
        cur_jd = jieqi_jd_near(jie, cur_jd);
        let (jy, jm, jd, _, _) = julian_to_gregorian(cur_jd);
        jie_jdns.push(julian_day_number(jy, jm, jd));
        cur_jd += 25.0;
    }

    // 用儒略日比较，正确处理跨年边界
    let date_jdn = julian_day_number(date.year(), date.month() as u8, date.day());

    for i in 0..11 {
        if date_jdn >= jie_jdns[i] && date_jdn < jie_jdns[i + 1] {
            return (chinese_year, i + 1);
        }
    }

    (chinese_year, 12)
}

fn is_before(a: (u8, u8), b: (u8, u8)) -> bool {
    a.0 < b.0 || (a.0 == b.0 && a.1 < b.1)
}

/// 某年的全部 24 节气（按时间顺序，从立春开始串行搜索）
pub fn all_jieqi(year: i32) -> Vec<(Jieqi, i32, u8, u8)> {
    let all_jieqi: Vec<Jieqi> = Jieqi::iter().collect();

    let mut cur_jd = julian_day_number(year, 2, 1) as f64;
    let mut result = Vec::with_capacity(24);

    for &jie in &all_jieqi {
        cur_jd = jieqi_jd_near(jie, cur_jd);
        let (y, m, d, _, _) = julian_to_gregorian(cur_jd);
        result.push((jie, y, m, d));
        cur_jd += 13.0; // 下一节约 15 天后
    }

    result
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
    fn jieqi_display() {
        assert_eq!(Jieqi::Lichun.to_string(), "立春");
        assert_eq!(Jieqi::Xiazhi.to_string(), "夏至");
        assert_eq!(Jieqi::Dongzhi.to_string(), "冬至");
        assert_eq!(Jieqi::Dahan.to_string(), "大寒");
    }

    #[test]
    fn jieqi_count() {
        assert_eq!(Jieqi::iter().count(), 24);
    }

    #[test]
    fn jie_vs_qi() {
        assert!(Jieqi::Lichun.is_jie());
        assert!(!Jieqi::Yushui.is_jie());
        assert!(Jieqi::Jingzhe.is_jie());
        assert!(!Jieqi::Chunfen.is_jie());
    }

    #[test]
    fn jieqi_solar_angle() {
        assert!((Jieqi::Lichun.solar_angle() - 315.0).abs() < 0.01);
        assert!((Jieqi::Chunfen.solar_angle() - 0.0).abs() < 0.01);
        assert!((Jieqi::Dongzhi.solar_angle() - 270.0).abs() < 0.01);
    }

    #[test]
    fn jieqi_month_index() {
        // 立春 → 寅月 (month 1)
        assert_eq!(Jieqi::Lichun.month_index(), 1);
        // 惊蛰 → 卯月 (month 2)
        assert_eq!(Jieqi::Jingzhe.month_index(), 2);
        // 小寒 → 丑月 (month 12)
        assert_eq!(Jieqi::Xiaohan.month_index(), 12);
    }

    #[test]
    fn solar_longitude_known_date() {
        // J2000.0 (2000-01-01 12:00 UT) ≈ JD 2451545.0
        // 此时太阳视黄经约 280°
        let jd = 2451545.0;
        let lon = solar_longitude(jd);
        assert!((lon - 280.0).abs() < 2.0);
    }

    // 已知 2026 年节气参照（数据来源：香港天文台）
    #[test]
    fn known_jieqi_2026() {
        // 2026年立春: 2月3日或4日
        let near = julian_day_number(2026, 2, 1) as f64;
        let (y, m, d, _, _) = julian_to_gregorian(jieqi_jd_near(Jieqi::Lichun, near));
        assert_eq!(y, 2026);
        assert_eq!(m, 2);
        assert!((3..=4).contains(&d), "立春日应为2月3或4日, got {}/{}", m, d);

        // 2026年春分: 3月20或21日
        let near = julian_day_number(2026, 3, 18) as f64;
        let (y, m, d, _, _) = julian_to_gregorian(jieqi_jd_near(Jieqi::Chunfen, near));
        assert_eq!(y, 2026);
        assert_eq!(m, 3);
        assert!(
            (20..=21).contains(&d),
            "春分日应为3月20或21日, got {}/{}",
            m,
            d
        );

        // 2026年夏至: 6月20或21日
        let near = julian_day_number(2026, 6, 18) as f64;
        let (_, _, d, _, _) = julian_to_gregorian(jieqi_jd_near(Jieqi::Xiazhi, near));
        assert!((20..=21).contains(&d), "夏至日应为6月20或21日, got {}", d);

        // 2026年冬至: 12月21或22日
        let near = julian_day_number(2026, 12, 18) as f64;
        let (_, m, d, _, _) = julian_to_gregorian(jieqi_jd_near(Jieqi::Dongzhi, near));
        assert_eq!(m, 12);
        assert!(
            (21..=22).contains(&d),
            "冬至日应为12月21或22日, got {}/{}",
            m,
            d
        );
    }

    #[test]
    fn month_index_jingzhe_2026() {
        // 实际惊蛰日可能为3月5或6日，以此日期为界
        let near = julian_day_number(2026, 3, 1) as f64;
        let jd_jingzhe = jieqi_jd_near(Jieqi::Jingzhe, near);
        let (_, _, jd_day, _, _) = julian_to_gregorian(jd_jingzhe);

        let before = date(2026, 3, jd_day - 1);
        let (_, mi) = month_index_for_date(before);
        assert_eq!(mi, 1); // 寅月

        let on_day = date(2026, 3, jd_day);
        let (_, mi) = month_index_for_date(on_day);
        assert_eq!(mi, 2); // 卯月
    }

    #[test]
    fn month_index_lichun_2026() {
        // 实际立春日，以此日期为界
        let near = julian_day_number(2026, 2, 1) as f64;
        let jd_lichun = jieqi_jd_near(Jieqi::Lichun, near);
        let (_, _, lichun_day, _, _) = julian_to_gregorian(jd_lichun);

        let before = date(2026, 2, lichun_day - 1);
        let (y, mi) = month_index_for_date(before);
        assert_eq!(y, 2025);
        assert_eq!(mi, 12); // 丑月

        let on_day = date(2026, 2, lichun_day);
        let (y, mi) = month_index_for_date(on_day);
        assert_eq!(y, 2026);
        assert_eq!(mi, 1); // 寅月
    }

    #[test]
    fn all_jieqi_well_ordered() {
        let terms = all_jieqi(2026);
        assert_eq!(terms.len(), 24);
        // 节气应按时间顺序排列
        for i in 1..terms.len() {
            let (_, y1, m1, d1) = terms[i - 1];
            let (_, y2, m2, d2) = terms[i];
            assert!(
                (y1, m1, d1) <= (y2, m2, d2),
                "节气 {} ({}/{}/{}) 应在 {} ({}/{}/{}) 之前",
                terms[i - 1].0,
                y1,
                m1,
                d1,
                terms[i].0,
                y2,
                m2,
                d2
            );
        }
    }

    #[test]
    fn jieqi_ordinal() {
        assert_eq!(Jieqi::Lichun.ordinal(), 0);
        assert_eq!(Jieqi::Dahan.ordinal(), 23);
    }
}
