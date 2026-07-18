<#
.SYNOPSIS
    TinyTransfer API 测试脚本
    用法: powershell -ExecutionPolicy Bypass -File scripts\test-api.ps1
#>

$ErrorActionPreference = "SilentlyContinue"
$BaseUrl = "http://127.0.0.1:5000"
$Pass = 0
$Fail = 0
$Results = @()

function Test-Api {
    param(
        [string]$Name,
        [string]$Method,
        [string]$Url,
        [string]$Body = "",
        [hashtable]$Headers = @{},
        [string]$ExpectContains = "",
        [int]$ExpectStatus = 200
    )

    $fullUrl = if ($Url.StartsWith("http")) { $Url } else { "$BaseUrl$Url" }

    $script:ok = $false
    $script:actualStatus = 0
    $script:detail = ""

    try {
        $params = @{
            Uri             = $fullUrl
            Method          = $Method
            UseBasicParsing = $true
            TimeoutSec      = 10
            ErrorAction     = "Stop"
        }
        if ($Headers.Count -gt 0) { $params.Headers = $Headers }
        if ($Body -ne "") { $params.Body = $Body }

        $resp = Invoke-WebRequest @params
        $script:actualStatus = [int]$resp.StatusCode
        $content = $resp.Content

        if ($ExpectContains -ne "") {
            $script:ok = ($script:actualStatus -eq $ExpectStatus) -and ($content -match $ExpectContains)
        } else {
            $script:ok = ($script:actualStatus -eq $ExpectStatus)
        }

        if ($content.Length -gt 200) {
            $script:detail = $content.Substring(0, 200) + "..."
        } else {
            $script:detail = $content
        }
    }
    catch {
        if ($_.Exception.Response) {
            $script:actualStatus = [int]$_.Exception.Response.StatusCode
        } else {
            $script:actualStatus = 0
        }

        if ($script:actualStatus -eq $ExpectStatus) {
            $script:ok = $true
            $script:detail = "Expected status $ExpectStatus"
        } else {
            $script:ok = $false
            $script:detail = $_.Exception.Message
        }
    }

    if ($script:ok) { $script:Pass++ } else { $script:Fail++ }

    $Results += [PSCustomObject]@{
        Status = $(if ($script:ok) { "PASS" } else { "FAIL" })
        Name   = $Name
        Method = $Method
        Url    = $Url
        HTTP   = $script:actualStatus
        Detail = $script:detail
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  TinyTransfer API Test Suite" -ForegroundColor Cyan
Write-Host "  Target: $BaseUrl" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# ---- 1. 服务器连通性 ----
Write-Host "`n[1] Server Connectivity" -ForegroundColor Yellow
Test-Api -Name "Index Page" -Method GET -Url "/" -ExpectContains "<html"
Test-Api -Name "Check Auth" -Method GET -Url "/api/check-auth" -ExpectContains "authenticated"
Test-Api -Name "Get IP" -Method GET -Url "/api/ip" -ExpectContains "url"

# ---- 2. 设备相关 ----
Write-Host "`n[2] Devices" -ForegroundColor Yellow
Test-Api -Name "Device List" -Method GET -Url "/api/devices" -ExpectContains "connected"
Test-Api -Name "Ping" -Method POST -Url "/api/ping" -ExpectContains "ok"

# ---- 3. 文件列表 ----
Write-Host "`n[3] File List" -ForegroundColor Yellow
Test-Api -Name "List Files" -Method GET -Url "/api/files"
Test-Api -Name "List Uploaded Files" -Method GET -Url "/api/uploaded-files"
Test-Api -Name "List All Files" -Method GET -Url "/api/all-files"
Test-Api -Name "All-Files Format (array)" -Method GET -Url "/api/all-files" -ExpectContains "^\["

# ---- 4. 磁盘信息 ----
Write-Host "`n[4] Disk Info" -ForegroundColor Yellow
Test-Api -Name "Disk Info" -Method GET -Url "/api/disk-info" -ExpectContains "total"

# ---- 5. 标题栏颜色 ----
Write-Host "`n[5] Titlebar Color" -ForegroundColor Yellow
Test-Api -Name "Set Dark" -Method POST -Url "/api/set-titlebar-color" -Body '{"is_dark":true}' -Headers @{"Content-Type"="application/json"} -ExpectContains "success"
Test-Api -Name "Set Light" -Method POST -Url "/api/set-titlebar-color" -Body '{"is_dark":false}' -Headers @{"Content-Type"="application/json"} -ExpectContains "success"

# ---- 6. 静态资源 ----
Write-Host "`n[6] Static Assets" -ForegroundColor Yellow
try {
    $indexResp = Invoke-WebRequest -Uri "$BaseUrl/" -UseBasicParsing -TimeoutSec 5
    $jsMatch = [regex]::Match($indexResp.Content, '/assets/[a-zA-Z0-9_-]+\.js')
    $cssMatch = [regex]::Match($indexResp.Content, '/assets/[a-zA-Z0-9_-]+\.css')
    if ($jsMatch.Success) {
        Test-Api -Name "JS Asset" -Method GET -Url $jsMatch.Value
    } else {
        $Fail++
        $Results += [PSCustomObject]@{Status="FAIL";Name="JS Asset";Method="GET";Url="/assets/*.js";HTTP=0;Detail="No JS in index.html"}
    }
    if ($cssMatch.Success) {
        Test-Api -Name "CSS Asset" -Method GET -Url $cssMatch.Value
    } else {
        $Fail++
        $Results += [PSCustomObject]@{Status="FAIL";Name="CSS Asset";Method="GET";Url="/assets/*.css";HTTP=0;Detail="No CSS in index.html"}
    }
} catch {
    $Fail++
    $Results += [PSCustomObject]@{Status="FAIL";Name="Static Assets";Method="GET";Url="/assets/*";HTTP=0;Detail=$_.Exception.Message}
}

# ---- 7. 认证流程 ----
Write-Host "`n[7] Auth Flow" -ForegroundColor Yellow
Test-Api -Name "No Password = Host" -Method GET -Url "/api/check-auth" -ExpectContains '"role":"host"'
Test-Api -Name "Login Empty Password" -Method POST -Url "/api/auth" -Body '{"password":""}' -Headers @{"Content-Type"="application/json"} -ExpectContains "success"

# ---- 8. SSE 连接（用 TcpClient 验证流式响应） ----
Write-Host "`n[8] SSE Connection" -ForegroundColor Yellow
try {
    $tcp = New-Object System.Net.Sockets.TcpClient
    $tcp.Connect("127.0.0.1", 5000)
    $stream = $tcp.GetStream()
    $req = "GET /api/events?ua=test HTTP/1.1`r`nHost: 127.0.0.1:5000`r`nConnection: keep-alive`r`n`r`n"
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($req)
    $stream.Write($bytes, 0, $bytes.Length)
    Start-Sleep -Milliseconds 1500
    $buf = New-Object byte[] 4096
    $read = $stream.Read($buf, 0, $buf.Length)
    $response = [System.Text.Encoding]::UTF8.GetString($buf, 0, $read)
    $tcp.Close()

    if ($response -match "HTTP/1.1 200" -and $response -match "text/event-stream" -and $response -match "hello") {
        $Pass++
        $Results += [PSCustomObject]@{Status="PASS";Name="SSE Connection";Method="GET";Url="/api/events";HTTP=200;Detail="hello event received"}
    } else {
        $Fail++
        $Results += [PSCustomObject]@{Status="FAIL";Name="SSE Connection";Method="GET";Url="/api/events";HTTP=0;Detail="No hello event"}
    }
} catch {
    $Fail++
    $Results += [PSCustomObject]@{Status="FAIL";Name="SSE Connection";Method="GET";Url="/api/events";HTTP=0;Detail=$_.Exception.Message}
}

# ---- 9. 下载/删除/ZIP 边界测试 ----
Write-Host "`n[9] Edge Cases" -ForegroundColor Yellow
Test-Api -Name "Download Non-existent" -Method GET -Url "/api/download/__nonexist__.txt" -ExpectStatus 404
Test-Api -Name "Delete Empty List" -Method POST -Url "/api/delete-files" -Body '{"filenames":[]}' -Headers @{"Content-Type"="application/json"} -ExpectContains "success"
Test-Api -Name "ZIP Empty List" -Method POST -Url "/api/download-zip" -Body '{"filenames":[]}' -Headers @{"Content-Type"="application/json"} -ExpectStatus 400

# ---- 输出结果 ----
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Test Results" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

foreach ($r in $Results) {
    if ($r.Status -eq "PASS") {
        Write-Host "  [OK] " -ForegroundColor Green -NoNewline
    } else {
        Write-Host "  [XX] " -ForegroundColor Red -NoNewline
    }
    Write-Host "$($r.Method) $($r.Url) (HTTP $($r.HTTP))" -NoNewline
    Write-Host ""
    if ($r.Status -eq "FAIL") {
        Write-Host "       -> $($r.Detail)" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
$total = $Pass + $Fail
$summaryColor = if ($Fail -eq 0) { "Green" } else { "Yellow" }
Write-Host "  Total: $total  |  Pass: $Pass  |  Fail: $Fail" -ForegroundColor $summaryColor
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if ($Fail -gt 0) { exit 1 } else { exit 0 }
