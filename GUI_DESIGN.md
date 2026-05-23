# Game of Life — GUI 版本

## 设计思路

### 技术选型

使用 **macroquad** 作为 GUI 渲染框架：
- 极简 API，零样板代码
- 内置 rand，无需额外依赖
- 编译速度在 Rust 游戏库中属较快一档
- 跨平台 (Linux/macOS/Windows/WASM)

### 架构

```
src/
├── main.rs    # 主循环、输入处理、状态管理
├── world.rs   # 核心逻辑（演化规则、初始化、统计）
└── gui.rs     # 绘制网格和 HUD
```

- `world.rs` — 纯逻辑，不依赖任何 GUI 代码，方便测试
- `gui.rs` — 仅负责绘制，读取 world 状态渲染
- `main.rs` — 胶水层，处理输入事件并驱动 world 演化

### 核心改进

1. **修复 off-by-one**：原版循环 `0..74` 漏掉了第 75 行/列，改为 `0..SIZE`
2. **邻居计算重写**：用双层 -1..=1 循环替代 8 个 if 分支，更清晰不易出错
3. **传引用**：world 参数改为 `&World` 避免 75x75 数组的无谓拷贝

### 交互控制

| 按键 | 功能 |
|------|------|
| Escape / Q | 退出程序 |
| Space | 暂停 / 继续 |
| 鼠标左键 | 翻转细胞状态 |
| R | 随机重新生成 |
| C | 清空所有细胞 |
| + / = | 加速 |
| - | 减速 |

### 渲染策略

- 窗口自适应：cell_size 根据窗口大小动态计算
- 活细胞绿色，死细胞深灰，1px 间隙形成网格线
- 顶部 HUD 显示代数、种群、速度、状态

## 如何运行

```bash
# 首次编译（macroquad 首次编译约 2-3 分钟，后续增量编译很快）
cargo run

# 从文件加载初始状态
cargo run -- pattern.txt
```

### pattern.txt 格式

每行两个数字（空格分隔），表示活细胞的 (行, 列) 坐标：

```
10 10
10 11
10 12
11 10
11 12
12 10
12 11
12 12
```

### 系统依赖 (Linux)

macroquad 需要一些系统库：

```bash
# Ubuntu/Debian
sudo apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# Arch
sudo pacman -S pkg-config libx11 libxi mesa alsa-lib
```

## 删除旧 Cargo.lock

首次编译前建议删除旧的 Cargo.lock，让 cargo 重新解析依赖：

```bash
rm Cargo.lock
cargo run
```
