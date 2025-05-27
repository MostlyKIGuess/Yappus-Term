#Requires -Version 5.0

# Yappus Terminal Windows Installer
# This script installs Yappus Terminal on Windows systems

Write-Host "Yappus Terminal Windows Installer" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan

# admin check
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

$appDir = "$env:USERPROFILE\Yappus-Term"
$configDir = "$env:APPDATA\yappus\yappus-term\config"

# rust installation
function Test-RustInstalled {
    try {
        $rustVersion = (rustc --version)
        $cargoVersion = (cargo --version)
        return $true
    }
    catch {
        return $false
    }
}

# rust install
if (-not (Test-RustInstalled)) {
    Write-Host "Rust is not installed. Installing Rust..." -ForegroundColor Yellow
    
    try {
        # Download rustup-init
        $rustupInitPath = "$env:TEMP\rustup-init.exe"
        Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupInitPath
        
        # Run rustup-init
        & $rustupInitPath -y --default-toolchain stable
        
        # Update PATH for current session
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path", "User") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "Machine")
        
        Write-Host "Rust installed successfully!" -ForegroundColor Green
    }
    catch {
        Write-Host "Failed to install Rust. Please install it manually from https://rustup.rs/" -ForegroundColor Red
        exit 1
    }
}
else {
    Write-Host "Rust is already installed." -ForegroundColor Green
}

Write-Host "Setting up directories..." -ForegroundColor Cyan
New-Item -ItemType Directory -Path $appDir -Force | Out-Null
New-Item -ItemType Directory -Path $configDir -Force | Out-Null

Write-Host "Cloning Yappus Terminal repository..." -ForegroundColor Cyan
Set-Location $appDir
try {
    if (Test-Path "$appDir\.git") {
        git pull
    }
    else {
        git clone https://github.com/MostlyKIGuess/Yappus-Term.git .
    }
}
catch {
    Write-Host "Failed to clone repository. Make sure Git is installed." -ForegroundColor Red
    Write-Host "You can install Git from https://git-scm.com/download/win" -ForegroundColor Red
    exit 1
}

Write-Host "Building Yappus Terminal..." -ForegroundColor Cyan
try {
    cargo build --release
}
catch {
    Write-Host "Failed to build Yappus Terminal. Please check the error messages above." -ForegroundColor Red
    exit 1
}

$targetPath = "$appDir\target\release\yappus.exe"
$addToPath = $false

if ($isAdmin) {
    $addToPath = (Read-Host "Do you want to add Yappus to your system PATH? (y/N)").ToLower() -eq 'y'
    
    if ($addToPath) {
        $pathEnv = [System.Environment]::GetEnvironmentVariable("Path", "Machine")
        if ($pathEnv -notlike "*$appDir\target\release*") {
            [System.Environment]::SetEnvironmentVariable("Path", "$pathEnv;$appDir\target\release", "Machine")
            Write-Host "Added Yappus to system PATH." -ForegroundColor Green
        }
    }
}
elseif ((Read-Host "Do you want to add Yappus to your user PATH? (y/N)").ToLower() -eq 'y') {
    $pathEnv = [System.Environment]::GetEnvironmentVariable("Path", "User")
    if ($pathEnv -notlike "*$appDir\target\release*") {
        [System.Environment]::SetEnvironmentVariable("Path", "$pathEnv;$appDir\target\release", "User")
        Write-Host "Added Yappus to user PATH." -ForegroundColor Green
    }
}

$createShortcut = (Read-Host "Do you want to create a desktop shortcut? (y/N)").ToLower() -eq 'y'
if ($createShortcut) {
    $desktopPath = [System.Environment]::GetFolderPath('Desktop')
    $shortcutPath = Join-Path -Path $desktopPath -ChildPath "Yappus Terminal.lnk"
    
    $WScriptShell = New-Object -ComObject WScript.Shell
    $shortcut = $WScriptShell.CreateShortcut($shortcutPath)
    $shortcut.TargetPath = $targetPath
    $shortcut.WorkingDirectory = Split-Path -Parent $targetPath
    $shortcut.Description = "Yappus Terminal AI Assistant"
    $shortcut.Save()
    
    Write-Host "Desktop shortcut created." -ForegroundColor Green
}

Write-Host "`nYappus Terminal has been successfully installed!" -ForegroundColor Green
Write-Host "`nTo run Yappus Terminal:" -ForegroundColor Cyan
Write-Host " - From any terminal: yappus" -ForegroundColor White
Write-Host " - Or run directly: $targetPath" -ForegroundColor White
Write-Host "`nOn first run, you will be prompted to enter your Google Gemini API key." -ForegroundColor Yellow
Write-Host "You can get your API key from: https://aistudio.google.com/app/apikey" -ForegroundColor Yellow

if ((Read-Host "`nDo you want to run Yappus Terminal now? (y/N)").ToLower() -eq 'y') {
    Set-Location $env:USERPROFILE
    & $targetPath
}
