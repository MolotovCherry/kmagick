package com.cherryleafroad.kmagick

/**
 * Retrieve the page geometry ([width], [height], [x] offset, [y] offset) of the image.
 */
data class PageGeometry(
    val width: Long,
    val height: Long,
    val x: Long,
    val y: Long
)
