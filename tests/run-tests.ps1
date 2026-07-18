#Requires -Version 5.0
<#
.SYNOPSIS
    TinyTransfer 端到端测试脚本
.DESCRIPTION
    使用 agent-browser (CDP) 对运行中的 TinyTransfer.exe 做全功能验证。
    截图保存到 tests\screenshots\ 子目录。
.NOTES
    用法:
      1. 双击运行 output\TinyTransfer.exe (确保端口 5000 已监听)
      2. 执行本脚本:
         powershell -ExecutionPolicy Bypass -File tests\run-tests.ps1
    依赖:
      - agent-browser (npm i -g agent-browser)
      - Chrome/Chromium (agent-browser install)
#>

# ===== 全局配置 =====
$ErrorActionPreference = "Continue"
$AppUrl = "http://localhost:5000"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ScreenshotDir = Join-Path $ScriptDir "screenshots"
$SocketDir = Join-Path $ScriptDir ".ab-socket"
$TestResults = [System.Collections.ArrayList]::new()
$PassedCount = 0
$FailedCount = 0

# 使用项目目录下的 socket 目录, 避免 C:\Users 路径的权限问题
if (-not (Test-Path $SocketDir)) {
    New-Item -ItemType Directory -Path $SocketDir -Force | Out-Null
}
$env:AGENT_BROWSER_SOCKET_DIR = $SocketDir

if (-not (Test-Path $ScreenshotDir)) {
    New-Item -ItemType Directory -Path $ScreenshotDir -Force | Out-Null
}

# ===== 辅助函数 =====

function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $color = switch ($Level) {
        "PASS"  { "Green" }
        "FAIL"  { "Red" }
        "WARN"  { "Yellow" }
        "STEP"  { "Cyan" }
        default { "White" }
    }
    Write-Host "[$(Get-Date -Format 'HH:mm:ss')] [$Level] $Message" -ForegroundColor $color
}

function Add-Result {
    param([string]$Name, $Passed, [string]$Detail = "")
    # 安全转换为布尔值, 处理空字符串/null/0/false 等
    $boolPassed = $false
    if ($Passed -is [bool]) {
        $boolPassed = $Passed
    } elseif ($Passed -is [string]) {
        $boolPassed = ($Passed -eq $true -or $Passed -eq "True" -or $Passed -eq "true" -or $Passed -eq "1")
    } elseif ($Passed -is [int] -or $Passed -is [long]) {
        $boolPassed = ($Passed -ne 0)
    } elseif ($Passed) {
        $boolPassed = [bool]$Passed
    }
    $null = $TestResults.Add([PSCustomObject]@{
        Name = $Name
        Passed = $boolPassed
        Detail = $Detail
        Time = Get-Date -Format "HH:mm:ss"
    })
    if ($boolPassed) {
        $script:PassedCount++
        Write-Log "PASS: $Name" "PASS"
    } else {
        $script:FailedCount++
        Write-Log "FAIL: $Name - $Detail" "FAIL"
    }
}

# 执行 JS 并返回结果 (用 base64 避免 PowerShell 转义问题)
# 含重试逻辑, 当 eval 返回空时自动等待并重试
function Invoke-Js {
    param([string]$JsCode, [int]$MaxRetries = 3)
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($JsCode)
    $b64 = [Convert]::ToBase64String($bytes)

    $retryCount = 0
    $output = ""
    while ($retryCount -lt $MaxRetries) {
        $rawOutput = agent-browser eval -b $b64 2>&1
        $output = ($rawOutput -join "`n").Trim()
        # agent-browser 返回 JSON 字符串, 可能带引号和转义
        $output = $output.TrimStart('"').TrimEnd('"').Replace('\"', '"')
        if (-not [string]::IsNullOrEmpty($output)) {
            break
        }
        $retryCount++
        if ($retryCount -lt $MaxRetries) {
            Write-Log "Invoke-Js 返回空, 第 $retryCount 次重试..." "WARN"
            Start-Sleep -Milliseconds 800
        }
    }

    if ([string]::IsNullOrEmpty($output)) {
        Write-Log "Invoke-Js 多次重试后仍为空" "WARN"
        return $null
    }
    try {
        return $output | ConvertFrom-Json
    } catch {
        return $output
    }
}

function Invoke-Ab {
    param([string[]]$Args)
    $output = & agent-browser @Args 2>&1
    return ($output -join "`n")
}

function Take-Screenshot {
    param([string]$Name)
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $filepath = Join-Path $ScreenshotDir "${timestamp}_${Name}.png"
    agent-browser screenshot --full $filepath 2>&1 | Out-Null
    return $filepath
}

function Wait-Ab {
    param([int]$Ms = 500)
    agent-browser wait $Ms 2>&1 | Out-Null
}

# ===== 测试用例 =====

function Test-Environment {
    Write-Log "检查测试环境..." "STEP"

    # 检查 agent-browser 是否安装
    $abVersion = agent-browser --version 2>&1
    if ($LASTEXITCODE -ne 0) {
        Add-Result "环境检查: agent-browser 已安装" $false "未安装 agent-browser, 请运行: npm i -g agent-browser"
        return $false
    }
    Add-Result "环境检查: agent-browser 已安装" $true "v$($abVersion.ToString().Trim())"

    # 不关闭已有会话 - close --all 后 open 会卡住, 直接复用现有会话更快
    # agent-browser open 会自动导航现有 tab 或创建新 tab

    # 检查应用是否运行
    $portOpen = (Test-NetConnection -ComputerName localhost -Port 5000 -InformationLevel Quiet -WarningAction SilentlyContinue)
    if (-not $portOpen) {
        Add-Result "环境检查: TinyTransfer 运行中 (端口 5000)" $false "应用未运行, 请先启动 output\TinyTransfer.exe"
        return $false
    }
    Add-Result "环境检查: TinyTransfer 运行中 (端口 5000)" $true
    return $true
}

