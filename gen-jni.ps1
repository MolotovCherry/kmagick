Set-Location -Path "$PSScriptRoot\kotlin\src"

$files = Get-ChildItem -Include "*.kt" -File -Recurse -Path . 
$ktFiles = Join-String -InputObject $files -Separator " "

kotlinc $ktFiles.Split(" ")

foreach ($f in $files) {
    javah -jni -force -cp . com.cherryleafroad.jmagick.$($f.BaseName)
}

Remove-Item -Recurse META-INF
$classes = Get-ChildItem -Include "*.class" -File -Recurse -Path .
foreach ($f in $classes) {
    Remove-Item $f
}
