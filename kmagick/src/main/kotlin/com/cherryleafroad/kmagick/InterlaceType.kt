package com.cherryleafroad.kmagick

/**
 * InterlaceType specifies the ordering of the red, green, and blue pixel information in the image.
 * Interlacing is usually used to make image information available to the user faster by taking advantage of the space
 * vs time tradeoff. For example, interlacing allows images on the Web to be recognizable sooner and satellite images
 * to accumulate/render with image resolution increasing over time.
 *
 * &nbsp;
 *
 * Use [LineInterlace] or [PlaneInterlace] to create an interlaced GIF or progressive JPEG image.
 */
enum class InterlaceType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedInterlace(0),

    /**
     * Don't interlace image (RGBRGBRGBRGBRGBRGB...)
     */
    NoInterlace(1),

    /**
     * Use scanline interlacing (RRR...GGG...BBB...RRR...GGG...BBB...)
     */
    LineInterlace(2),

    /**
     * Use plane interlacing (RRRRRR...GGGGGG...BBBBBB...)
     */
    PlaneInterlace(3),

    /**
     * Similar to plane interlaing except that the different planes are saved to individual files (e.g. image.R, image.G, and image.B)
     */
    PartitionInterlace(4),
    GIFInterlace(5),
    JPEGInterlace(6),
    PNGInterlace(7)
}