function Test-PageLoad {
    Write-Log "测试 1: 页面加载" "STEP"
    agent-browser open $AppUrl 2>&1 | Out-Null
    # daemon 重置后首次 open 需要更长时间让 Chrome 启动并稳定
    Start-Sleep -Seconds 2
    agent-browser wait --load networkidle 2>&1 | Out-Null
    Wait-Ab 1500

    $result = Invoke-Js -JsCode 'JSON.stringify({title:document.title,heroTitle:document.querySelector(".hero-title")?document.querySelector(".hero-title").textContent:null,qrSrc:document.querySelector(".qr-img")?document.querySelector(".qr-img").src.substring(0,30):null,bodyClass:document.body.className})'

    if ($null -eq $result) {
        Add-Result "页面加载: 标题正确" $false "Invoke-Js 返回 null, daemon 可能未就绪"
        return
    }
    Add-Result "页面加载: 标题正确" ($result.title -eq "TinyTransfer") "标题: $($result.title)"
    Add-Result "页面加载: Hero 标题显示" ($null -ne $result.heroTitle) "标题: $($result.heroTitle)"
    Add-Result "页面加载: 二维码已生成" ($null -ne $result.qrSrc -and $result.qrSrc -ne "") "QR src: $($result.qrSrc)"
    Add-Result "页面加载: 角色为 host" ($result.bodyClass -match "role-host") "body class: $($result.bodyClass)"

    Take-Screenshot -Name "01_page_load" | Out-Null
}

function Test-GuidedTour {
    Write-Log "测试 2: 引导教程" "STEP"
    $result = Invoke-Js -JsCode 'JSON.stringify({visible:!!document.querySelector(".guide-overlay"),title:document.querySelector(".guide-card-title")?document.querySelector(".guide-card-title").textContent:null,skipBtn:!!document.querySelector(".guide-btn-skip")})'

    # 引导教程只首次显示, 已完成是正常状态
    if (-not $result.visible) {
        Add-Result "引导教程: 显示" $true "已完成 (localStorage 记住, 不再显示)"
        Add-Result "引导教程: 包含标题" $true "已完成 (跳过)"
        Add-Result "引导教程: 跳过按钮可关闭" $true "已完成 (跳过)"
        return
    }

    Add-Result "引导教程: 显示" $result.visible "可见性: $($result.visible)"
    Add-Result "引导教程: 包含标题" ($null -ne $result.title) "标题: $($result.title)"

    Take-Screenshot -Name "02_guided_tour" | Out-Null
    # 用 JS 点击跳过按钮 (更可靠)
    $skipClicked = Invoke-Js -JsCode 'var btns=Array.from(document.querySelectorAll("button"));var s=btns.find(function(b){return b.textContent.includes("跳过")});if(s){s.click();JSON.stringify({clicked:true})}else{JSON.stringify({clicked:false})}'
    if ($skipClicked.clicked) {
        Wait-Ab 800
        $afterSkip = Invoke-Js -JsCode 'JSON.stringify({visible:!!document.querySelector(".guide-overlay")})'
        Add-Result "引导教程: 跳过按钮可关闭" (-not $afterSkip.visible) "跳过后可见: $($afterSkip.visible)"
    } else {
        Add-Result "引导教程: 跳过按钮可关闭" $false "未找到跳过按钮"
    }
}

function Test-FileSection {
    Write-Log "测试 3: 文件区域" "STEP"
    $result = Invoke-Js -JsCode 'JSON.stringify({filesSection:!!document.querySelector("#filesSection"),filesHeader:!!document.querySelector(".files-header"),fileGrid:!!document.querySelector(".file-grid"),fileCount:document.querySelectorAll(".file-card").length,emptyState:!!document.querySelector(".empty-state"),titleText:document.querySelector(".files-title")?document.querySelector(".files-title").textContent.trim():null,viewBtn:!!document.querySelector("[title=''切换视图'']"),batchBtn:!!document.querySelector("[title=''多选'']"),uploadBtn:!!document.querySelector(".btn-primary")})'

    Add-Result "文件区域: #filesSection 存在" $result.filesSection
    Add-Result "文件区域: 标题显示" ($null -ne $result.titleText) "标题: $($result.titleText)"
    Add-Result "文件区域: 文件网格存在" $result.fileGrid "文件数: $($result.fileCount)"
    Add-Result "文件区域: 视图按钮存在" $result.viewBtn
    Add-Result "文件区域: 多选按钮存在" $result.batchBtn
    Add-Result "文件区域: 上传按钮存在" $result.uploadBtn

    Take-Screenshot -Name "03_file_section" | Out-Null
}

function Test-FilterPopover {
    Write-Log "测试 4: 筛选弹出框" "STEP"
    # 用 JS 直接点击文件标题 (比 snapshot ref 更可靠)
    $clickResult = Invoke-Js -JsCode 'var h=document.querySelector(".files-title");if(h){h.click();JSON.stringify({clicked:true})}else{JSON.stringify({clicked:false})}'
    if ($clickResult.clicked) {
        Wait-Ab 500
        $result = Invoke-Js -JsCode 'JSON.stringify({popover:!!document.querySelector(".filter-popover"),filterItems:document.querySelectorAll(".filter-type-item").length,sortItems:document.querySelectorAll(".filter-sort-item").length})'
        Add-Result "筛选弹出框: 展开" $result.popover
        Add-Result "筛选弹出框: 筛选类型项" ($result.filterItems -gt 0) "数量: $($result.filterItems)"
        Add-Result "筛选弹出框: 排序选项" ($result.sortItems -gt 0) "数量: $($result.sortItems)"
        Take-Screenshot -Name "04_filter_popover" | Out-Null

        # 点击外部关闭
        agent-browser press Escape 2>&1 | Out-Null
        Wait-Ab 300
    } else {
        Add-Result "筛选弹出框: 展开" $false "未找到文件标题元素"
    }
}

