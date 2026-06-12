# gpui step by step 系列教程

_以下介绍由AI生成_

> 一步步掌握 [Zed](https://zed.dev) 编辑器背后的高性能 Rust GUI 框架

[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80+-orange)](https://www.rust-lang.org)
[![GPUI](https://img.shields.io/badge/gpui-0.2.2-green)](https://crates.io/crates/gpui)

---

## 关于本教程

**GPUI** 是 Zed 团队推出的 GPU 加速 Rust GUI 框架，融合即时模式范式的简洁与现代 GPU 渲染的性能，轻松达到 60–144fps 的流畅体验。

本教程通过循序渐进的实战项目，带你从零掌握 GPUI 核心概念。每个章节均配有可运行的完整代码。

> 📖 **配套专栏：**[掘金 · GPUI 实战教程](https://juejin.cn/column/7647790979405824038)

---

## 核心特性（计数器示例）

| 特性           | 说明                                |
| -------------- | ----------------------------------- |
| **响应式状态** | 内部状态变化自动触发 UI 重绘        |
| **键盘事件**   | `KeyBinding` 绑定方向键控制数值增减 |
| **鼠标交互**   | 左/中/右键分别触发不同计数操作      |
| **焦点管理**   | 组件可获取/失去焦点并响应键盘输入   |

---

## 章节目录

| 章节 | 标题                       | 核心内容                                     |
| :--: | -------------------------- | -------------------------------------------- |
|  1   | 起手式                     | 第一个 GPUI 程序、App 生命周期               |
|  2   | 核心概念                   | Render / Entity / Context、状态管理          |
|  3   | 消息传递                   | EventEmitter 事件系统、组件通信              |
|  4   | 自定义组件                 | 组件拆分与复用、props 传递                   |
|  5   | 焦点处理                   | FocusHandle、键盘与点击交互                  |
| 6🚧  | 全局状态管理与 tokio 集成  | Global trait、tokio 运行时注入、全局状态存取 |
| 7🚧  | 实践项目 TODO List         | 综合实战：布局、状态、事件、组件通信         |
| 8🚧  | 深入渲染流程 Element trait | Element trait 原理、绘制管线、自定义渲染     |
| 9🚧  | 后台线程管理               | spawn / background_spawn、异步任务调度       |

> 🚧 更多章节持续更新，欢迎 Star 关注！

---

## 快速开始

**前置要求：** Rust ≥ 1.80，macOS / Windows / Linux（需 GPU 支持）

```bash
git clone https://github.com/sslime336/gpui-step-by-step.git
cd gpui-step-by-step
cargo run
```

---

## 学习路线

1. **核心理念** — 即时模式范式、GPU 加速渲染优势
2. **组件基础** — Entity 生命周期、Render trait、Context 用法
3. **事件交互** — 鼠标/键盘事件、自定义 Action、焦点管理
4. **组件通信** — EventEmitter、消息订阅与广播
5. **完整应用** — 布局系统、样式管理、多视图组合

---

## 贡献

欢迎提交 Issue 和 PR！遇到问题或有更好的实现思路，一起完善这个教程。

---

## 许可证

[MIT](./LICENSE)

---

[🐙 GitHub](https://github.com/sslime336/gpui-step-by-step) &nbsp;·&nbsp; [📖 掘金专栏](https://juejin.cn/column/7647790979405824038)

> 如果这个教程对你有帮助，欢迎 ⭐ Star！
