package com.cherryleafroad.kmagick

@Suppress("unused")
enum class ImageType(val id: Int) {
    UndefinedType(0),
    BilevelType(1),
    GrayscaleType(2),
    GrayscaleAlphaType(3),
    PaletteType(4),
    PaletteAlphaType(5),
    TrueColorType(6),
    TrueColorAlphaType(7),
    ColorSeparationType(8),
    ColorSeparationAlphaType(9),
    OptimizeType(10),
    PaletteBilevelAlphaType(11)
}