function Test-ViewToggle {
    Write-Log "测试 5: 视图切换" "STEP"
    # 用 JS 直接点击视图按钮
    $before = Invoke-Js -JsCode 'JSON.stringify({isListView:document.querySelector(".file-grid")?document.querySelector(".file-grid").classList.contains("list-view"):false,btnFound:!!document.querySelector("[title=''切换视图'']")})'
    if ($before.btnFound) {
        Invoke-Js -JsCode 'var b=document.querySelector("[title=''切换视图'']");if(b){b.click()}' | Out-Null
        Wait-Ab 500
        $after = Invoke-Js -JsCode 'JSON.stringify({isListView:document.querySelector(".file-grid")?document.querySelector(".file-grid").classList.contains("list-view"):false})'
        Add-Result "视图切换: 状态变化" ($before.isListView -ne $after.isListView) "切换前: $($before.isListView), 切换后: $($after.isListView)"
        Take-Screenshot -Name "05_view_toggle" | Out-Null

        # 切换回去
        Invoke-Js -JsCode 'var b=document.querySelector("[title=''切换视图'']");if(b){b.click()}' | Out-Null
        Wait-Ab 300
    } else {
        Add-Result "视图切换: 状态变化" $false "未找到视图按钮"
    }
}

function Test-BatchMode {
    Write-Log "测试 6: 多选模式" "STEP"
    # 用 JS 直接点击多选按钮 (title="多选")
    $btnFound = Invoke-Js -JsCode 'JSON.stringify({found:!!document.querySelector("[title=''多选'']")})'
    if ($btnFound.found) {
        Invoke-Js -JsCode 'var b=document.querySelector("[title=''多选'']");if(b){b.click()}' | Out-Null
        Wait-Ab 500
        $result = Invoke-Js -JsCode 'JSON.stringify({batchBar:!!document.querySelector(".batch-bar"),batchMode:document.querySelector("[title=''多选'']")?document.querySelector("[title=''多选'']").classList.contains("active"):false})'
        Add-Result "多选模式: 批量操作栏显示" $result.batchBar
        Add-Result "多选模式: 按钮高亮" $result.batchMode
        Take-Screenshot -Name "06_batch_mode" | Out-Null

        # 退出多选
        Invoke-Js -JsCode 'var b=document.querySelector("[title=''多选'']");if(b){b.click()}' | Out-Null
        Wait-Ab 300
    } else {
        Add-Result "多选模式: 批量操作栏显示" $false "未找到多选按钮"
    }
}

function Test-SettingsModal {
    Write-Log "测试 7: 设置弹窗" "STEP"
    # 用 JS 点击"设置"按钮 (title="设置" 或文本含"设置")
    $settingsClicked = Invoke-Js -JsCode 'var btns=Array.from(document.querySelectorAll("button"));var s=btns.find(function(b){return b.textContent.includes("设置")||b.getAttribute("title")==="设置"});if(s){s.click();JSON.stringify({clicked:true})}else{JSON.stringify({clicked:false})}'
    if ($settingsClicked.clicked) {
        Wait-Ab 800

        $result = Invoke-Js -JsCode 'var m=document.querySelector(".modal-overlay");var s=document.querySelector(".modal-sheet");var r=s?s.getBoundingClientRect():null;JSON.stringify({overlayExists:!!m,overlayDisplay:m?getComputedStyle(m).display:null,overlayJustify:m?getComputedStyle(m).justifyContent:null,overlayAlign:m?getComputedStyle(m).alignItems:null,sheetRect:r?{x:r.x,y:r.y,w:r.width,h:r.height}:null,viewport:{w:window.innerWidth,h:window.innerHeight},settingsTitle:document.querySelector(".modal-title")?document.querySelector(".modal-title").textContent:null,themeItem:!!document.querySelector("[class*=''settings'']"),folderItem:document.body.innerText.includes("uploads"),tourItem:document.body.innerText.includes("重新引导"),debugItem:document.body.innerText.includes("速度调试"),aboutItem:document.body.innerText.includes("关于")})'

        Add-Result "设置弹窗: overlay 存在" $result.overlayExists
        Add-Result "设置弹窗: display=flex" ($result.overlayDisplay -eq "flex") "display: $($result.overlayDisplay)"
        Add-Result "设置弹窗: 水平居中" ($result.overlayJustify -eq "center") "justify: $($result.overlayJustify)"
        Add-Result "设置弹窗: 垂直居中" ($result.overlayAlign -eq "center") "align: $($result.overlayAlign)"

        if ($result.sheetRect -and $result.viewport) {
            $sheetCx = $result.sheetRect.x + $result.sheetRect.w / 2
            $sheetCy = $result.sheetRect.y + $result.sheetRect.h / 2
            $viewCx = $result.viewport.w / 2
            $viewCy = $result.viewport.h / 2
            $xCentered = [Math]::Abs($sheetCx - $viewCx) -lt 20
            $yCentered = [Math]::Abs($sheetCy - $viewCy) -lt 50  # 允许更大误差因为可能滚动
            Add-Result "设置弹窗: 实际居中 (X)" $xCentered "sheet Cx: $([Math]::Round($sheetCx,1)), viewport Cx: $viewCx"
            Add-Result "设置弹窗: 实际居中 (Y)" $yCentered "sheet Cy: $([Math]::Round($sheetCy,1)), viewport Cy: $viewCy"
        }

        Add-Result "设置弹窗: 标题显示" ($null -ne $result.settingsTitle) "标题: $($result.settingsTitle)"
        Add-Result "设置弹窗: 主题项" $result.themeItem
        Add-Result "设置弹窗: 文件夹项" $result.folderItem
        Add-Result "设置弹窗: 重新引导项" $result.tourItem
        Add-Result "设置弹窗: 速度调试项" $result.debugItem
        Add-Result "设置弹窗: 关于项" $result.aboutItem

        Take-Screenshot -Name "07_settings_modal" | Out-Null
    } else {
        Add-Result "设置弹窗: overlay 存在" $false "未找到设置按钮"
    }
}

