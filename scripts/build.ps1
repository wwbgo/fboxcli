#Requires -Version 7.0
<#
.SYNOPSIS
    fboxcli 多平台编译脚本
.DESCRIPTION
    - win32-x64: 本地 cargo 编译
    - darwin-arm64: SSH 到 Mac 编译
    - linux-x64: SSH 到 Linux 编译
.PARAMETER Target
    指定编译目标：win32-x64, linux-x64, darwin-arm64, all (默认 all)
.PARAMETER SkipCopy
    只编译，不拷贝到 npm 目录
.EXAMPLE
    ./scripts/build.ps1                          # 编译所有平台
    ./scripts/build.ps1 -Target win32-x64        # 只编译 Windows
    ./scripts/build.ps1 -Target darwin-arm64      # 只编译 macOS
    ./scripts/build.ps1 -Target linux-x64         # 只编译 Linux
#>
param(
    [ValidateSet('all', 'win32-x64', 'darwin-arm64', 'linux-x64')]
    [string[]]$Target = @('all'),
    [switch]$SkipCopy
)

$ErrorActionPreference = 'Stop'

$RootDir = Split-Path -Parent (Split-Path -Parent $PSCommandPath)
$NpmDir = Join-Path $RootDir 'npm'
$ScriptsDir = Split-Path -Parent $PSCommandPath

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

# 平台定义（从 .env 变量读取远程主机配置）
$Targets = @{
    'win32-x64'    = @{ RustTarget = 'x86_64-pc-windows-msvc';  BinName = 'fboxcli.exe'; Location = 'local' }
    'darwin-arm64' = @{ RustTarget = 'aarch64-apple-darwin';     BinName = 'fboxcli';     Location = 'remote'; Host = $DARWIN_ARM64_HOST; RemoteDir = $DARWIN_ARM64_REMOTE_DIR }
    'linux-x64'    = @{ RustTarget = 'x86_64-unknown-linux-gnu'; BinName = 'fboxcli';     Location = 'remote'; Host = $LINUX_X64_HOST;    RemoteDir = $LINUX_X64_REMOTE_DIR }
}

$buildList = if ($Target -contains 'all') { $Targets.Keys } else { $Target }

# ── 辅助函数 ──

function Invoke-Remote {
    param([string]$SshHost, [string]$Command)
    Write-Host "    [ssh] $Command" -ForegroundColor DarkGray
    ssh -o ConnectTimeout=10 -o StrictHostKeyChecking=no $SshHost $Command
    if ($LASTEXITCODE -ne 0) { Write-Error "SSH command failed: $Command" }
}

function Sync-Source {
    param([string]$SshHost, [string]$RemoteDir)
    Write-Host "    Syncing source to ${SshHost}:${RemoteDir} ..." -ForegroundColor Yellow

    # 创建目录并清理旧源码（保留 target 缓存）
    Invoke-Remote $SshHost "mkdir -p $RemoteDir && cd $RemoteDir && ls | grep -v '^target$' | xargs rm -rf 2>/dev/null; true"

    # 创建临时 zip
    $tempZip = Join-Path ([System.IO.Path]::GetTempPath()) 'fboxcli-src.zip'
    Remove-Item $tempZip -Force -ErrorAction SilentlyContinue

    $srcItems = Get-ChildItem -Path $RootDir | Where-Object {
        $_.Name -notin @('target', 'node_modules', '.git')
    }
    Compress-Archive -Path $srcItems.FullName -DestinationPath $tempZip -Force

    # 上传并解压
    scp -o StrictHostKeyChecking=no $tempZip "${SshHost}:${RemoteDir}/src.zip"
    if ($LASTEXITCODE -ne 0) { Write-Error "scp upload failed" }
    Invoke-Remote $SshHost "cd $RemoteDir && unzip -oq src.zip && rm src.zip"

    Remove-Item $tempZip -Force -ErrorAction SilentlyContinue
    Write-Host "    Sync complete" -ForegroundColor Green
}

# ── 同步源码到需要的远程主机（去重） ──

$remoteHosts = @{}
foreach ($suffix in $buildList) {
    $t = $Targets[$suffix]
    if ($t.Location -eq 'remote' -and -not $remoteHosts.ContainsKey($t.Host)) {
        $remoteHosts[$t.Host] = $t.RemoteDir
    }
}
foreach ($h in $remoteHosts.Keys) {
    Write-Host "==> Syncing source to $h" -ForegroundColor Cyan
    Sync-Source $h $remoteHosts[$h]
}

# ── 编译 ──

foreach ($suffix in $buildList) {
    $t = $Targets[$suffix]

    if ($t.Location -eq 'local') {
        Write-Host "==> [$suffix] Building locally for $($t.RustTarget)" -ForegroundColor Cyan
        cargo build --release --target $t.RustTarget
        if ($LASTEXITCODE -ne 0) { Write-Error "Build failed for $($t.RustTarget)" }

        $src = Join-Path $RootDir 'target' $t.RustTarget 'release' $t.BinName
        $size = [math]::Round((Get-Item $src).Length / 1MB, 1)
        Write-Host "    Output: $src ($size MB)" -ForegroundColor Green

        if (-not $SkipCopy) {
            $dest = Join-Path $NpmDir "fboxcli-$suffix" $t.BinName
            Copy-Item $src $dest -Force
            Write-Host "    Copied -> $dest" -ForegroundColor Green
        }
    }
    else {
        Write-Host "==> [$suffix] Building on $($t.Host) for $($t.RustTarget)" -ForegroundColor Cyan
        Invoke-Remote $t.Host "cd $($t.RemoteDir) && cargo build --release --target $($t.RustTarget)"

        $remoteBin = "$($t.RemoteDir)/target/$($t.RustTarget)/release/$($t.BinName)"
        Invoke-Remote $t.Host "ls -lh $remoteBin"

        if (-not $SkipCopy) {
            $dest = Join-Path $NpmDir "fboxcli-$suffix" $t.BinName
            Write-Host "    Downloading binary ..." -ForegroundColor Yellow
            scp -o StrictHostKeyChecking=no "$($t.Host):${remoteBin}" "$dest"
            if ($LASTEXITCODE -ne 0) { Write-Error "scp download failed" }
            $size = [math]::Round((Get-Item $dest).Length / 1MB, 1)
            Write-Host "    Copied -> $dest ($size MB)" -ForegroundColor Green
        }
    }
}

# ── 汇总 ──

Write-Host "`n==> Build complete!" -ForegroundColor Cyan
foreach ($suffix in $buildList) {
    $t = $Targets[$suffix]
    if ($t.Location -eq 'local') {
        $bin = Join-Path $RootDir 'target' $t.RustTarget 'release' $t.BinName
    } else {
        $bin = Join-Path $NpmDir "fboxcli-$suffix" $t.BinName
    }
    if (Test-Path $bin) {
        $size = [math]::Round((Get-Item $bin).Length / 1MB, 1)
        Write-Host "    $($suffix.PadRight(15)) $size MB" -ForegroundColor White
    }
}
