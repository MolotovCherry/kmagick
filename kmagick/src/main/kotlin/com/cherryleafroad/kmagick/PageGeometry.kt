package com.cherryleafroad.kmagick

/**
 * Retrieve the page geometry ([width], [height], [x] offset, [y] offset) of the image.
 */
data class PageGeometry(
    /**
     * The page geometry's [width]
     */
    val width: Long,

    /**
     * The page geometry's [height]
     */
    val height: Long,

    /**
     * The page geometry's [x] offset
     */
    val x: Long,

    /**
     * The page geometry's [y] offset
     */
    val y: Long
)