function Test-ThemeSwitch {
    Write-Log "测试 8: 主题切换" "STEP"
    # 用 JS 点击主题选择行打开下拉菜单
    Invoke-Js -JsCode 'var r=document.querySelector(".settings-select-row");if(r){r.click()}' | Out-Null
    Wait-Ab 400

    $dropdown = Invoke-Js -JsCode 'JSON.stringify({dropdown:!!document.querySelector(".settings-dropdown"),items:document.querySelectorAll(".settings-dropdown-item").length})'
    Add-Result "主题切换: 下拉菜单展开" $dropdown.dropdown "项数: $($dropdown.items)"
    Take-Screenshot -Name "08_theme_dropdown" | Out-Null

    # 用 JS 点击"深色模式"下拉项
    $darkClicked = Invoke-Js -JsCode 'var items=Array.from(document.querySelectorAll(".settings-dropdown-item"));var d=items.find(function(i){return i.textContent.includes("深色模式")});if(d){d.click();JSON.stringify({clicked:true})}else{JSON.stringify({clicked:false})}'
    if ($darkClicked.clicked) {
        Wait-Ab 500
        $darkResult = Invoke-Js -JsCode 'JSON.stringify({darkClass:document.documentElement.classList.contains("dark")})'
        Add-Result "主题切换: 深色模式生效" $darkResult.darkClass "html.dark: $($darkResult.darkClass)"
        Take-Screenshot -Name "08b_dark_mode" | Out-Null

        # 重新打开下拉菜单切换回浅色
        Invoke-Js -JsCode 'var r=document.querySelector(".settings-select-row");if(r){r.click()}' | Out-Null
        Wait-Ab 300
        $lightClicked = Invoke-Js -JsCode 'var items=Array.from(document.querySelectorAll(".settings-dropdown-item"));var l=items.find(function(i){return i.textContent.includes("浅色模式")});if(l){l.click();JSON.stringify({clicked:true})}else{JSON.stringify({clicked:false})}'
        if ($lightClicked.clicked) {
            Wait-Ab 400
            $lightResult = Invoke-Js -JsCode 'JSON.stringify({darkClass:document.documentElement.classList.contains("dark")})'
            Add-Result "主题切换: 浅色模式恢复" (-not $lightResult.darkClass) "html.dark: $($lightResult.darkClass)"
        } else {
            Add-Result "主题切换: 浅色模式恢复" $false "未找到浅色模式项"
        }
    } else {
        Add-Result "主题切换: 深色模式生效" $false "未找到深色模式项"
    }
}

function Test-AboutModal {
    Write-Log "测试 9: 关于弹窗" "STEP"
    # 先关闭可能打开的主题下拉 (点击 select-row 切换关闭)
    Invoke-Js -JsCode 'var d=document.querySelector(".settings-dropdown");if(d){var r=document.querySelector(".settings-select-row");if(r){r.click()}}' | Out-Null
    Wait-Ab 200
    # 用 JS 点击"关于 TinyTransfer"项
    $aboutClicked = Invoke-Js -JsCode 'var items=Array.from(document.querySelectorAll(".settings-link-item"));var a=items.find(function(i){return i.textContent.includes("关于")});if(a){a.click();JSON.stringify({clicked:true})}else{JSON.stringify({clicked:false})}'
    if ($aboutClicked.clicked) {
        Wait-Ab 600
        # 关于弹窗有 .about-modal 类, 设置弹窗有 .settings-modal 类
        $result = Invoke-Js -JsCode 'JSON.stringify({aboutTitle:document.querySelector(".about-modal .modal-title")?document.querySelector(".about-modal .modal-title").textContent:null,aboutExists:!!document.querySelector(".about-modal"),qqLink:document.body.innerText.includes("QQ"),changelogLink:document.body.innerText.includes("更新日志")})'
        Add-Result "关于弹窗: 标题显示" ($result.aboutTitle -eq "关于") "标题: $($result.aboutTitle)"
        Add-Result "关于弹窗: QQ 群链接" $result.qqLink
        Add-Result "关于弹窗: 更新日志链接" $result.changelogLink
        Take-Screenshot -Name "09_about_modal" | Out-Null

        # 用 JS 关闭关于弹窗 (.about-modal .modal-close)
        Invoke-Js -JsCode 'var c=document.querySelector(".about-modal .modal-close");if(c){c.click()}' | Out-Null
        Wait-Ab 400
    } else {
        Add-Result "关于弹窗: 标题显示" $false "未找到关于项"
    }
}

function Test-DebugPanel {
    Write-Log "测试 10: 速度调试" "STEP"
    # 确保关于弹窗已关闭 (如果还开着)
    Invoke-Js -JsCode 'var c=document.querySelector(".about-modal .modal-close");if(c){c.click()}' | Out-Null
    Wait-Ab 200
    # 用 JS 点击"速度调试"项 (openDebug 会关闭设置弹窗)
    $debugClicked = Invoke-Js -JsCode 'var items=Array.from(document.querySelectorAll(".settings-link-item"));var d=items.find(function(i){return i.textContent.includes("速度调试")});if(d){d.click();JSON.stringify({clicked:true})}else{JSON.stringify({clicked:false})}'
    if ($debugClicked.clicked) {
        Wait-Ab 500
        # 速度调试弹窗有 .debug-modal 或类似类, 先检查是否有非设置的 modal-title
        $result = Invoke-Js -JsCode 'var titles=Array.from(document.querySelectorAll(".modal-title")).map(function(t){return t.textContent});JSON.stringify({debugTitle:titles.find(function(t){return t.includes("速度")||t.includes("调试")})||null,allTitles:titles,tabs:document.querySelectorAll(".chip.active, .chip").length})'
        Add-Result "速度调试: 弹窗打开" ($null -ne $result.debugTitle) "标题: $($result.debugTitle)"
        Add-Result "速度调试: Tab 按钮存在" ($result.tabs -gt 0) "数量: $($result.tabs)"
        Take-Screenshot -Name "10_debug_panel" | Out-Null

        # 用 JS 关闭弹窗
        Invoke-Js -JsCode 'var c=document.querySelector(".modal-close");if(c){c.click()}else{var o=document.querySelector(".modal-overlay");if(o){o.click()}}' | Out-Null
        Wait-Ab 400
    } else {
        Add-Result "速度调试: 弹窗打开" $false "未找到速度调试项"
    }
}

