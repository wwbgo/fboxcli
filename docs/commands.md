# 命令参考

## 全局选项

```
fboxcli [OPTIONS] <COMMAND>

Options:
  --json       以 JSON 格式输出（AI Agent 模式）
  -h, --help   显示帮助信息
  -V, --version 显示版本号
```

所有命令均支持 `--json` 全局标志。加上 `--json` 后输出结构化 JSON，适合 AI Agent 或脚本解析；不加则输出人类可读的表格。

---

## config - 配置管理

### config set

设置配置项。可以一次设置一个或多个配置。

```bash
fboxcli config set [OPTIONS]

Options:
  --server <SERVER>                服务器地址
  --client-id <CLIENT_ID>          开发者 Client ID（developer 模式使用）
  --client-secret <CLIENT_SECRET>  开发者 Client Secret（developer 模式使用）
  --login-mode <MODE>              登录模式："developer"（开发者）或 "user"（用户）
```

**示例：**

```bash
# 设置公有云服务器和开发者凭证
fboxcli config set --server https://openapi.fbox360.com \
  --client-id myClientId \
  --client-secret myClientSecret

# 切换为用户模式（使用内置 client 凭证，只需用户名密码）
fboxcli config set --login-mode user

# 设置私有云地址
fboxcli config set --server https://my-private-fbox.com
```

### config show

显示当前配置。

```bash
fboxcli config show
```

**示例：**

```bash
# 查看当前配置（表格格式）
fboxcli config show

# 以 JSON 格式查看配置
fboxcli config show --json
```

### config reset

重置配置为默认值并清除缓存的 Token。

```bash
fboxcli config reset
```

**示例：**

```bash
fboxcli config reset
```

---

## auth - 认证管理

### auth login

登录并获取 access_token。不带参数时使用 client_credentials 模式，带 `-u/-p` 时使用 password 模式。

```bash
fboxcli auth login [OPTIONS]

Options:
  -u, --username <USERNAME>  FlexManager 用户名（password grant 模式）
  -p, --password <PASSWORD>  FlexManager 密码（password grant 模式）
```

**示例：**

```bash
# client_credentials 模式登录（需先通过 config set 配置 client_id 和 client_secret）
fboxcli auth login

# password 模式登录（使用内置 client 凭证，无需配置 client_id/client_secret）
fboxcli auth login -u user@example.com -p myPassword123

# 登录并以 JSON 返回 Token
fboxcli auth login --json
```

### auth token

显示当前缓存的 Token 信息（access_token、过期时间等）。

```bash
fboxcli auth token
```

**示例：**

```bash
# 查看 Token 详情
fboxcli auth token

# 以 JSON 格式获取 Token（可用于脚本提取 access_token）
fboxcli auth token --json
```

### auth logout

清除缓存的 Token。

```bash
fboxcli auth logout
```

**示例：**

```bash
fboxcli auth logout
```

---

## box - FBox 设备管理

### box list

列出所有 FBox 设备（按分组），显示 ID、别名、序列号、连接状态。

```bash
fboxcli box list
```

**示例：**

```bash
# 以表格形式列出所有设备
fboxcli box list

# 以 JSON 格式列出（包含完整分组结构）
fboxcli box list --json
```

### box get

按序列号获取单个 FBox 的信息。

```bash
fboxcli box get <BOX_NO>

Arguments:
  <BOX_NO>  FBox 序列号
```

**示例：**

```bash
# 获取序列号为 FB001234 的设备信息
fboxcli box get FB001234

# JSON 格式输出
fboxcli box get FB001234 --json
```

### box add

添加新的 FBox 到账号下。

```bash
fboxcli box add <BOX_NO> <PASSWORD> [OPTIONS]

Arguments:
  <BOX_NO>    FBox 序列号
  <PASSWORD>  FBox 密码

Options:
  --alias <ALIAS>  设备别名
  --group <GROUP>  分组名称
```

**示例：**

