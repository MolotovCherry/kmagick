package com.cherryleafroad.kmagick

/**
 * By default, ImageMagick defines resolutions in pixels per inch. [ResolutionType] provides a means to adjust this.
 */
enum class ResolutionType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedResolution(0),

    /**
     * Density specifications are specified in units of pixels per inch (english units).
     */
    PixelsPerInchResolution(1),

    /**
     * Density specifications are specified in units of pixels per centimeter (metric units).
     */
    PixelsPerCentimeterResolution(2)
}
