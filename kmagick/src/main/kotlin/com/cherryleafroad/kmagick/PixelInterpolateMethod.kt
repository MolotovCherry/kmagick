package com.cherryleafroad.kmagick

/**
 * Set the pixel color interpolation method to use when looking up a color based on a floating point or real value.
 *
 * &nbsp;
 *
 * When looking up the color of a pixel using a non-integer floating point value, you typically fall in between the pixel
 * colors defined by the source image. This setting determines how the color is determined from the colors of the pixels
 * surrounding that point. That is how to determine the color of a point that falls between two, or even four different
 * colored pixels.
 */
enum class PixelInterpolateMethod(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedInterpolatePixel(0),

    /**
     * The average color of the surrounding four pixels
     */
    AverageInterpolatePixel(1),

    /**
     * The average color of the surrounding nine pixels
     */
    Average9InterpolatePixel(2),

    /**
     * The average color of the surrounding sixteen pixels
     */
    Average16InterpolatePixel(3),
    BackgroundInterpolatePixel(4),

    /**
     * A double linear interpolation of pixels (the default)
     */
    BilinearInterpolatePixel(5),
    BlendInterpolatePixel(6),

    /**
     * Fitted bicubic-spines of surrounding 16 pixels
     */
    CatromInterpolatePixel(7),

    /**
     * The color of the top-left pixel (floor function)
     */
    IntegerInterpolatePixel(8),

    /**
     * Divide area into two flat triangular interpolations
     */
    MeshInterpolatePixel(9),

    /**
     * The nearest pixel to the lookup point (rounded function)
     */
    NearestInterpolatePixel(10),

    /**
     * Direct spline curves (colors are blurred)
     */
    SplineInterpolatePixel(11)
}
