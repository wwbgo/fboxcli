#Requires -Version 7.0
<#
.SYNOPSIS
    fboxcli npm 发布脚本 (PowerShell 7+)
.DESCRIPTION
    调用 build.ps1 编译所有平台，同步版本号，发布到 npm
.PARAMETER SkipBuild
    跳过编译步骤（使用 npm 目录中已有的二进制）
.PARAMETER DryRun
    干跑模式，不实际发布
.EXAMPLE
    ./scripts/npm-publish.ps1
    ./scripts/npm-publish.ps1 -DryRun
    ./scripts/npm-publish.ps1 -SkipBuild
#>
param(
    [switch]$SkipBuild,
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'

$RootDir = Split-Path -Parent (Split-Path -Parent $PSCommandPath)
$NpmDir = Join-Path $RootDir 'npm'
$ScriptsDir = Split-Path -Parent $PSCommandPath

# 平台包列表
$PlatformSuffixes = @('win32-x64', 'darwin-arm64', 'linux-x64')
$BinNames = @{
    'win32-x64'    = 'fboxcli.exe'
    'darwin-arm64' = 'fboxcli'
    'linux-x64'    = 'fboxcli'
}

# ── 加载 .env ──
$EnvFile = Join-Path $ScriptsDir '.env'
if (Test-Path $EnvFile) {
    Get-Content $EnvFile | ForEach-Object {
        if ($_ -match '^\s*([^#][^=]+?)\s*=\s*(.+?)\s*$') {
            Set-Variable -Name $Matches[1] -Value $Matches[2]
        }
    }
} else {
    Write-Error "Missing .env file: $EnvFile"
}

# 从 Cargo.toml 提取版本号
$CargoToml = Get-Content (Join-Path $RootDir 'Cargo.toml') -Raw
if ($CargoToml -match '(?m)^version\s*=\s*"([^"]+)"') {
    $Version = $Matches[1]
} else {
    Write-Error 'Failed to extract version from Cargo.toml'
}
Write-Host "==> Version: $Version"

$NpmTag = 'latest'
$NpmRegistry = if ($NPM_REGISTRY) { $NPM_REGISTRY } else { 'https://registry.npmjs.org' }

# Step 1: 编译
if (-not $SkipBuild) {
    Write-Host "==> Building all platforms ..." -ForegroundColor Cyan
    & "$ScriptsDir/build.ps1" -Target all
    if ($LASTEXITCODE -ne 0) { Write-Error "Build failed" }
} else {
    Write-Host '==> Skipping build (-SkipBuild)'
    foreach ($suffix in $PlatformSuffixes) {
        $binPath = Join-Path $NpmDir "fboxcli-$suffix" $BinNames[$suffix]
        if (-not (Test-Path $binPath)) {
            Write-Error "Missing binary: $binPath"
        }
    }
}

# Step 2: 同步版本号到所有 package.json
Write-Host '==> Updating versions'

Get-ChildItem -Path $NpmDir -Directory | ForEach-Object {
    $PkgJson = Join-Path $_.FullName 'package.json'
    if (Test-Path $PkgJson) {
        $pkg = Get-Content $PkgJson -Raw | ConvertFrom-Json
        $pkg.version = $Version
        $pkg | ConvertTo-Json -Depth 10 | Set-Content $PkgJson -NoNewline
    }
}

# 更新主包的 optionalDependencies 版本
$MainPkgPath = Join-Path $NpmDir 'fboxcli' 'package.json'
$mainPkg = Get-Content $MainPkgPath -Raw | ConvertFrom-Json
foreach ($key in @($mainPkg.optionalDependencies.PSObject.Properties.Name)) {
    $mainPkg.optionalDependencies.$key = $Version
}
$mainPkg | ConvertTo-Json -Depth 10 | Set-Content $MainPkgPath -NoNewline

# Step 3: 发布平台包（先于主包）
foreach ($suffix in $PlatformSuffixes) {
    Write-Host "==> Publishing @flexem/fboxcli-${suffix}@${Version}"
    Push-Location (Join-Path $NpmDir "fboxcli-$suffix")
    try {
        if ($DryRun) {
            npm publish --access public --tag $NpmTag --registry $NpmRegistry "--//registry.npmjs.org/:_authToken=$NPM_TOKEN" --dry-run
        } else {
            npm publish --access public --tag $NpmTag --registry $NpmRegistry "--//registry.npmjs.org/:_authToken=$NPM_TOKEN"
        }
        if ($LASTEXITCODE -ne 0) { Write-Error "Publish failed for @flexem/fboxcli-$suffix" }
    } finally {
        Pop-Location
    }
}

# Step 4: 发布主包
Write-Host "==> Publishing @flexem/fboxcli@${Version}"
Push-Location (Join-Path $NpmDir 'fboxcli')
try {
    if ($DryRun) {
        npm publish --access public --tag $NpmTag --registry $NpmRegistry "--//registry.npmjs.org/:_authToken=$NPM_TOKEN" --dry-run
    } else {
        npm publish --access public --tag $NpmTag --registry $NpmRegistry "--//registry.npmjs.org/:_authToken=$NPM_TOKEN"
    }
    if ($LASTEXITCODE -ne 0) { Write-Error 'Publish failed for @flexem/fboxcli' }
} finally {
    Pop-Location
}

# Step 5: 清理二进制文件
foreach ($suffix in $PlatformSuffixes) {
    $binPath = Join-Path $NpmDir "fboxcli-$suffix" $BinNames[$suffix]
    if (Test-Path $binPath) { Remove-Item $binPath }
}

Write-Host "==> Done! Published @flexem/fboxcli@${Version}"
