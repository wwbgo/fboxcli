#!/usr/bin/env bash
set -euo pipefail

# ============================================================
# fboxcli npm 发布脚本
# 调用 build.ps1 编译所有平台，同步版本号，发布到 npm
# ============================================================

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
NPM_DIR="$ROOT_DIR/npm"
SCRIPTS_DIR="$(cd "$(dirname "$0")" && pwd)"

# 平台包列表
PLATFORM_SUFFIXES=("win32-x64" "darwin-arm64" "linux-x64")
declare -A BIN_NAMES=(
  ["win32-x64"]="fboxcli.exe"
  ["darwin-arm64"]="fboxcli"
  ["linux-x64"]="fboxcli"
)

# 从 Cargo.toml 提取版本号
VERSION=$(grep '^version' "$ROOT_DIR/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
echo "==> Version: $VERSION"

NPM_TAG="latest"
DRY_RUN="${DRY_RUN:-}"  # 设置 DRY_RUN=--dry-run 进行测试

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

NPM_REGISTRY="${NPM_REGISTRY:-https://registry.npmjs.org}"

# Step 1: 编译
if [[ "${1:-}" != "--skip-build" ]]; then
  echo "==> Building all platforms ..."
  bash "$SCRIPTS_DIR/build.sh" all
else
  echo "==> Skipping build (--skip-build)"
  for suffix in "${PLATFORM_SUFFIXES[@]}"; do
    bin_name="${BIN_NAMES[$suffix]}"
    if [ ! -f "$NPM_DIR/fboxcli-${suffix}/$bin_name" ]; then
      echo "ERROR: Missing binary $NPM_DIR/fboxcli-${suffix}/$bin_name"
      exit 1
    fi
  done
fi

# Step 2: 同步版本号到所有 package.json
echo "==> Updating versions to $VERSION"

for pkg_dir in "$NPM_DIR"/*/; do
  if [ -f "$pkg_dir/package.json" ]; then
    node -e "
      const fs = require('fs');
      const path = '$pkg_dir/package.json';
      const pkg = JSON.parse(fs.readFileSync(path, 'utf8'));
      pkg.version = '$VERSION';
      fs.writeFileSync(path, JSON.stringify(pkg, null, 2) + '\n');
    "
  fi
done

# 更新主包的 optionalDependencies 版本
node -e "
  const fs = require('fs');
  const path = '$NPM_DIR/fboxcli/package.json';
  const pkg = JSON.parse(fs.readFileSync(path, 'utf8'));
  for (const key of Object.keys(pkg.optionalDependencies || {})) {
    pkg.optionalDependencies[key] = '$VERSION';
  }
  fs.writeFileSync(path, JSON.stringify(pkg, null, 2) + '\n');
"

# Step 3: 发布平台包（先于主包）
for suffix in "${PLATFORM_SUFFIXES[@]}"; do
  echo "==> Publishing @flexem/fboxcli-${suffix}@${VERSION}"
  (cd "$NPM_DIR/fboxcli-${suffix}" && npm publish --access public --tag "$NPM_TAG" --registry "$NPM_REGISTRY" "--//registry.npmjs.org/:_authToken=$NPM_TOKEN" $DRY_RUN)
done

# Step 4: 发布主包
echo "==> Publishing @flexem/fboxcli@${VERSION}"
(cd "$NPM_DIR/fboxcli" && npm publish --access public --tag "$NPM_TAG" --registry "$NPM_REGISTRY" "--//registry.npmjs.org/:_authToken=$NPM_TOKEN" $DRY_RUN)

# Step 5: 清理二进制文件
for suffix in "${PLATFORM_SUFFIXES[@]}"; do
  rm -f "$NPM_DIR/fboxcli-${suffix}/${BIN_NAMES[$suffix]}"
done

echo "==> Done! Published @flexem/fboxcli@${VERSION}"
