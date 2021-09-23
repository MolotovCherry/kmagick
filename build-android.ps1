# run this in ps 7 or more
# options: arm aarch64 x86 x84_64 ; default: aarch64
param([String]$arch="aarch64", [switch]$release, [switch]$expand) 

$sep = if ($isWindows) {
    ";"
} else {
    ":"
}

$root = Resolve-Path -Path "$PSScriptRoot/.."

$content = Get-Content  -Path "$root/Application.mk"
$static = (Select-String -InputObject $content -Pattern "STATIC_BUILD\s+:=\s+([^\s]+)").Matches.Groups[1]
if ($static -eq "true") {
    $static = "1"
} else {
    $static = "0"
}

$imdir = Resolve-Path -Path "$root/ImageMagick-*"
$jnidir = "$root/jniLibs"
$includedir = $imdir
$imlibs = "magick-7"
$libdirs = ""

$dirs = Get-ChildItem -Directory -Path $jnidir
foreach ($d in $dirs) {
    if ($libdirs.Length -eq 0) {
        $libdirs += "$d"
    } else {
        $libdirs += "$sep$d"
    }
}

if ($arch -eq "aarch64") {
    $includearch = "arm64"
    $target = "aarch64-linux-android"
} elseif ($arch -eq "arm") {
    $includearch = "arm"
    $target = "armv7-linux-androideabi"
} elseif ($arch -eq "x86") {
    $includearch = "x86"
    $target = "i686-linux-android"
} elseif ($arch -eq "x86_64") {
    $includearch = "x86_64"
    $target = "x86_64-linux-android"
} else {
    $includearch = "arm64"
    $target = "aarch64-linux-android"
}

$IMAGE_MAGICK_DIR = $imdir
$IMAGE_MAGICK_LIBS = "magickwand-7${sep}magickcore-7"
$IMAGE_MAGICK_LIB_DIRS = $libdirs
$IMAGE_MAGICK_INCLUDE_DIRS = "$imdir$sep$imdir/configs/$includearch"
$IMAGE_MAGICK_STATIC = $static

if ($env:IMAGE_MAGICK_DIR -ne $IMAGE_MAGICK_DIR) {
    $env:IMAGE_MAGICK_DIR = $IMAGE_MAGICK_DIR
}
if ($env:IMAGE_MAGICK_LIBS -ne $IMAGE_MAGICK_LIBS) {
    $env:IMAGE_MAGICK_LIBS = $IMAGE_MAGICK_LIBS
}
if ($env:IMAGE_MAGICK_LIB_DIRS -ne $IMAGE_MAGICK_LIB_DIRS) {
    $env:IMAGE_MAGICK_LIB_DIRS = $IMAGE_MAGICK_LIB_DIRS
}
if ($env:IMAGE_MAGICK_INCLUDE_DIRS -ne $IMAGE_MAGICK_INCLUDE_DIRS) {
    $env:IMAGE_MAGICK_INCLUDE_DIRS = $IMAGE_MAGICK_INCLUDE_DIRS
}
if ($env:IMAGE_MAGICK_STATIC -ne $IMAGE_MAGICK_STATIC) {
    $env:IMAGE_MAGICK_STATIC = $IMAGE_MAGICK_STATIC
}

$flags = ""
if ($release) {
    $flags = "--release"
}


if (!$expand) {
    if ($release) {
      xargo build --color=always --target=$target $flags
    } else {
      cargo build --color=always --target=$target $flags
    }
} else {
    cargo expand --color=always --target=$target $flags
}
