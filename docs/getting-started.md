# 快速入门

## 前置条件

- Rust 工具链（rustc 1.85+，支持 edition 2024）
- FBox 平台开发者账号（获取 client_id 和 client_secret）

## 编译安装

```bash
cd d:/code/fboxcli
cargo build --release
```

编译完成后，可执行文件位于 `target/release/fboxcli.exe`（Windows）或 `target/release/fboxcli`（Linux/macOS）。

可选：将可执行文件添加到系统 PATH。

## 配置

### 方式一：开发者模式（developer）

使用开发者 client_id 和 client_secret 进行认证，适合服务端和自动化场景。

```bash
fboxcli config set \
  --server https://openapi.fbox360.com \
  --client-id YOUR_CLIENT_ID \
  --client-secret YOUR_CLIENT_SECRET

fboxcli auth login
```

### 方式二：用户模式（user）

使用 FlexManager 用户名和密码登录。此模式使用内置的 client 凭证，用户无需配置 client_id/client_secret，只需提供用户名和密码即可。

```bash
# 如需指定私有云地址（公有云可省略）
fboxcli config set --server https://openapi.fbox360.com

# 直接使用用户名密码登录
fboxcli auth login -u YOUR_USERNAME -p YOUR_PASSWORD
```

> 内置 client 凭证通过编译时环境变量 `FBOX_BUILTIN_CLIENT_ID` / `FBOX_BUILTIN_CLIENT_SECRET` 注入。

### 查看当前配置

```bash
fboxcli config show

# JSON 格式查看
fboxcli config show --json
```

### 重置配置

```bash
fboxcli config reset
```

## 验证连接

```bash
# 查看 Token 信息
fboxcli auth token

# 获取 FBox 设备列表
fboxcli box list
```

## JSON 输出模式

所有命令支持 `--json` 全局标志。加上后输出结构化 JSON，不加则输出人类可读的表格：

```bash
# 表格输出（默认，人类可读）
fboxcli box list

# JSON 输出（AI Agent 友好，可供脚本和 AI 解析）
fboxcli box list --json
```

## 常见使用场景

### 查看设备状态

```bash
# 列出所有设备
fboxcli box list

# 查看特定设备详情
fboxcli box get FB001234
fboxcli box info 12345
```

### 读写监控点数据

```bash
# 列出设备上的监控点
fboxcli dmon list 12345

# 读取多个监控点的实时值
fboxcli dmon get-value 12345 --ids 1001,1002,1003

# 按名称读取
fboxcli dmon get-value 12345 --names "温度,压力"

# 写入值
fboxcli dmon set-value 12345 --id 1001 --value 100
```

### 查询历史数据

```bash
# 查看历史记录条目
fboxcli history list 12345

# 查询原始历史数据（时间范围为毫秒时间戳）
fboxcli history query --ids 2001 --begin 1700000000000 --end 1700003600000

# 按小时粒度查询
fboxcli history query --ids 2001 --begin 1700000000000 --end 1700086400000 \
  --granularity 2 --limit 50
```

### 报警管理

```bash
# 查看报警条目
fboxcli alarm list 12345

# 查看报警历史
fboxcli alarm history 12345 --begin 1700000000000 --end 1700086400000

# 确认报警
fboxcli alarm confirm 3001
```

### AI Agent 集成

AI Agent 可以通过 `--json` 标志获取结构化数据：

```bash
# Agent 获取设备列表
fboxcli box list --json

# Agent 读取监控点
fboxcli dmon get-value 12345 --ids 1001 --json

# Agent 写入控制值
fboxcli dmon set-value 12345 --id 1001 --value 75.5

# Agent 查询报警状态
fboxcli alarm list 12345 --json
```

## 配置文件位置

| 文件 | 路径 | 说明 |
|------|------|------|
| 配置文件 | `~/.fboxcli/config.toml` | 服务器地址、凭证信息 |
| Token 缓存 | `~/.fboxcli/token.json` | access_token、refresh_token、过期时间 |

在 Windows 上，`~` 对应 `C:\Users\<用户名>\`。

## 下一步

- 阅读 [命令参考](commands.md) 了解全部命令和参数
- 阅读 [API 接口映射](api-reference.md) 了解 CLI 与 API 的对应关系
- 阅读 [配置说明](configuration.md) 了解 Token 管理和认证细节