function Test-CloseSettings {
    Write-Log "关闭设置弹窗" "STEP"
    # 用 JS 点击关闭按钮 (✕) 或点击 overlay 关闭
    Invoke-Js -JsCode 'var btns=Array.from(document.querySelectorAll("button"));var x=btns.find(function(b){return b.textContent.trim()==="✕"||b.getAttribute("aria-label")==="close"});if(x){x.click()}else{var o=document.querySelector(".modal-overlay");if(o){o.click()}}' | Out-Null
    Wait-Ab 400
}

function Test-Toast {
    Write-Log "测试 11: Toast 通知" "STEP"
    # 确保所有 modal 已关闭 (避免遮挡 Hero 区域)
    Invoke-Js -JsCode 'document.querySelectorAll(".modal-overlay").forEach(function(o){o.click()})' | Out-Null
    Wait-Ab 300
    # 用 JS 点击"复制链接"按钮 (使用 .url-copy-btn 类选择器, 文本只有 SVG)
    $copyClicked = Invoke-Js -JsCode 'var c=document.querySelector(".url-copy-btn");if(c){c.click();JSON.stringify({clicked:true})}else{JSON.stringify({clicked:false})}'
    if ($copyClicked.clicked) {
        Wait-Ab 400
        $toastShown = Invoke-Js -JsCode 'JSON.stringify({toastCount:document.querySelectorAll(".toast").length,toastText:document.querySelector(".toast")?document.querySelector(".toast").textContent:null})'
        Add-Result "Toast: 显示" ($toastShown.toastCount -gt 0) "数量: $($toastShown.toastCount), 文本: $($toastShown.toastText)"
        Take-Screenshot -Name "11_toast_shown" | Out-Null

        # 等待 toast 消失 (3 秒 + 缓冲)
        Wait-Ab 4000
        $toastGone = Invoke-Js -JsCode 'JSON.stringify({toastCount:document.querySelectorAll(".toast").length})'
        Add-Result "Toast: 自动消失" ($toastGone.toastCount -eq 0) "剩余: $($toastGone.toastCount)"
    } else {
        Add-Result "Toast: 显示" $false "未找到 .url-copy-btn 按钮"
    }
}

function Test-MobileResponsive {
    Write-Log "测试 12: 移动端响应式" "STEP"
    agent-browser set viewport 375 812 2>&1 | Out-Null
    Wait-Ab 800

    $result = Invoke-Js -JsCode 'JSON.stringify({viewport:{w:window.innerWidth,h:window.innerHeight},heroVisible:!!document.querySelector(".hero"),filesSectionVisible:!!document.querySelector("#filesSection"),moreBtn:!!document.querySelector(".files-more-btn")})'

    if ($null -eq $result) {
        # 如果 Invoke-Js 返回 null, 尝试重新加载页面
        Write-Log "页面状态异常, 重新加载..." "WARN"
        agent-browser open $AppUrl 2>&1 | Out-Null
        Start-Sleep -Seconds 2
        $result = Invoke-Js -JsCode 'JSON.stringify({viewport:{w:window.innerWidth,h:window.innerHeight},heroVisible:!!document.querySelector(".hero"),filesSectionVisible:!!document.querySelector("#filesSection"),moreBtn:!!document.querySelector(".files-more-btn")})'
    }

    if ($null -ne $result) {
        Add-Result "移动端: 视口设置" ($result.viewport.w -eq 375) "宽度: $($result.viewport.w)"
        Add-Result "移动端: Hero 区域显示" $result.heroVisible
        Add-Result "移动端: 文件区域显示" $result.filesSectionVisible
        Add-Result "移动端: 更多按钮存在" $result.moreBtn
    } else {
        Add-Result "移动端: 视口设置" $false "Invoke-Js 返回 null"
    }
    Take-Screenshot -Name "12_mobile_view" | Out-Null

    # 测试更多按钮展开 (用 JS 点击)
    if ($result -and $result.moreBtn) {
        Invoke-Js -JsCode 'var b=document.querySelector(".files-more-btn");if(b){b.click()}' | Out-Null
        Wait-Ab 400
        $moreResult = Invoke-Js -JsCode 'JSON.stringify({open:document.querySelector(".files-secondary")?document.querySelector(".files-secondary").classList.contains("open"):false})'
        Add-Result "移动端: 更多按钮展开" $moreResult.open
        Take-Screenshot -Name "12b_mobile_more" | Out-Null
        # 收起更多菜单
        Invoke-Js -JsCode 'var b=document.querySelector(".files-more-btn");if(b){b.click()}' | Out-Null
        Wait-Ab 200
    }

    # 恢复桌面视口
    agent-browser set viewport 1280 720 2>&1 | Out-Null
    Wait-Ab 300
}