```bash
# 最简添加
fboxcli box add FB001234 fboxpassword

# 添加时指定别名和分组
fboxcli box add FB001234 fboxpassword --alias "车间1号" --group "生产车间"
```

### box rename

重命名 FBox 设备。

```bash
fboxcli box rename <BOX_ID> <ALIAS>

Arguments:
  <BOX_ID>  FBox ID（数字）
  <ALIAS>   新别名
```

**示例：**

```bash
fboxcli box rename 12345 "新名称"
```

### box delete

删除 FBox 设备。

```bash
fboxcli box delete <BOX_ID>

Arguments:
  <BOX_ID>  FBox ID（数字）
```

**示例：**

```bash
fboxcli box delete 12345
```

### box info

获取 FBox 的详细配置信息（IP、DNS、刷新间隔、固件版本等）。

```bash
fboxcli box info <BOX_ID>

Arguments:
  <BOX_ID>  FBox ID（数字）
```

**示例：**

```bash
fboxcli box info 12345

# JSON 格式查看完整信息
fboxcli box info 12345 --json
```

### box memo

设置 FBox 的备注信息。

```bash
fboxcli box memo <BOX_ID> <CONTENT>

Arguments:
  <BOX_ID>   FBox ID（数字）
  <CONTENT>  备注内容
```

**示例：**

```bash
fboxcli box memo 12345 "位于A栋3楼配电房"
```

---

## group - FBox 分组管理

### group add

新增 FBox 分组。

```bash
fboxcli group add <NAME> [OPTIONS]

Arguments:
  <NAME>  分组名称

Options:
  --parent <PARENT>  父分组 ID
```

**示例：**

```bash
# 创建顶级分组
fboxcli group add "生产车间"

# 创建子分组
fboxcli group add "1号产线" --parent 100
```

### group rename

重命名分组。

```bash
fboxcli group rename <GROUP_ID> <NAME>

Arguments:
  <GROUP_ID>  分组 ID（数字）
  <NAME>      新名称
```

**示例：**

```bash
fboxcli group rename 100 "生产车间-新"
```

### group delete

删除分组。

```bash
fboxcli group delete <GROUP_ID>

Arguments:
  <GROUP_ID>  分组 ID（数字）
```

**示例：**

```bash
fboxcli group delete 100
```

---

## dmon - 数据监控点管理

### dmon list

列出 FBox 的所有监控点（按分组），显示 ID、名称、数据类型、读写权限等。

```bash
fboxcli dmon list <BOX_ID>

Arguments:
  <BOX_ID>  FBox ID（数字）
```

**示例：**

```bash
# 列出设备 12345 的所有监控点
fboxcli dmon list 12345

# JSON 格式（包含分组结构）
fboxcli dmon list 12345 --json
```

### dmon get-value

获取监控点的实时值。可按 ID 或名称查询。

```bash
fboxcli dmon get-value <BOX_ID> [OPTIONS]

Arguments:
  <BOX_ID>  FBox ID（数字）

Options:
  --ids <IDS>          逗号分隔的监控点 ID
  --names <NAMES>      逗号分隔的监控点名称
  --timeout <TIMEOUT>  超时时间，毫秒 [默认: 5000]
```

**示例：**

```bash
# 按 ID 获取多个监控点的值
fboxcli dmon get-value 12345 --ids 1001,1002,1003

# 按名称获取
fboxcli dmon get-value 12345 --names "温度,压力,流量"

# 设置超时并以 JSON 输出
fboxcli dmon get-value 12345 --ids 1001 --timeout 10000 --json
```

### dmon set-value

向监控点写入值。可通过 ID 或名称指定监控点。

```bash
fboxcli dmon set-value <BOX_ID> --value <VALUE> [OPTIONS]

Arguments:
  <BOX_ID>  FBox ID（数字）

Options:
  --id <ID>        监控点 ID
  --name <NAME>    监控点名称（可替代 ID）
  --value <VALUE>  要写入的值（必填）
```

**示例：**

