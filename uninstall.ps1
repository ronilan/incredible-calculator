#
# uninstall.ps1 — remove the `calculator` command that install.ps1 put on the
# system. Designed to be run standalone — download and run.
#
# Usage:
#   powershell -ExecutionPolicy Bypass -File uninstall.ps1
#   powershell -ExecutionPolicy Bypass -File uninstall.ps1 -CommandName <name>
#
# Self-elevates (UAC prompt) when not run as administrator.

param(
    # Optional command-name override, for an install made under another name.
    [string]$CommandName
)

$ErrorActionPreference = "Stop"

# --- The only project-specific setting ---------------------------------------
# The GitHub repository (owner/name) the installer uses. Its name also picks
# the folder under C:\Program Files (must match install.ps1).
$DefaultRepo = "ronilan/calculator"
# ------------------------------------------------------------------------------

$folderName = ($DefaultRepo -split '/')[-1]
$Dest = Join-Path $env:ProgramFiles $folderName
$command = if ($CommandName) { $CommandName } else { "calculator" }

# --- 1. Self-elevate if not running as administrator -------------------------
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()
           ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "Requesting administrator rights..."
    if ($PSCommandPath) {
        $relaunchArgs = if ($CommandName) { "-ExecutionPolicy Bypass -File `"$PSCommandPath`" -CommandName $CommandName" }
                        else             { "-ExecutionPolicy Bypass -File `"$PSCommandPath`"" }
        Start-Process powershell $relaunchArgs -Verb RunAs
        exit
    }
    # Piped straight into iex (no file on disk): re-download to temp and
    # relaunch that copy elevated.
    $tmpScript = Join-Path ([System.IO.Path]::GetTempPath()) "uninstall.ps1"
    Invoke-WebRequest -Uri "https://raw.githubusercontent.com/$DefaultRepo/main/uninstall.ps1" -OutFile $tmpScript
    $relaunchArgs = if ($CommandName) { "-ExecutionPolicy Bypass -File `"$tmpScript`" -CommandName $CommandName" }
                    else              { "-ExecutionPolicy Bypass -File `"$tmpScript`"" }
    Start-Process powershell $relaunchArgs -Verb RunAs
    exit
}

# --- 2. Remove the command ----------------------------------------------------
$exe = Join-Path $Dest "$command.exe"
if (Test-Path $exe) {
    Remove-Item $exe -Force
    Write-Host "Removed: $exe"
} else {
    Write-Host "Nothing to do: $exe does not exist."
}

# --- 3. Remove the install folder if it is now empty --------------------------
if ((Test-Path $Dest) -and -not (Get-ChildItem $Dest -ErrorAction SilentlyContinue)) {
    Remove-Item $Dest -Force
    Write-Host "Removed empty folder: $Dest"
}

# --- 4. Remove the folder from the system PATH (only if present) --------------
$machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
if (($machinePath -split ';') -contains $Dest) {
    $newPath = (($machinePath -split ';') | Where-Object { $_ -ne $Dest }) -join ';'
    [Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")
    Write-Host "Removed $Dest from the system PATH."
}
