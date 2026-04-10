# 项目架构

## 目录结构

```
fboxcli/
├── Cargo.toml              # 项目依赖和元数据
├── docs/                   # 项目文档
│   ├── README.md           # 项目总览
│   ├── getting-started.md  # 快速入门
│   ├── commands.md         # 命令参考
│   ├── api-reference.md    # API 接口映射
│   ├── architecture.md     # 项目架构（本文件）
│   └── configuration.md    # 配置说明
├── src/
│   ├── main.rs             # 入口，命令分发
│   ├── config.rs           # 配置文件管理
│   ├── output.rs           # 输出格式化（JSON/Table）
│   ├── cli/                # CLI 命令定义层
│   │   ├── mod.rs          # 根命令和子命令枚举
│   │   ├── auth.rs         # auth 子命令
│   │   ├── config_cmd.rs   # config 子命令
│   │   ├── box_cmd.rs      # box 子命令
│   │   ├── group.rs        # group 子命令
│   │   ├── dmon.rs         # dmon 子命令
│   │   ├── history.rs      # history 子命令
│   │   ├── alarm.rs        # alarm 子命令
│   │   ├── contact.rs      # contact 子命令
│   │   ├── device.rs       # device 子命令
│   │   ├── control.rs      # control 子命令
│   │   └── location.rs     # location 子命令
│   ├── api/                # API 调用层
│   │   ├── mod.rs          # FBoxClient 核心（HTTP 客户端 + Token 管理）
│   │   ├── auth.rs         # 认证 API
│   │   ├── box_api.rs      # FBox 设备 API
│   │   ├── group.rs        # 分组 API
│   │   ├── dmon.rs         # 监控点 API
│   │   ├── history.rs      # 历史数据 API
│   │   ├── alarm.rs        # 报警 API
│   │   ├── contact.rs      # 联系人 API
│   │   ├── device.rs       # 设备 API
│   │   ├── control.rs      # 统一写组 API
│   │   └── location.rs     # 地理位置 API
│   └── models/             # 数据模型层
│       ├── mod.rs          # 模块声明
│       ├── auth.rs         # Token 响应模型
│       ├── box_model.rs    # FBox 设备模型
│       ├── dmon.rs         # 监控点模型
│       ├── alarm.rs        # 报警模型
│       ├── history.rs      # 历史数据模型
│       ├── device.rs       # 设备模型
│       ├── contact.rs      # 联系人模型
│       └── control.rs      # 统一写组模型
```

## 分层架构

```
┌──────────────────────────────────┐
│           main.rs                │  入口 & 命令分发
├──────────────────────────────────┤
│           cli/                   │  CLI 参数解析 (clap derive)
│  解析用户输入 → 调用 API 层     │  处理输出格式化
├──────────────────────────────────┤
│           api/                   │  API 调用层
│  FBoxClient + HTTP 请求         │  Token 自动管理
├──────────────────────────────────┤
│           models/                │  数据模型
│  Serialize / Deserialize        │  Tabled (表格输出)
├──────────────────────────────────┤
│     config.rs  │  output.rs     │  基础设施
│  配置文件读写  │  格式化输出    │
└──────────────────────────────────┘
```

## 核心组件

### FBoxClient (`src/api/mod.rs`)

HTTP 客户端核心，包含：

- `reqwest::Client` 实例（30秒超时）
- 配置信息 (`AppConfig`)
- Token 缓存 (`CachedToken`)

关键方法：

| 方法 | 说明 |
|------|------|
| `ensure_token()` | 检查 Token 有效性，过期前 60 秒自动刷新 |
| `request(method, path)` | 构建带 Bearer Token 的请求 |
| `get(path)` | GET 请求 |
| `post_json(path, body)` | POST JSON 请求 |
| `put_json(path, body)` | PUT JSON 请求 |
| `delete(path)` | DELETE 请求 |

Token 刷新策略：
1. 检查 `expires_at` 是否在 60 秒内过期
2. 如果有 `refresh_token`（password grant），先尝试刷新
3. 刷新失败或无 refresh_token，重新登录获取

### 输出格式化 (`src/output.rs`)

双模式输出策略：

- **Table 模式**（默认）: 使用 `tabled` crate 输出人类可读表格
- **JSON 模式**（`--json`）: 使用 `serde_json::to_string_pretty` 输出结构化 JSON

关键函数：

| 函数 | 说明 |
|------|------|
| `print_list(data, format)` | 输出列表数据 |
| `print_single(data, format)` | 输出单条数据 |
| `print_success(msg, format)` | 输出操作成功消息 |
| `print_json(data)` | 直接输出 JSON |

### 错误处理

- 使用 `anyhow` 进行错误传播
- `check_response()` 统一检查 HTTP 响应
- 解析 `X-FBox-Code` 响应头获取业务错误码
- HTTP 429 返回限流提示

## Agent-Native 设计

参考 [CLI-Anything](https://github.com/HKUDS/CLI-Anything) 的设计理念：

1. **结构化输出**: `--json` 全局标志，所有数据输出到 stdout，错误输出到 stderr
2. **自描述**: clap derive 自动生成 `--help`
3. **确定性行为**: 相同输入产生相同输出
4. **层级化命令**: `fboxcli <domain> <action> [args]`

## 扩展指南

### 添加新的 API 模块

1. 在 `src/models/` 添加数据模型（实现 `Serialize`, `Deserialize`, 可选 `Tabled`）
2. 在 `src/api/` 添加 API 调用函数
3. 在 `src/cli/` 添加 CLI 子命令（clap `Subcommand` derive）
4. 在 `src/cli/mod.rs` 的 `Commands` 枚举中注册新模块
5. 在 `src/main.rs` 的 match 中添加命令分发

### Option 字段的 Tabled 支持

`Option<T>` 字段需要 `display_with` 属性：

```rust
fn display_option<T: std::fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "-".to_string(),
    }
}

#[derive(Tabled)]
pub struct MyModel {
    #[tabled(rename = "Name", display_with = "display_option")]
    pub name: Option<String>,
}
```

对于 `Option<serde_json::Value>`，使用 `display_json_option` 变体。

## 依赖说明

| 依赖 | 版本 | 用途 |
|------|------|------|
| clap | 4 | CLI 参数解析（derive 模式） |
| reqwest | 0.13 | 异步 HTTP 客户端 |
| tokio | 1 | 异步运行时 |
| serde | 1 | 序列化框架 |
| serde_json | 1 | JSON 序列化 |
| tabled | 0.20 | 表格格式输出 |
| toml | 0.9 | TOML 配置文件解析 |
| anyhow | 1 | 错误处理 |
| dirs | 6 | 用户目录路径 |
| time | 0.3 | 时间处理 |
| uuid | 1 | UUID 生成（X-FBox-ClientId） |