```bash
# 按 ID 写值
fboxcli dmon set-value 12345 --id 1001 --value 100

# 按名称写值
fboxcli dmon set-value 12345 --name "目标温度" --value 75.5
```

### dmon start

开启数据推送。省略 `--uid` 时开启 FBox 下所有监控点的推送。

```bash
fboxcli dmon start <BOX_ID> [OPTIONS]

Arguments:
  <BOX_ID>  FBox ID（数字）

Options:
  --uid <UID>  指定单个监控点 UID（省略则开启全部）
```

**示例：**

```bash
# 开启所有监控点推送
fboxcli dmon start 12345

# 只开启单个监控点
fboxcli dmon start 12345 --uid 1001
```

### dmon stop

停止数据推送。省略 `--uid` 时停止 FBox 下所有监控点的推送。

```bash
fboxcli dmon stop <BOX_ID> [OPTIONS]

Arguments:
  <BOX_ID>  FBox ID（数字）

Options:
  --uid <UID>  指定单个监控点 UID（省略则停止全部）
```

**示例：**

```bash
# 停止所有监控点推送
fboxcli dmon stop 12345

# 只停止单个监控点
fboxcli dmon stop 12345 --uid 1001
```

### dmon groups

列出监控点分组。

```bash
fboxcli dmon groups <BOX_ID>

Arguments:
  <BOX_ID>  FBox ID（数字）
```

**示例：**

```bash
fboxcli dmon groups 12345
fboxcli dmon groups 12345 --json
```

### dmon delete

删除监控点。

```bash
fboxcli dmon delete <BOX_ID> --ids <IDS>

Arguments:
  <BOX_ID>  FBox ID（数字）

Options:
  --ids <IDS>  逗号分隔的监控点 ID（必填）
```

**示例：**

```bash
# 删除单个监控点
fboxcli dmon delete 12345 --ids 1001

# 批量删除
fboxcli dmon delete 12345 --ids 1001,1002,1003
```

---

## history - 历史数据管理

### history query

查询历史记录数据。支持不同粒度和时间范围。

```bash
fboxcli history query --ids <IDS> --begin <BEGIN> --end <END> [OPTIONS]

Options:
  --ids <IDS>                  逗号分隔的通道 ID（必填）
  --begin <BEGIN>              开始时间，毫秒时间戳（必填）
  --end <END>                  结束时间，毫秒时间戳（必填）
  --granularity <GRANULARITY>  粒度：0=原始, 1=分钟, 2=小时, 3=天 [默认: 0]
  --limit <LIMIT>              最大条数，最大 1000 [默认: 100]
  --tz <TZ>                    时区字符串
```

**示例：**

```bash
# 查询原始数据（最近1小时，当前时间戳需自行计算）
fboxcli history query --ids 2001,2002 --begin 1700000000000 --end 1700003600000

# 按小时粒度查询，限制50条
fboxcli history query --ids 2001 --begin 1700000000000 --end 1700086400000 \
  --granularity 2 --limit 50

# JSON 格式输出
fboxcli history query --ids 2001 --begin 1700000000000 --end 1700003600000 --json

# 指定时区
fboxcli history query --ids 2001 --begin 1700000000000 --end 1700003600000 \
  --tz "Asia/Shanghai"
```

### history list

列出 FBox 的历史记录条目配置。

```bash
fboxcli history list <BOX_ID>

Arguments:
  <BOX_ID>  FBox ID（数字）
```

**示例：**

```bash
fboxcli history list 12345
fboxcli history list 12345 --json
```

### history delete

删除历史记录条目。

```bash
fboxcli history delete <BOX_ID> --ids <IDS>

Arguments:
  <BOX_ID>  FBox ID（数字）

Options:
  --ids <IDS>  逗号分隔的条目 ID（必填）
```

**示例：**

```bash
fboxcli history delete 12345 --ids 2001,2002
```

---

## alarm - 报警管理

### alarm list

列出 FBox 的报警条目配置。

```bash
fboxcli alarm list <BOX_ID>

Arguments:
  <BOX_ID>  FBox ID（数字）
```