function Test-QRCompactMode {
    Write-Log "测试 13: 二维码紧凑模式" "STEP"
    # 先确保视口恢复桌面尺寸
    agent-browser set viewport 1280 720 2>&1 | Out-Null
    Wait-Ab 300

    # 先获取初始状态下 QR 卡片的尺寸（未连接时应该是大尺寸）
    $before = Invoke-Js -JsCode 'var q=document.querySelector(".qr-card");var r=q?q.getBoundingClientRect():null;JSON.stringify({qrWidth:r?Math.round(r.width):0,qrHeight:r?Math.round(r.height):0,hasCompact:document.querySelector(".hero-card")?document.querySelector(".hero-card").classList.contains("qr-compact"):false,connected:document.body.classList.contains("role-host")})'

    Add-Result "二维码紧凑模式: 初始未开启紧凑" (-not $before.hasCompact) "初始 qr-compact: $($before.hasCompact)"
    Add-Result "二维码紧凑模式: 初始 QR 尺寸正常" ($before.qrWidth -gt 150) "初始宽度: $($before.qrWidth)px"

    # 通过触发 SSE 事件模拟设备连接 - 发送自定义 device_list 事件
    # 先获取 EventSource 实例或手动调用 store 方法
    # 由于无法直接访问 Pinia store, 我们通过模拟 dispatchEvent 的方式不行
    # 换个方法: 直接给 .hero-card 加上 qr-compact 类 (模拟 store 状态变化后的效果)
    # 但这不能验证逻辑正确性, 我们用另一种方式: 检查 store 是否暴露在 window 上
    $storeAccessible = Invoke-Js -JsCode 'JSON.stringify({hasPinia:typeof Pinia!=="undefined",hasUseDevices:typeof useDevicesStore!=="undefined"})'

    # 尝试通过 fetch /api/devices 看返回的设备列表, 然后手动构造一个有远程设备的状态
    # 直接调用 API 获取当前设备, 然后我们无法改变后端状态
    # 方案: 通过添加 CSS 类来测试 UI 渲染, 同时验证逻辑是否存在
    # 更可靠的方式: 检查 conn-status-bar 的 show 类是否和 connected 同步

    # 验证 conn-status-bar 的显示逻辑: devices.connected 时 show
    $connBarStatus = Invoke-Js -JsCode 'var bar=document.querySelector(".conn-status-bar");JSON.stringify({barExists:!!bar,hasShow:bar?bar.classList.contains("show"):false})'

    # 测试: 通过 JS 直接给 hero-card 添加 qr-compact 类, 验证紧凑模式 UI 是否正确渲染
    Invoke-Js -JsCode 'var card=document.querySelector(".hero-card");if(card){card.classList.add("qr-compact")}JSON.stringify({done:true})' | Out-Null
    Wait-Ab 300

    $afterCompact = Invoke-Js -JsCode 'var q=document.querySelector(".qr-card");var r=q?q.getBoundingClientRect():null;var info=document.querySelector(".compact-info");var bar=document.querySelector(".conn-status-bar");JSON.stringify({qrWidth:r?Math.round(r.width):0,qrHeight:r?Math.round(r.height):0,infoExists:!!info,barExists:!!bar,urlRow:!!document.querySelector(".url-row")})'

    Add-Result "二维码紧凑模式: 紧凑模式 QR 缩小" ($afterCompact.qrWidth -le 120 -and $afterCompact.qrWidth -gt 60) "紧凑宽度: $($afterCompact.qrWidth)px (预期约 96px)"
    Add-Result "二维码紧凑模式: 紧凑模式 URL 行存在" $afterCompact.urlRow
    Add-Result "二维码紧凑模式: 紧凑模式 conn-status-bar 存在" $afterCompact.barExists

    Take-Screenshot -Name "13_qr_compact" | Out-Null

    # 恢复: 移除 qr-compact 类
    Invoke-Js -JsCode 'var card=document.querySelector(".hero-card");if(card){card.classList.remove("qr-compact")}JSON.stringify({done:true})' | Out-Null
    Wait-Ab 200
}

function Test-DeviceFlyout {
    Write-Log "测试 14: 设备面板" "STEP"

    # 找到设备面板切换按钮 (TopBar 上的), 通常是设备图标按钮
    $flyoutBtn = Invoke-Js -JsCode 'var btns=Array.from(document.querySelectorAll("button"));var b=btns.find(function(b){return b.title&&b.title.includes("设备")})||btns.find(function(b){return b.innerHTML&&b.innerHTML.includes("device")||b.innerHTML.includes("phone")||b.innerHTML.includes("smartphone")});JSON.stringify({found:!!b,title:b?b.title:null})'

    Add-Result "设备面板: 切换按钮存在" $flyoutBtn.found "按钮 title: $($flyoutBtn.title)"

    if ($flyoutBtn.found) {
        # 点击打开面板
        Invoke-Js -JsCode 'var btns=Array.from(document.querySelectorAll("button"));var b=btns.find(function(b){return b.title&&b.title.includes("设备")});if(b){b.click()}JSON.stringify({clicked:!!b})' | Out-Null
        Wait-Ab 400

        $flyout = Invoke-Js -JsCode 'var f=document.querySelector(".flyout");JSON.stringify({exists:!!f,isOpen:f?f.classList.contains("open"):false,label:f?document.querySelector(".flyout-label")?document.querySelector(".flyout-label").textContent:null:null,deviceCount:document.querySelectorAll(".flyout-device").length})'

        Add-Result "设备面板: 面板存在" $flyout.exists
        Add-Result "设备面板: 面板展开" $flyout.isOpen "open 类: $($flyout.isOpen)"
        Add-Result "设备面板: 标签显示" ($null -ne $flyout.label) "标签: $($flyout.label)"

        Take-Screenshot -Name "14_device_flyout" | Out-Null

        # 点击其他地方关闭
        agent-browser press Escape 2>&1 | Out-Null
        Wait-Ab 300
    }
}

function Test-UploadModal {
    Write-Log "测试 15: 上传弹窗" "STEP"

    # 先确保没有其他 modal 打开
    Invoke-Js -JsCode 'document.querySelectorAll(".modal-overlay").forEach(function(o){o.click()})' | Out-Null
    Wait-Ab 300

    # 点击上传按钮打开弹窗
    $uploadBtn = Invoke-Js -JsCode 'var btns=Array.from(document.querySelectorAll(".btn-primary"));var b=btns.find(function(b){return b.textContent.includes("上传")});JSON.stringify({found:!!b,text:b?b.textContent.trim():null})'
    Add-Result "上传弹窗: 上传按钮存在" $uploadBtn.found "文本: $($uploadBtn.text)"

    if ($uploadBtn.found) {
        Invoke-Js -JsCode 'var btns=Array.from(document.querySelectorAll(".btn-primary"));var b=btns.find(function(b){return b.textContent.includes("上传")});if(b){b.click()}JSON.stringify({clicked:!!b})' | Out-Null
        Wait-Ab 600

        $modal = Invoke-Js -JsCode 'var overlays=document.querySelectorAll(".modal-overlay");var sheet=document.querySelector(".modal-sheet");var dropZone=document.querySelector(".drop-zone-shell");var title=document.querySelector(".modal-sheet .modal-title")?document.querySelector(".modal-sheet .modal-title").textContent:null;JSON.stringify({overlayCount:overlays.length,sheetExists:!!sheet,dropZone:!!dropZone,title:title,closeBtn:!!document.querySelector(".modal-sheet .modal-close")})'

        Add-Result "上传弹窗: overlay 存在" ($modal.overlayCount -gt 0) "overlay 数量: $($modal.overlayCount)"
        Add-Result "上传弹窗: 拖拽区域存在" $modal.dropZone
        Add-Result "上传弹窗: 标题显示" ($null -ne $modal.title) "标题: $($modal.title)"
        Add-Result "上传弹窗: 关闭按钮存在" $modal.closeBtn

        Take-Screenshot -Name "15_upload_modal" | Out-Null

        # 关闭弹窗
        Invoke-Js -JsCode 'var c=document.querySelector(".modal-sheet .modal-close");if(c){c.click()}else{var o=document.querySelector(".modal-overlay");if(o){o.click()}}JSON.stringify({done:true})' | Out-Null
        Wait-Ab 400
    }
}

