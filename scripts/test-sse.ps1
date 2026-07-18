# SSE 连接测试：用 TcpClient 直接验证
$ErrorActionPreference = "Stop"
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

    Write-Host "SSE Response (first $read bytes):"
    Write-Host $response

    if ($response -match "HTTP/1.1 200" -and $response -match "text/event-stream") {
        Write-Host ""
        Write-Host "SSE: PASS" -ForegroundColor Green
    } else {
        Write-Host ""
        Write-Host "SSE: FAIL (unexpected response)" -ForegroundColor Red
    }

    $tcp.Close()
} catch {
    Write-Host "SSE Error:" $_.Exception.Message
    Write-Host "SSE: FAIL" -ForegroundColor Red
}
