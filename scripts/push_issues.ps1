$ErrorActionPreference = "Stop"
$files = Get-ChildItem -Path docs\issue\*.md

foreach ($file in $files) {
    Write-Host "Processing $($file.Name)..."
    $content = Get-Content $file.FullName
    
    # Extract title
    $title = ""
    $titleLine = $content | Where-Object { $_ -match "^title:\s*`"(.*)`"" } | Select-Object -First 1
    if ($titleLine -match "^title:\s*`"(.*)`"") {
        $title = $matches[1]
    }
    
    # Extract labels
    $labels = ""
    $labelsLine = $content | Where-Object { $_ -match "^labels:\s*\[(.*)\]" } | Select-Object -First 1
    if ($labelsLine -match "^labels:\s*\[(.*)\]") {
        $labels = $matches[1] -replace '["\s]', ''
    }
    
    # Extract body (skip frontmatter)
    $endFrontmatterIdx = 0
    for ($i = 1; $i -lt $content.Length; $i++) {
        if ($content[$i] -eq "---") {
            $endFrontmatterIdx = $i
            break
        }
    }
    
    $body = $content[($endFrontmatterIdx + 1)..($content.Length - 1)] -join "`n"
    
    $tempFile = "$($file.FullName).tmp"
    Set-Content -Path $tempFile -Value $body -Encoding UTF8
    
    Write-Host "Pushing issue: $title with labels: $labels"
    gh issue create --title "$title" --label "$labels" --body-file "$tempFile"
    
    Remove-Item -Path $tempFile
}

Write-Host "All issues pushed successfully!"