**示例：**

```bash
fboxcli alarm list 12345
fboxcli alarm list 12345 --json
```

### alarm history

获取报警历史记录。

```bash
fboxcli alarm history <BOX_ID> --begin <BEGIN> --end <END> [OPTIONS]

Arguments:
  <BOX_ID>  FBox ID（数字）

Options:
  --begin <BEGIN>  开始时间，毫秒时间戳（必填）
  --end <END>      结束时间，毫秒时间戳（必填）
  --limit <LIMIT>  最大条数
```

**示例：**

```bash
# 查询最近24小时的报警记录
fboxcli alarm history 12345 --begin 1700000000000 --end 1700086400000

# 限制返回50条
fboxcli alarm history 12345 --begin 1700000000000 --end 1700086400000 --limit 50

# JSON 格式
fboxcli alarm history 12345 --begin 1700000000000 --end 1700086400000 --json
```

### alarm confirm

确认（应答）一条报警。

```bash
fboxcli alarm confirm <UID>

Arguments:
  <UID>  报警条目 UID
```

**示例：**

```bash
fboxcli alarm confirm 3001
```

### alarm groups

列出 FBox 的报警分组及其关联联系人。

```bash
fboxcli alarm groups <BOX_ID>

Arguments:
  <BOX_ID>  FBox ID（数字）
```

**示例：**

```bash
fboxcli alarm groups 12345
fboxcli alarm groups 12345 --json
```

### alarm add-group

添加报警分组，可关联联系人。

```bash
fboxcli alarm add-group <BOX_ID> <NAME> [OPTIONS]

Arguments:
  <BOX_ID>  FBox ID（数字）
  <NAME>    分组名称（20字以内）

Options:
  --contacts <CONTACTS>  逗号分隔的联系人 UID
```

**示例：**

```bash
# 创建不关联联系人的分组
fboxcli alarm add-group 12345 "紧急报警"

# 创建并关联联系人
fboxcli alarm add-group 12345 "设备报警" --contacts 4001,4002
```

### alarm delete-group

删除报警分组。

```bash
fboxcli alarm delete-group <BOX_ID> <UID>

Arguments:
  <BOX_ID>  FBox ID（数字）
  <UID>     报警分组 UID
```

**示例：**

```bash
fboxcli alarm delete-group 12345 3001
```

### alarm delete

删除报警条目。

```bash
fboxcli alarm delete <BOX_ID> --ids <IDS>

Arguments:
  <BOX_ID>  FBox ID（数字）

Options:
  --ids <IDS>  逗号分隔的报警条目 ID（必填）
```

**示例：**

```bash
fboxcli alarm delete 12345 --ids 3001,3002
```

---

## contact - 联系人管理

### contact list

列出所有报警联系人。

```bash
fboxcli contact list
```

**示例：**

```bash
fboxcli contact list
fboxcli contact list --json
```

### contact get

获取单个联系人详细信息。

```bash
fboxcli contact get <UID>

Arguments:
  <UID>  联系人 UID
```

**示例：**

```bash
fboxcli contact get 4001
fboxcli contact get 4001 --json
```

### contact add

添加联系人。

```bash
fboxcli contact add <NAME> [OPTIONS]

Arguments:
  <NAME>  联系人名称

Options:
  --email <EMAIL>              邮箱地址
  --phone <PHONE>              手机号
  --notice-type <NOTICE_TYPE>  通知类型：0=无, 1=短信, 2=语音, 3=短信+语音 [默认: 0]
```

**示例：**

```bash
# 添加只有邮箱的联系人
fboxcli contact add "张三" --email zhangsan@example.com

# 添加手机联系人并开启短信通知
fboxcli contact add "李四" --phone 13800138000 --notice-type 1

# 添加完整联系人（邮箱+手机+短信和语音通知）
fboxcli contact add "王五" --email wangwu@example.com --phone 13900139000 --notice-type 3
```

### contact update

