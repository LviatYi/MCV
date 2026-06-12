$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Split-Path -Parent $scriptDir
$skillsTarget = Join-Path $repoRoot 'skills'
$codexDir = Join-Path $env:USERPROFILE '.codex'
$skillsLink = Join-Path $codexDir 'skills'

function Test-SamePath {
    param(
        [Parameter(Mandatory = $true)][string]$Left,
        [Parameter(Mandatory = $true)][string]$Right
    )

    $leftFull = [System.IO.Path]::GetFullPath($Left).TrimEnd('\')
    $rightFull = [System.IO.Path]::GetFullPath($Right).TrimEnd('\')
    return [string]::Equals($leftFull, $rightFull, [System.StringComparison]::OrdinalIgnoreCase)
}

function Get-BackupPath {
    param(
        [Parameter(Mandatory = $true)][string]$Directory,
        [Parameter(Mandatory = $true)][string]$Name
    )

    $index = 1
    do {
        $candidateName = "$Name.backup$index"
        $candidatePath = Join-Path $Directory $candidateName
        $index += 1
    } while (Test-Path -LiteralPath $candidatePath)

    return $candidatePath
}

if (-not (Test-Path -LiteralPath $skillsTarget -PathType Container)) {
    New-Item -ItemType Directory -Path $skillsTarget -Force | Out-Null
}

New-Item -ItemType Directory -Path $codexDir -Force | Out-Null

if (Test-Path -LiteralPath $skillsLink) {
    $existing = Get-Item -LiteralPath $skillsLink -Force

    if ($existing.LinkType) {
        $targets = @($existing.Target)
        foreach ($target in $targets) {
            if ($target -and (Test-SamePath -Left $target -Right $skillsTarget)) {
                Write-Host "Codex skills symlink already exists: $skillsLink -> $skillsTarget"
                exit 0
            }
        }

        Remove-Item -LiteralPath $skillsLink -Force
    }
    elseif ($existing.PSIsContainer) {
        Get-ChildItem -LiteralPath $skillsLink -Force | ForEach-Object {
            $destination = Join-Path $skillsTarget $_.Name

            if (Test-Path -LiteralPath $destination) {
                $backupPath = Get-BackupPath -Directory $skillsTarget -Name $_.Name
                $backupName = Split-Path -Leaf $backupPath
                Write-Warning "Skill '$($_.Name)' already exists in repository skills. Moving user copy as '$backupName'."
                $destination = $backupPath
            }

            Move-Item -LiteralPath $_.FullName -Destination $destination
        }

        Remove-Item -LiteralPath $skillsLink -Force
    }
    else {
        throw "Path already exists and is not a directory or symlink: $skillsLink"
    }
}

New-Item -ItemType SymbolicLink -Path $skillsLink -Target $skillsTarget | Out-Null
Write-Host "Created Codex skills symlink: $skillsLink -> $skillsTarget"
