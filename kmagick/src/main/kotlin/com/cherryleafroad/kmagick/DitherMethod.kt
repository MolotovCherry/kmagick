package com.cherryleafroad.kmagick

/**
 * Apply a Riemersma or Floyd-Steinberg error diffusion dither to images when general color reduction is applied via an
 * option, or automagically when saving to specific formats. This enabled by default.
 *
 * &nbsp;
 *
 * Dithering places two or more colors in neighboring pixels so that to the eye a closer approximation of the images
 * original color is reproduced. This reduces the number of colors needed to reproduce the image but at the cost of a
 * lower level pattern of colors. Error diffusion dithers can use any set of colors (generated or user defined) to an image.
 */
enum class DitherMethod(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedDitherMethod(0),

    /**
     * Don't use any dither method.
     */
    NoDitherMethod(1),

    /**
     * Riemersma dither is a novel dithering algorithm that can reduce a grey scale or color image to any color map
     * (also called a palette) and that restricts the influence of a dithered pixel to a small area around it.
     */
    RiemersmaDitherMethod(2),

    /**
     * FloydSteinberg dither reduces an image into a reduced set of colors while attempting to minimize perceptual changes.
     */
    FloydSteinbergDitherMethod(3);

    internal companion object {
        fun fromNative(id: Int): ClipPathUnits {
            return (ClipPathUnits::id::find)(id)!!
        }
    }
}
