# Occultism

一个 Rust 实现的中国传统玄学工具库，提供数术领域的基础数据类型、算法与核心抽象。

## 设计目标

- **类型安全**：利用 Rust 的类型系统对玄学概念进行精确建模，在编译期消除非法状态
- **高性能**：零成本抽象，满足大规模排盘与推演的计算需求
- **正确性**：穷举验证与属性测试，确保算法与经典文献一致
- **可扩展**：通过 trait 体系支持多种流派与算法的组合

## 子 crate: shushu

`shushu` 为数术基础子 crate，提供核心数据类型：

| 类型 | 说明 | 主要方法 |
|---|---|---|
| `Yinyang` | 阴阳（阴/阳） | `Display` |
| `Wuxing` | 五行（金木水火土） | `Display`, `sheng()`, `ke()` |
| `Tiangan` | 十天干（甲～癸） | `Display`, `wuxing()`, `yinyang()`, `ordinal()` |
| `Dizhi` | 十二地支（子～亥） | `Display`, `wuxing()`, `yinyang()`, `ordinal()`, `shengxiao()` |

这些类型通过根 crate 重导出，可直接 `use occultism::Tiangan` 引入。

## 功能规划

### 基础定义

- [x] 十天干（甲、乙、丙、丁、戊、己、庚、辛、壬、癸）
- [x] 十二地支（子、丑、寅、卯、辰、巳、午、未、申、酉、戌、亥）
- [x] 阴阳与五行（金、木、水、火、土）及其生克关系
- [ ] 八卦（乾、坤、震、巽、坎、离、艮、兑）与六十四卦
- [ ] 天干地支的刑、冲、合、害、破等交互关系
- [ ] 二十四节气
- [ ] 纳音五行
- [ ] 地支藏干

### 后续规划

后续将逐步扩展至六爻、八字、紫微斗数等应用层模块。

## 项目结构

```
occultism/
├── Cargo.toml          # workspace 根配置
├── src/                # occultism crate（入口，重导出子 crate）
├── shushu/             # 数术子 crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── yinyang.rs     # 阴阳
│       ├── wuxing.rs      # 五行（含生克）
│       ├── tiangan.rs     # 十天干
│       └── dizhi.rs       # 十二地支（含生肖）
└── README.md
```

## 构建

```bash
cargo build
cargo test --workspace
```

## 许可

MIT
