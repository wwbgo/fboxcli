# FBoxCLI

FBox IoT 平台的 Agent-Native 命令行工具，使用 Rust 编写。

## 简介

FBoxCLI 是一个面向 AI Agent 友好的命令行工具，用于与 FBox IoT 云平台 API 进行交互。它覆盖了 FBox 平台的全部 API 接口，包括设备管理、数据监控、报警管理、历史数据查询等。

设计灵感来自 [CLI-Anything](https://github.com/HKUDS/CLI-Anything) 项目的 Agent-Native 理念。

## 特性

- **Agent-Native**: 全局 `--json` 标志输出结构化 JSON，方便 AI Agent 解析
- **自描述**: 每个命令和子命令均有 `--help` 说明
- **Token 自动管理**: 缓存 Token 并在过期前自动刷新
- **完整 API 覆盖**: 11 个命令模块，覆盖 FBox 平台全部 API
- **双模式输出**: 表格模式（人类可读）和 JSON 模式（机器可读）

## 安装

```bash
# 从源码编译
cd d:/code/fboxcli
cargo build --release

# 编译产物位于
# target/release/fboxcli.exe (Windows)
# target/release/fboxcli (Linux/macOS)
```

## 快速开始

```bash
# 方式一：开发者模式（需配置 client_id/client_secret）
fboxcli config set --server https://openapi.fbox360.com \
  --client-id YOUR_CLIENT_ID \
  --client-secret YOUR_CLIENT_SECRET
fboxcli auth login

# 方式二：用户模式（使用内置 client 凭证，只需用户名密码）
fboxcli auth login -u YOUR_USERNAME -p YOUR_PASSWORD

# 查看 FBox 列表
fboxcli box list

# 以 JSON 格式输出（AI Agent 模式）
fboxcli box list --json
```

## 命令总览

| 命令 | 说明 | 常用示例 |
|------|------|---------|
| `config` | 配置管理 | `fboxcli config set --server https://openapi.fbox360.com` |
| `auth` | 认证管理 | `fboxcli auth login` |
| `box` | FBox 设备管理 | `fboxcli box list`、`fboxcli box get FB001234` |
| `group` | FBox 分组管理 | `fboxcli group add "车间1"` |
| `dmon` | 数据监控点 | `fboxcli dmon get-value 12345 --ids 1001,1002` |
| `history` | 历史数据 | `fboxcli history query --ids 2001 --begin T1 --end T2` |
| `alarm` | 报警管理 | `fboxcli alarm list 12345`、`fboxcli alarm confirm 3001` |
| `contact` | 联系人管理 | `fboxcli contact list`、`fboxcli contact add "张三"` |
| `device` | 设备驱动管理 | `fboxcli device list 12345`、`fboxcli device drivers` |
| `control` | 统一写组 | `fboxcli control write --uid 6001 --value 100` |
| `location` | 地理位置 | `fboxcli location 12345,12346` |

## 典型使用场景

### 设备监控

```bash
# 查看所有设备及状态
fboxcli box list

# 读取监控点实时数据
fboxcli dmon get-value 12345 --ids 1001,1002,1003

# 写入控制值
fboxcli dmon set-value 12345 --id 1001 --value 75.5
```

### 数据分析

```bash
# 查询历史数据（按小时粒度）
fboxcli history query --ids 2001 --begin 1700000000000 --end 1700086400000 \
  --granularity 2 --json
```

### AI Agent 集成

```bash
# Agent 发现设备
fboxcli box list --json

# Agent 读传感器
fboxcli dmon get-value 12345 --names "温度,湿度" --json

# Agent 下发控制
fboxcli control write --name "温度控制组" --value 25.0
```

## 文档索引

| 文档 | 说明 |
|------|------|
| [快速入门](getting-started.md) | 安装、配置、首次使用、常见场景 |
| [命令参考](commands.md) | 全部命令的详细参数和使用示例 |
| [API 接口映射](api-reference.md) | CLI 命令与 FBox API 接口的对应关系 |
| [项目架构](architecture.md) | 代码结构、设计模式、扩展指南 |
| [配置说明](configuration.md) | 配置文件、Token 管理、频率限制 |

## 技术栈

- **Rust 2024 Edition**
- `clap` 4.6 - 命令行参数解析
- `reqwest` 0.13 + `tokio` - 异步 HTTP 客户端
- `serde` / `serde_json` / `toml` - 序列化
- `tabled` 0.20 - 表格输出
- `anyhow` - 错误处理

## 许可证

MIT
