package com.cherryleafroad.kmagick

/**
 * The wand type
 */
enum class WandType(internal val id: Int) {
    /**
     * PixelWand
     */
    PixelWand(0),

    /**
     * DrawingWand
     */
    DrawingWand(1),

    /**
     * MagickWand
     */
    MagickWand(2)
}
