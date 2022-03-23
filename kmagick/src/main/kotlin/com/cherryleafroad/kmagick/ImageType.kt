package com.cherryleafroad.kmagick

/**
 * ImageType indicates the type classification of the image.
 */
enum class ImageType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedType(0),

    /**
     * Monochrome image
     */
    BilevelType(1),

    /**
     * Grayscale image
     */
    GrayscaleType(2),

    /**
     * Grayscale image with opacity
     */
    GrayscaleAlphaType(3),

    /**
     * Indexed color (palette) image
     */
    PaletteType(4),

    /**
     * Indexed color (palette) image with opacity
     */
    PaletteAlphaType(5),

    /**
     * Truecolor image
     */
    TrueColorType(6),

    /**
     * Truecolor image with opacity
     */
    TrueColorAlphaType(7),

    /**
     * Cyan/Yellow/Magenta/Black (CYMK) image
     */
    ColorSeparationType(8),

    ColorSeparationAlphaType(9),

    OptimizeType(10),

    PaletteBilevelAlphaType(11)
}
