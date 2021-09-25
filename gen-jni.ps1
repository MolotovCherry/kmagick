Set-Location -Path "$PSScriptRoot\kotlin\src"

$classes = @(
    "DrawingWand", "Magick", "MagickWand", "PixelWand"
)

$files = Get-ChildItem -Include "*.kt" -File -Recurse -Path . 
$ktFiles = Join-String -InputObject $files -Separator " "

kotlinc $ktFiles.Split(" ")

foreach ($c in $classes) {
    javah -jni -force -cp . -o "$c.h" "com.cherryleafroad.kmagick.$c"
}

Remove-Item -Recurse META-INF
$compiled_classes = Get-ChildItem -Include "*.class" -File -Recurse -Path .
foreach ($f in $compiled_classes) {
    Remove-Item $f
}