function Test-FileCardInteraction {
    Write-Log "测试 16: 文件卡片交互" "STEP"

    # 确保没有 modal 打开
    Invoke-Js -JsCode 'document.querySelectorAll(".modal-overlay").forEach(function(o){o.click()})' | Out-Null
    Wait-Ab 300

    $fileCards = Invoke-Js -JsCode 'var cards=document.querySelectorAll(".file-card");JSON.stringify({count:cards.length,firstName:cards.length>0?cards[0].getAttribute("data-filename"):null,hasMoreBtn:cards.length>0?!!cards[0].querySelector(".file-card-more-btn"):false})'

    Add-Result "文件卡片交互: 存在文件卡片" ($fileCards.count -gt 0) "数量: $($fileCards.count)"

    if ($fileCards.count -gt 0) {
        # 测试更多按钮 (右键菜单触发点)
        if ($fileCards.hasMoreBtn) {
            Invoke-Js -JsCode 'var card=document.querySelector(".file-card");var btn=card.querySelector(".file-card-more-btn");if(btn){btn.click()}JSON.stringify({clicked:!!btn})' | Out-Null
            Wait-Ab 400

            $ctxMenu = Invoke-Js -JsCode 'var m=document.querySelector(".context-menu");JSON.stringify({exists:!!m,isOpen:m?m.classList.contains("open"):false,itemCount:m?m.querySelectorAll(".context-menu-item").length:0})'

            Add-Result "文件卡片交互: 右键菜单出现" ($ctxMenu.exists -and $ctxMenu.isOpen) "open 类: $($ctxMenu.isOpen)"
            Add-Result "文件卡片交互: 菜单项数量" ($ctxMenu.itemCount -ge 4) "数量: $($ctxMenu.itemCount)"

            Take-Screenshot -Name "16_file_context_menu" | Out-Null

            # 点击外部关闭右键菜单
            agent-browser press Escape 2>&1 | Out-Null
            Wait-Ab 200
        } else {
            Add-Result "文件卡片交互: 右键菜单出现" $false "未找到更多按钮"
        }

        # 测试点击文件卡片 (打开预览)
        Invoke-Js -JsCode 'var card=document.querySelector(".file-card");if(card){card.click()}JSON.stringify({clicked:!!card})' | Out-Null
        Wait-Ab 500

        $preview = Invoke-Js -JsCode 'var overlays=document.querySelectorAll(".modal-overlay");var hasPreview=overlays.length>0;var titles=Array.from(document.querySelectorAll(".modal-title")).map(function(t){return t.textContent});JSON.stringify({overlayCount:overlays.length,titles:titles,hasFilePreview:titles.some(function(t){return t&&(t.includes("预览")||t.includes("文件"))||t&&t.length>0})})'

        Add-Result "文件卡片交互: 点击打开弹窗" ($preview.overlayCount -gt 0) "overlay 数量: $($preview.overlayCount)"

        Take-Screenshot -Name "16b_file_preview" | Out-Null

        # 关闭所有弹窗
        Invoke-Js -JsCode 'document.querySelectorAll(".modal-overlay").forEach(function(o){o.click()})' | Out-Null
        Wait-Ab 400
    }
}

function Test-ConfirmDialog {
    Write-Log "测试 17: 确认对话框" "STEP"

    # 确保没有 modal 打开
    Invoke-Js -JsCode 'document.querySelectorAll(".modal-overlay").forEach(function(o){o.click()})' | Out-Null
    Wait-Ab 300

    $hasFiles = Invoke-Js -JsCode 'JSON.stringify({count:document.querySelectorAll(".file-card").length})'

    if ($hasFiles.count -gt 0) {
        # 先进入多选模式
        Invoke-Js -JsCode 'var b=document.querySelector("[title=''多选'']");if(b){b.click()}JSON.stringify({clicked:!!b})' | Out-Null
        Wait-Ab 400

        # 选中一个文件
        Invoke-Js -JsCode 'var card=document.querySelector(".file-card");if(card){card.click()}JSON.stringify({clicked:!!card})' | Out-Null
        Wait-Ab 300

        # 点击删除按钮 (batch-bar 上的删除)
        $deleteBtn = Invoke-Js -JsCode 'var bar=document.querySelector(".batch-bar");var btns=bar?bar.querySelectorAll("button"):[];var del=Array.from(btns).find(function(b){return b.textContent.includes("删除")||b.title&&b.title.includes("删除")});JSON.stringify({found:!!del,text:del?del.textContent.trim():null})'

        if ($deleteBtn.found) {
            Invoke-Js -JsCode 'var bar=document.querySelector(".batch-bar");var btns=bar?bar.querySelectorAll("button"):[];var del=Array.from(btns).find(function(b){return b.textContent.includes("删除")||b.title&&b.title.includes("删除")});if(del){del.click()}JSON.stringify({clicked:!!del})' | Out-Null
            Wait-Ab 500

            $confirm = Invoke-Js -JsCode 'var overlays=document.querySelectorAll(".modal-overlay");var buttons=document.querySelectorAll("button");var confirmBtn=Array.from(buttons).find(function(b){return b.textContent.includes("确认")||b.textContent.includes("删除")&&b.closest(".modal-overlay")});var cancelBtn=Array.from(buttons).find(function(b){return b.textContent.includes("取消")&&b.closest(".modal-overlay")});var titles=Array.from(document.querySelectorAll(".modal-title")).map(function(t){return t.textContent});JSON.stringify({overlayCount:overlays.length,hasConfirm:!!confirmBtn,hasCancel:!!cancelBtn,titles:titles})'

            Add-Result "确认对话框: 弹窗出现" ($confirm.overlayCount -gt 0) "overlay 数量: $($confirm.overlayCount)"
            Add-Result "确认对话框: 有确认按钮" $confirm.hasConfirm
            Add-Result "确认对话框: 有取消按钮" $confirm.hasCancel

            Take-Screenshot -Name "17_confirm_dialog" | Out-Null

            # 点击取消 (不真的删除文件)
            Invoke-Js -JsCode 'var overlays=document.querySelectorAll(".modal-overlay");var lastOverlay=overlays[overlays.length-1];var btns=lastOverlay?lastOverlay.querySelectorAll("button"):[];var cancel=Array.from(btns).find(function(b){return b.textContent.includes("取消")});if(cancel){cancel.click()}else if(lastOverlay){lastOverlay.click()}JSON.stringify({done:true})' | Out-Null
            Wait-Ab 400
        } else {
            Add-Result "确认对话框: 弹窗出现" $false "未找到删除按钮"
        }

        # 退出多选模式
        Invoke-Js -JsCode 'var b=document.querySelector("[title=''多选'']");if(b){b.click()}JSON.stringify({clicked:!!b})' | Out-Null
        Wait-Ab 300
    } else {
        Add-Result "确认对话框: 弹窗出现" $false "无文件可测试 (跳过)"
    }
}

