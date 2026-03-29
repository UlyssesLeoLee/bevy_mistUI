[CmdletBinding()]
param(
    [ValidateSet("gallery", "rope")]
    [string]$Mode = "gallery",

    [switch]$Release,

    [switch]$Detached,

    [switch]$VisualMock,

    [switch]$EnableWgpuValidation
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = (Resolve-Path (Join-Path $scriptDir "..\..")).Path

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "cargo is not installed or not available on PATH."
}

function Disable-VulkanValidationNoise {
    foreach ($name in @(
        "VK_LAYER_PATH",
        "VK_INSTANCE_LAYERS",
        "VK_LOADER_LAYERS_ENABLE"
    )) {
        if (Test-Path "Env:$name") {
            Remove-Item "Env:$name"
        }
    }

    $env:ROPE_CLIENT_ENABLE_WGPU_VALIDATION = "0"
    $env:BEVY_MISTUI_ENABLE_WGPU_VALIDATION = "0"
    $env:WGPU_VALIDATION = "0"
    $env:WGPU_DEBUG = "0"
}

function Enable-VulkanValidationFlags {
    $env:ROPE_CLIENT_ENABLE_WGPU_VALIDATION = "1"
    $env:BEVY_MISTUI_ENABLE_WGPU_VALIDATION = "1"
}

$cargoArgs = switch ($Mode) {
    "gallery" {
        @(
            "run",
            "--manifest-path", "plugins/bevy_mistUI/Cargo.toml",
            "--example", "bevy_mistUI_gallery"
        )
    }
    "rope" {
        @(
            "run",
            "--manifest-path", "client/bevy/Cargo.toml",
            "-p", "rope_bevy",
            "--bin", "ui_smoke_debug"
        )
    }
}

if ($Release) {
    $cargoArgs += "--release"
}

if ($VisualMock) {
    if ($Mode -eq "gallery") {
        $cargoArgs += "--"
        $cargoArgs += "--visual-mock"
    } else {
        Write-Host "[bevy_mistUI] VisualMock only applies to gallery mode; ignoring it for rope mode."
    }
}

if ($EnableWgpuValidation) {
    Enable-VulkanValidationFlags
} else {
    Disable-VulkanValidationNoise
}

Write-Host ("[bevy_mistUI] mode={0} visual_mock={1} validation={2} repo={3}" -f $Mode, $VisualMock.IsPresent, $EnableWgpuValidation.IsPresent, $repoRoot)

Push-Location $repoRoot
try {
    if ($Detached) {
        Start-Process cargo -ArgumentList $cargoArgs -WorkingDirectory $repoRoot | Out-Null
        Write-Host "[bevy_mistUI] Debug scene launched in the background."
    } else {
        & cargo @cargoArgs
        if ($LASTEXITCODE -ne 0) {
            exit $LASTEXITCODE
        }
    }
} finally {
    Pop-Location
}
