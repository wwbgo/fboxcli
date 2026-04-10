# 配置说明

## 配置文件

### 位置

| 文件 | 路径 | 说明 |
|------|------|------|
| 配置目录 | `~/.fboxcli/` | 所有配置文件的父目录 |
| 配置文件 | `~/.fboxcli/config.toml` | 服务器地址和凭证 |
| Token 缓存 | `~/.fboxcli/token.json` | 缓存的 access_token |

Windows 上 `~` 对应 `C:\Users\<用户名>\`。

### config.toml 格式

```toml
server = "https://openapi.fbox360.com"
grant_type = "developer"

# developer 模式需要配置以下字段
client_id = "your_client_id"
client_secret = "your_client_secret"

# user 模式需要配置以下字段（client_id/client_secret 使用内置值，无需配置）
username = "user@example.com"
password = "your_password"

# 自动生成，cli_ 前缀 + UUID，用于 dmon start/stop 的 X-FBox-ClientId 头
client_uid = "cli_xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
```

### token.json 格式

```json
{
  "access_token": "eyJ...",
  "refresh_token": "xxx...",
  "expires_at": 1700000000,
  "grant_type": "client_credentials"
}
```

| 字段 | 说明 |
|------|------|
| access_token | 当前有效的访问令牌 |
| refresh_token | 刷新令牌（仅 password grant） |
| expires_at | 过期时间（Unix 时间戳，秒） |
| grant_type | 获取此 Token 的认证方式 |

## 服务器地址

| 环境 | 地址 |
|------|------|
| 公有云（默认） | `https://openapi.fbox360.com` |
| 私有云 | 由用户配置的私有域名 |

## 认证方式

FBoxCLI 支持两种登录模式，通过 `--login-mode` 参数切换：

| 模式名 | OAuth2 grant_type | 说明 |
|--------|-------------------|------|
| `developer` | client_credentials | 开发者模式，需配置 client_id/client_secret |
| `user` | password | 用户模式，使用内置 client 凭证 |

### developer（开发者模式）

适用于服务端应用和自动化脚本。需要手动配置开发者 ID 和密钥。

```bash
fboxcli config set \
  --client-id YOUR_ID \
  --client-secret YOUR_SECRET

fboxcli auth login
```

特点：
- 需要手动配置 client_id 和 client_secret
- 不需要用户名密码
- 不返回 refresh_token
- access_token 过期后自动重新获取

### user（用户模式）

适用于代表特定用户操作的场景。使用内置的 client 凭证，用户只需提供用户名和密码。

```bash
fboxcli auth login -u user@example.com -p password
```

特点：
- 无需配置 client_id/client_secret（使用编译时内置值）
- 需要 FlexManager 用户名和密码
- 返回 refresh_token（30天有效）
- Token 过期时先尝试 refresh，失败再用密码重新登录

#### 编译时注入内置凭证

password 模式使用的 client 凭证通过编译时环境变量注入：

```bash
FBOX_BUILTIN_CLIENT_ID=your_real_id \
FBOX_BUILTIN_CLIENT_SECRET=your_real_secret \
cargo build --release
```

如果未设置环境变量，默认值为 `"builtin"`。

## Token 生命周期

```
登录 → 获取 access_token (有效期 ~2h)
          ↓
  每次 API 请求前检查
          ↓
  距过期 < 60秒 → 刷新/重新登录
          ↓
  缓存新 Token 到 token.json
```

CLI 自动处理 Token 的刷新和重新获取，用户无需手动管理。

## X-FBox-ClientId

`dmon start` 和 `dmon stop`（全局模式）需要 `X-FBox-ClientId` 请求头。

- 首次运行相关命令时自动生成 UUID v4
- 持久化到 `config.toml` 的 `client_uid` 字段
- 后续调用复用同一 UUID

## API 频率限制

| 维度 | 限制 |
|------|------|
| 每分钟 | 80 次 |
| 每小时 | 3000 次 |
| 每天 | 30000 次 |

超过限制时 API 返回 HTTP 429，CLI 会输出限流提示信息。
