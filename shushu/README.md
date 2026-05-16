# Shushu（数术）

数术基础子 crate，提供中国传统玄学的核心数据类型。

## 已实现类型

| 类型 | 说明 | 主要方法 |
|---|---|---|
| `Yinyang` | 阴阳（阴/阳） | `Display` |
| `Wuxing` | 五行（金木水火土） | `Display`, `sheng()`, `ke()` |
| `Tiangan` | 十天干（甲～癸） | `Display`, `wuxing()`, `yinyang()`, `ordinal()` |
| `Dizhi` | 十二地支（子～亥） | `Display`, `wuxing()`, `yinyang()`, `ordinal()`, `shengxiao()` |
| `Jieqi` | 二十四节气（立春～大寒） | `Display`, `solar_angle()`, `is_jie()`, `month_index()` |
| `Shichen` | 十二时辰（子～亥） | `Display`, `from_hour_minute()` |
| `Pillar` | 一柱（天干+地支） | `Display`, `new()` |
| `Bazi` | 四柱八字（年柱+月柱+日柱+时柱） | `from_date()` |

## 功能规划

- [x] 十天干（甲、乙、丙、丁、戊、己、庚、辛、壬、癸）
- [x] 十二地支（子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥）
- [x] 阴阳与五行（金、木、水、火、土）及其生克关系
- [x] 十二时辰（子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥）
- [x] 二十四节气（立春、雨水……小寒、大寒）含天文算法
- [x] 四柱八字（年柱、月柱、日柱、时柱）含排盘算法
- [ ] 八卦（乾、坤、震、巽、坎、离、艮、兑）与六十四卦
- [ ] 天干地支的刑、冲、合、害、破等交互关系
- [ ] 纳音五行
- [ ] 地支藏干

## 构建

```bash
cargo build -p shushu
cargo test -p shushu
```

## 许可

MIT
