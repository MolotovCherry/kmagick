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

$IMAGE_MAGICK_DIR = $imdir
$IMAGE_MAGICK_LIBS = $imlibs
$IMAGE_MAGICK_LIB_DIRS = $libdirs
$IMAGE_MAGICK_INCLUDE_DIRS = $includedir
$IMAGE_MAGICK_STATIC = $staticVal

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
    #if ($release) {
    #  xargo build --target=$target $flags
    #} else {
      cargo build --target=$target -p kmagick-rs $flags
    #}
} else {
    cargo expand --target=$target -p kmagick-rs $flags
}