更新联系人信息。只需传入要修改的字段。

```bash
fboxcli contact update <UID> [OPTIONS]

Arguments:
  <UID>  联系人 UID

Options:
  --name <NAME>    新名称
  --email <EMAIL>  新邮箱
  --phone <PHONE>  新手机号
```

**示例：**

```bash
# 修改名称
fboxcli contact update 4001 --name "张三丰"

# 修改邮箱和手机
fboxcli contact update 4001 --email new@example.com --phone 13700137000
```

### contact delete

删除联系人。

```bash
fboxcli contact delete <UID>

Arguments:
  <UID>  联系人 UID
```

**示例：**

```bash
fboxcli contact delete 4001
```

---

## device - 设备管理

### device list

列出 FBox 连接的 PLC/设备列表。

```bash
fboxcli device list <BOX_ID>

Arguments:
  <BOX_ID>  FBox ID（数字）
```

**示例：**

```bash
fboxcli device list 12345
fboxcli device list 12345 --json
```

### device drivers

列出服务器支持的驱动列表。

```bash
fboxcli device drivers [BOX_TYPE]

Arguments:
  [BOX_TYPE]  盒子类型：0=标准, 1=Mini, 2=Lite, 3=VPN [默认: 0]
```

**示例：**

```bash
# 查看标准型支持的驱动
fboxcli device drivers

# 查看 Mini 型支持的驱动
fboxcli device drivers 1

# JSON 格式（包含寄存器信息）
fboxcli device drivers 0 --json
```

### device registers

获取指定设备的寄存器详细信息。

```bash
fboxcli device registers <DEVICE_ID>

Arguments:
  <DEVICE_ID>  设备 ID
```

**示例：**

```bash
fboxcli device registers 5001
fboxcli device registers 5001 --json
```

---

## control - 统一写组管理

### control list

列出所有统一写组。

```bash
fboxcli control list
```

**示例：**

```bash
fboxcli control list
fboxcli control list --json
```

### control get

获取单个统一写组的详细信息。

```bash
fboxcli control get <GROUP_ID>

Arguments:
  <GROUP_ID>  统一写组 ID
```

**示例：**

```bash
fboxcli control get 6001
fboxcli control get 6001 --json
```

### control add

通过 JSON 数据添加统一写组。

```bash
fboxcli control add <JSON_DATA>

Arguments:
  <JSON_DATA>  JSON 格式的分组定义
```

**示例：**

```bash
fboxcli control add '{"name":"温度控制组","type":7,"controlOptions":[{"bn":"FB001234","dgn":"默认分组","dn":"目标温度","alias":"车间1号"}]}'
```

### control delete

删除统一写组。

```bash
fboxcli control delete --ids <IDS>

Options:
  --ids <IDS>  逗号分隔的写组 ID（必填）
```

**示例：**

```bash
fboxcli control delete --ids 6001
fboxcli control delete --ids 6001,6002
```

### control write

向统一写组写入值。通过 UID 或名称指定目标分组。

```bash
fboxcli control write --value <VALUE> [OPTIONS]

Options:
  --uid <UID>      写组 UID
  --name <NAME>    写组名称（可替代 UID）
  --value <VALUE>  要写入的值（必填）
```

**示例：**

```bash
# 按 UID 写值
fboxcli control write --uid 6001 --value 100

# 按名称写值
fboxcli control write --name "温度控制组" --value 75.5

# 写入字符串值
fboxcli control write --uid 6001 --value '"hello"'

# 写入布尔值
fboxcli control write --uid 6001 --value true
```

---

## location - 地理位置

获取一个或多个 FBox 设备的地理位置信息（经纬度、地址）。

```bash
fboxcli location <IDS>

Arguments:
  <IDS>  逗号分隔的 FBox ID
```

**示例：**

```bash
# 查询单个设备位置
fboxcli location 12345

# 查询多个设备位置
fboxcli location 12345,12346,12347

# JSON 格式输出
fboxcli location 12345,12346 --json
```
