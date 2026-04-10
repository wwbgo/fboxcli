#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# fboxcli 多平台编译脚本
#   - win32-x64:    本地 cargo 编译
#   - darwin-arm64: SSH 到 Mac 编译
#   - linux-x64:    SSH 到 Linux 编译
#
# 用法:
#   ./scripts/build.sh                         # 编译所有平台
#   ./scripts/build.sh win32-x64               # 只编译 Windows
#   ./scripts/build.sh darwin-arm64 linux-x64   # 编译多个
#   ./scripts/build.sh --skip-copy all          # 只编译不拷贝
# ============================================================

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
NPM_DIR="$ROOT_DIR/npm"
SCRIPTS_DIR="$(cd "$(dirname "$0")" && pwd)"

SKIP_COPY=false

# ── 加载 .env ──
ENV_FILE="$SCRIPTS_DIR/.env"
if [ -f "$ENV_FILE" ]; then
  set -a
  source "$ENV_FILE"
  set +a
else
  echo "ERROR: Missing .env file: $ENV_FILE" >&2
  exit 1
fi

# ── 平台定义 ──
declare -A TARGETS=(
  ["win32-x64"]="x86_64-pc-windows-msvc:fboxcli.exe:local::"
  ["darwin-arm64"]="aarch64-apple-darwin:fboxcli:remote:${DARWIN_ARM64_HOST}:${DARWIN_ARM64_REMOTE_DIR}"
  ["linux-x64"]="x86_64-unknown-linux-gnu:fboxcli:remote:${LINUX_X64_HOST}:${LINUX_X64_REMOTE_DIR}"
)

# ── 解析参数 ──
BUILD_LIST=()
for arg in "$@"; do
  case "$arg" in
    --skip-copy) SKIP_COPY=true ;;
    all)         BUILD_LIST=("win32-x64" "darwin-arm64" "linux-x64") ;;
    *)           BUILD_LIST+=("$arg") ;;
  esac
done
# 默认编译全部
if [ ${#BUILD_LIST[@]} -eq 0 ]; then
  BUILD_LIST=("win32-x64" "darwin-arm64" "linux-x64")
fi

# ── 辅助函数 ──

invoke_remote() {
  local ssh_host="$1"
  local cmd="$2"
  echo "    [ssh] $cmd"
  ssh -o ConnectTimeout=10 -o StrictHostKeyChecking=no "$ssh_host" "$cmd"
}

sync_source() {
  local ssh_host="$1"
  local remote_dir="$2"
  echo "    Syncing source to ${ssh_host}:${remote_dir} ..."

  # 创建目录并清理旧源码（保留 target 缓存）
  invoke_remote "$ssh_host" "mkdir -p $remote_dir && cd $remote_dir && ls | grep -v '^target$' | xargs rm -rf 2>/dev/null; true"

  # 打包源码（排除 target、node_modules、.git）
  local temp_tar="/tmp/fboxcli-src.tar.gz"
  tar czf "$temp_tar" -C "$ROOT_DIR" \
    --exclude='target' \
    --exclude='node_modules' \
    --exclude='.git' \
    .

  # 上传并解压
  scp -o StrictHostKeyChecking=no "$temp_tar" "${ssh_host}:${remote_dir}/src.tar.gz"
  invoke_remote "$ssh_host" "cd $remote_dir && tar xzf src.tar.gz && rm src.tar.gz"

  rm -f "$temp_tar"
  echo "    Sync complete"
}

file_size_mb() {
  local file="$1"
  if [[ "$(uname)" == "Darwin" ]]; then
    stat -f '%z' "$file" | awk '{printf "%.1f", $1/1048576}'
  else
    stat --printf='%s' "$file" | awk '{printf "%.1f", $1/1048576}'
  fi
}

# ── 收集需要同步的远程主机（去重） ──

declare -A REMOTE_HOSTS
for suffix in "${BUILD_LIST[@]}"; do
  IFS=':' read -r _ _ location ssh_host remote_dir <<< "${TARGETS[$suffix]}"
  if [[ "$location" == "remote" && -z "${REMOTE_HOSTS[$ssh_host]:-}" ]]; then
    REMOTE_HOSTS["$ssh_host"]="$remote_dir"
  fi
done

for host in "${!REMOTE_HOSTS[@]}"; do
  echo "==> Syncing source to $host"
  sync_source "$host" "${REMOTE_HOSTS[$host]}"
done

# ── 编译 ──

for suffix in "${BUILD_LIST[@]}"; do
  IFS=':' read -r rust_target bin_name location ssh_host remote_dir <<< "${TARGETS[$suffix]}"

  if [[ "$location" == "local" ]]; then
    echo "==> [$suffix] Building locally for $rust_target"
    cargo build --release --target "$rust_target"

    src="$ROOT_DIR/target/$rust_target/release/$bin_name"
    size=$(file_size_mb "$src")
    echo "    Output: $src ($size MB)"

    if [[ "$SKIP_COPY" == "false" ]]; then
      dest="$NPM_DIR/fboxcli-${suffix}/$bin_name"
      cp "$src" "$dest"
      echo "    Copied -> $dest"
    fi
  else
    echo "==> [$suffix] Building on $ssh_host for $rust_target"
    invoke_remote "$ssh_host" "cd $remote_dir && cargo build --release --target $rust_target"

    remote_bin="$remote_dir/target/$rust_target/release/$bin_name"
    invoke_remote "$ssh_host" "ls -lh $remote_bin"

    if [[ "$SKIP_COPY" == "false" ]]; then
      dest="$NPM_DIR/fboxcli-${suffix}/$bin_name"
      echo "    Downloading binary ..."
      scp -o StrictHostKeyChecking=no "${ssh_host}:${remote_bin}" "$dest"
      size=$(file_size_mb "$dest")
      echo "    Copied -> $dest ($size MB)"
    fi
  fi
done

# ── 汇总 ──

echo ""
echo "==> Build complete!"
for suffix in "${BUILD_LIST[@]}"; do
  IFS=':' read -r rust_target bin_name location _ _ <<< "${TARGETS[$suffix]}"
  if [[ "$location" == "local" ]]; then
    bin="$ROOT_DIR/target/$rust_target/release/$bin_name"
  else
    bin="$NPM_DIR/fboxcli-${suffix}/$bin_name"
  fi
  if [ -f "$bin" ]; then
    size=$(file_size_mb "$bin")
    printf "    %-15s %s MB\n" "$suffix" "$size"
  fi
done