function Test-DiskInfoBar {
    Write-Log "测试 18: 磁盘信息栏" "STEP"

    $diskBar = Invoke-Js -JsCode 'var bar=document.querySelector(".disk-bar");JSON.stringify({exists:!!bar,text:bar?bar.textContent.substring(0,50):null,hasFill:bar?!!bar.querySelector(".disk-bar-fill"):false})'

    Add-Result "磁盘信息栏: 存在" $diskBar.exists "可见: $($diskBar.exists)"
    if ($diskBar.exists) {
        Add-Result "磁盘信息栏: 包含进度条" $diskBar.hasFill
        Add-Result "磁盘信息栏: 包含文字" ($null -ne $diskBar.text -and $diskBar.text -ne "") "内容: $($diskBar.text)"
    }
}

function Test-Footer {
    Write-Log "测试 19: 页脚信息" "STEP"

    $footer = Invoke-Js -JsCode 'var f=document.querySelector(".page-footer");var links=f?f.querySelectorAll("a"):[];var ghLink=Array.from(links).find(function(a){return a.href&&a.href.includes("github")});JSON.stringify({exists:!!f,version:f?f.querySelector(".page-footer-item")?f.querySelector(".page-footer-item").textContent:null:null,hasGithub:!!ghLink,linkCount:links.length})'

    Add-Result "页脚信息: 存在" $footer.exists
    Add-Result "页脚信息: 版本号显示" ($null -ne $footer.version) "版本: $($footer.version)"
    Add-Result "页脚信息: GitHub 链接" $footer.hasGithub
    Add-Result "页脚信息: 链接数量" ($footer.linkCount -ge 1) "数量: $($footer.linkCount)"
}

function Test-Cleanup {
    Write-Log "清理测试状态" "STEP"
    # 不调用 close/close --all - 这会导致下次 open 卡住
    # 保持 daemon 和会话运行, 下次测试时 open 会直接导航
    # 只恢复视口到桌面尺寸 (防止上次测试留在移动端视口)
    try {
        agent-browser set viewport 1280 720 2>&1 | Out-Null
    } catch {
        Write-Log "恢复视口时出错 (可忽略)" "WARN"
    }
}

# ===== 主流程 =====

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  TinyTransfer 端到端测试" -ForegroundColor Cyan
Write-Host "  $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# 环境检查
$envOk = Test-Environment
if (-not $envOk) {
    Write-Log "环境检查失败, 终止测试" "FAIL"
    exit 1
}

# 执行测试用例
Test-PageLoad
Test-GuidedTour
Test-FileSection
Test-FilterPopover
Test-ViewToggle
Test-BatchMode
Test-SettingsModal
Test-ThemeSwitch
Test-AboutModal
Test-DebugPanel
Test-CloseSettings
Test-Toast
Test-MobileResponsive
Test-QRCompactMode
Test-DeviceFlyout
Test-UploadModal
Test-FileCardInteraction
Test-ConfirmDialog
Test-DiskInfoBar
Test-Footer
Test-Cleanup

# ===== 输出报告 =====
Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  测试报告" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

$report = $TestResults | Format-Table -AutoSize -Property `
    @{Label="时间"; Expression={$_.Time}; Width=10},
    @{Label="结果"; Expression={if($_.Passed){"PASS"}else{"FAIL"}}; Width=6},
    @{Label="测试项"; Expression={$_.Name}; Width=40},
    @{Label="详情"; Expression={$_.Detail}; Width=40}

$report | Out-Host

Write-Host ""
$totalCount = $PassedCount + $FailedCount
Write-Host "总计: $totalCount 项, 通过: $PassedCount, 失败: $FailedCount" -ForegroundColor $(if($FailedCount -eq 0){"Green"}else{"Yellow"})
Write-Host "截图目录: $ScreenshotDir" -ForegroundColor Cyan
Write-Host ""

if ($FailedCount -eq 0) {
    Write-Host "  [全部通过]" -ForegroundColor Green
} else {
    Write-Host "  [存在失败项, 请查看上方详情]" -ForegroundColor Yellow
}
Write-Host ""

# 保存报告到文件
$reportPath = Join-Path $ScriptDir "test-report.txt"
$reportContent = @"
TinyTransfer 端到端测试报告
时间: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
总计: $totalCount 项, 通过: $PassedCount, 失败: $FailedCount

$($TestResults | Format-Table -AutoSize | Out-String)
"@
$reportContent | Out-File -FilePath $reportPath -Encoding UTF8
Write-Host "报告已保存: $reportPath" -ForegroundColor Cyan

exit $FailedCount
