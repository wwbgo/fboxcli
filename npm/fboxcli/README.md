# FBoxCLI

FBox IoT 平台的 Agent-Native 命令行工具。

## 安装

```bash
npm install -g @flexem/fboxcli
```

或通过 npx 直接使用：

```bash
npx @flexem/fboxcli --help
```

## 快速开始

```bash
# 用户模式登录
fboxcli auth login -u YOUR_USERNAME -p YOUR_PASSWORD

# 查看 FBox 列表
fboxcli box list

# JSON 输出（AI Agent 模式）
fboxcli box list --json
```

## 特性

- **Agent-Native**: 全局 `--json` 标志输出结构化 JSON，方便 AI Agent 解析
- **自描述**: 每个命令和子命令均有 `--help` 说明
- **Token 自动管理**: 缓存 Token 并在过期前自动刷新
- **完整 API 覆盖**: 11 个命令模块，覆盖 FBox 平台全部 API

## 支持平台

| 平台 | 架构 | npm 包 |
|------|------|--------|
| Windows | x64 | `@flexem/fboxcli-win32-x64` |
| macOS | ARM64 | `@flexem/fboxcli-darwin-arm64` |
| Linux | x64 | `@flexem/fboxcli-linux-x64` |

## 文档

详细文档请参考：https://github.com/flexem/fboxcli

## 许可证

MIT
