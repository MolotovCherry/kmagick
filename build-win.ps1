# run this in ps 7 or more
# options: x86 x84_64 ; default: x86_64
param([String]$arch="x86_64", [switch]$static, [switch]$release, [switch]$expand)

$imdir = Resolve-Path -Path "C:/Program Files/ImageMagick-*"
$includedir = "$imdir\include"
$imlibs = "CORE_RL_MagickWand_;CORE_RL_MagickCore_"

if ($arch -eq "x86") {
    $target = "i686-pc-windows-msvc"
} elseif ($arch -eq "x86_64") {
    $target = "x86_64-pc-windows-msvc"
} else {
    $target = "x86_64-pc-windows-msvc"
}

$libdirs = "$imdir;$imdir\lib"
$staticVal = "0"
if ($static) {
    $staticVal = "0"
} else {
    $staticVal = "1"
}

$env:IMAGE_MAGICK_DIR = $imdir
$env:IMAGE_MAGICK_LIBS = $imlibs
$env:IMAGE_MAGICK_LIB_DIRS = $libdirs
$env:IMAGE_MAGICK_INCLUDE_DIRS = $includedir
$env:IMAGE_MAGICK_STATIC = $staticVal

$flags = ""
if ($release) {
    $flags = "--release"
}

if (!$expand) {
    if ($release) {
      xargo build --target=$target $flags
    } else {
      cargo build --target=$target $flags
    }
} else {
    cargo expand --target=$target $flags
}
