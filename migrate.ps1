while ($true) {
    try {
        sqlx migrate run --source adapter/migrations
        break  # 成功したらループを抜ける
    } catch {
        Write-Host "Migration failed. Retrying in 1 second..."
        Start-Sleep -Seconds 1
    }
}