# API 接口映射

本文档列出 FBoxCLI 命令与 FBox 平台 API 接口的对应关系。

**基础信息**:
- 公有云地址: `https://openapi.fbox360.com`
- 认证方式: `Authorization: Bearer {access_token}`
- 频率限制: 80次/分钟, 3000次/小时, 30000次/天

---

## 认证接口

| CLI 命令 | HTTP 方法 | API 路径 | Content-Type |
|---------|----------|----------|-------------|
| `auth login` (client_credentials) | POST | `/idserver/core/connect/token` | form-urlencoded |
| `auth login -u -p` (password) | POST | `/idserver/core/connect/token` | form-urlencoded |
| Token 自动刷新 | POST | `/idserver/core/connect/token` | form-urlencoded |

### client_credentials 参数

| 参数 | 值 |
|------|-----|
| scope | `fbox` |
| client_id | 开发者 ID |
| client_secret | 密钥原文 |
| grant_type | `client_credentials` |

### password 参数

| 参数 | 值 |
|------|-----|
| scope | `openid offline_access fbox email profile` |
| client_id | 开发者 ID |
| client_secret | 密钥原文 |
| grant_type | `password` |
| username | FlexManager 账号 |
| password | FlexManager 密码 |

### Token 有效期

- access_token: 约 2 小时
- refresh_token: 30 天（仅 password grant 返回）

---

## FBox 设备接口

| CLI 命令 | HTTP 方法 | API 路径 |
|---------|----------|----------|
| `box list` | GET | `/api/client/box/grouped` |
| `box get <boxNo>` | GET | `/api/client/box/reg/boxno/{boxNo}` |
| `box add` | POST | `/api/client/box/reg` |
| `box rename <boxId> <alias>` | POST | `/api/client/v2/box/{boxId}/alias` |
| `box delete <boxId>` | DELETE | `/api/client/v2/box/{boxId}` |
| `box info <boxId>` | GET | `/api/v2/box/{boxId}/info` |
| `box memo <boxId> <content>` | POST | `/api/client/box/{boxId}/memo` |

---

## FBox 分组接口

| CLI 命令 | HTTP 方法 | API 路径 |
|---------|----------|----------|
| `group add <name>` | PUT | `/api/client/group` |
| `group rename <groupId> <name>` | POST | `/api/client/v2/box/group/mgt` |
| `group delete <groupId>` | DELETE | `/api/client/group/{groupId}` |

---

## 数据监控点接口

| CLI 命令 | HTTP 方法 | API 路径 | 备注 |
|---------|----------|----------|------|
| `dmon list <boxId>` | GET | `/api/v2/box/{boxId}/dmon/grouped` | |
| `dmon get-value <boxId>` | POST | `/api/v2/box/{boxId}/dmon/value/get` | |
| `dmon set-value <boxId>` | POST | `/api/v2/box/{boxId}/dmon/value` | Body 为数组 |
| `dmon start <boxId>` | POST | `/api/box/{boxId}/dmon/start` | 需 X-FBox-ClientId |
| `dmon start --uid <uid>` | POST | `/api/dmon/{uid}/start` | |
| `dmon stop <boxId>` | POST | `/api/box/{boxId}/dmon/stop` | 需 X-FBox-ClientId |
| `dmon stop --uid <uid>` | POST | `/api/dmon/{uid}/stop` | |
| `dmon groups <boxId>` | GET | `/api/v2/box/{boxId}/dmon/groups` | |
| `dmon delete <boxId>` | POST | `/api/v2/box/{boxId}/dmon/del` | |

### X-FBox-ClientId

`dmon start` 和 `dmon stop`（全局模式）需要 `X-FBox-ClientId` 请求头，值为 UUID。CLI 首次运行时自动生成并持久化到配置文件。

### 监控点值状态码

| 状态码 | 含义 |
|-------|------|
| 0 | 正常 |
| 1 | 无数据 |
| 2 | 超时 |
| 3 | 错误 |
| 4 | Socket 异常 |
| 5 | FDS 错误 |
| 16 | 未完成 |

---

## 历史数据接口

| CLI 命令 | HTTP 方法 | API 路径 |
|---------|----------|----------|
| `history query` | POST | `/hs/v2/hdata/get` |
| `history list <boxId>` | GET | `/api/v2/box/{boxId}/hdataitems` |
| `history delete <boxId>` | POST | `/api/v2/box/{boxId}/hdataitems/del` |

### 历史数据粒度

| 值 | 含义 |
|----|------|
| 0 | 原始数据 |
| 1 | 分钟 |
| 2 | 小时 |
| 3 | 天 |

---

## 报警接口

| CLI 命令 | HTTP 方法 | API 路径 |
|---------|----------|----------|
| `alarm list <boxId>` | GET | `/api/v2/box/{boxId}/alarm` |
| `alarm history <boxId>` | GET | `/api/v2/box/{boxId}/alarm/data` |
| `alarm confirm <uid>` | POST | `/api/alarm/{uid}/confirm` |
| `alarm groups <boxId>` | POST | `/api/v2/alarm/group/get` |
| `alarm add-group` | PUT | `/api/v2/alarm/group` |
| `alarm delete-group` | POST | `/api/v2/alarm/group/del` |
| `alarm delete <boxId>` | POST | `/api/v2/box/{boxId}/alarm/del` |

---

## 联系人接口

| CLI 命令 | HTTP 方法 | API 路径 |
|---------|----------|----------|
| `contact list` | GET | `/api/v2/contacts` |
| `contact get <uid>` | GET | `/api/v2/contact/{uid}` |
| `contact add` | PUT | `/api/v2/contact` |
| `contact update <uid>` | POST | `/api/v2/contact` |
| `contact delete <uid>` | DELETE | `/api/v2/contact/{uid}` |

### 通知类型

| 值 | 含义 |
|----|------|
| 0 | 无 |
| 1 | 短信 |
| 2 | 语音 |
| 3 | 短信 + 语音 |

---

## 设备接口

| CLI 命令 | HTTP 方法 | API 路径 |
|---------|----------|----------|
| `device list <boxId>` | GET | `/api/v2/box/{boxId}/device` |
| `device drivers [boxType]` | GET | `/api/device/spec/{boxType}` |
| `device registers <deviceId>` | GET | `/api/device/{deviceId}/spec` |

### 盒子类型

| 值 | 含义 |
|----|------|
| 0 | 标准 |
| 1 | Mini |
| 2 | Lite |
| 3 | VPN |

---

## 统一写组接口

| CLI 命令 | HTTP 方法 | API 路径 |
|---------|----------|----------|
| `control list` | GET | `/api/v2/control/groups` |
| `control get <groupId>` | GET | `/api/v2/control/group/{groupId}` |
| `control add` | PUT | `/api/v2/control/group` |
| `control delete` | POST | `/api/v2/control/group/del` |
| `control write` | POST | `/api/v2/control/group/write/value` |

---

## 地理位置接口

| CLI 命令 | HTTP 方法 | API 路径 |
|---------|----------|----------|
| `location <ids>` | POST | `/api/client/v2/box/location` |
