# Occultism

一个 Rust 实现的中国传统玄学工具库，提供数术领域的基础数据类型、算法与核心抽象。

## 设计目标

- **类型安全**：利用 Rust 的类型系统对玄学概念进行精确建模，在编译期消除非法状态
- **高性能**：零成本抽象，满足大规模排盘与推演的计算需求
- **正确性**：穷举验证与属性测试，确保算法与经典文献一致
- **可扩展**：通过 trait 体系支持多种流派与算法的组合

## 子 crate

| Crate | 说明 |
|---|---|
| [`shushu`](shushu/README.md) | 数术基础库，提供天干、地支、五行、节气、八字等核心类型 |
| [`daliuren`](daliuren/README.md) | 大六壬，提供六壬占卜的数据结构与算法 |

## 项目结构

```
occultism/
├── Cargo.toml              # workspace 根配置
├── src/                    # occultism crate（入口，重导出子 crate）
├── shushu/                 # 数术子 crate
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       ├── lib.rs
│       ├── yinyang.rs         # 阴阳
│       ├── wuxing.rs          # 五行（含生克）
│       ├── tiangan.rs         # 十天干
│       ├── dizhi.rs           # 十二地支（含生肖）
│       ├── jieqi.rs           # 二十四节气（天文算法）
│       └── bazi.rs            # 四柱八字（含时辰）
├── daliuren/               # 大六壬子 crate
│   ├── Cargo.toml
│   ├── README.md
│   └── src/
│       └── lib.rs
└── README.md
```

## 构建

```bash
cargo build
cargo test --workspace
```

## 许可

MIT
