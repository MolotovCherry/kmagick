package com.cherryleafroad.kmagick

/**
 * [OrientationType] specifies the orientation of the image. Useful for when the image is produced via a different
 * ordinate system, the camera was turned on its side, or the page was scanned sideways.
 */
enum class OrientationType(internal val id: Int) {
    /**
     * Scanline Direction: Unknown
     *
     * Frame Direction: Unknown
     */
    UndefinedOrientation(0),

    /**
     * Scanline Direction: Left to right
     *
     * Frame Direction: Top to bottom
     */
    TopLeftOrientation(1),

    /**
     * Scanline Direction: Right to left
     *
     * Frame Direction: Top to bottom
     */
    TopRightOrientation(2),

    /**
     * Scanline Direction: Right to left
     *
     * Frame Direction: Bottom to top
     */
    BottomRightOrientation(3),

    /**
     * Scanline Direction: Left to right
     *
     * Frame Direction: Bottom to top
     */
    BottomLeftOrientation(4),

    /**
     * Scanline Direction: Top to bottom
     *
     * Frame Direction: Left to right
     */
    LeftTopOrientation(5),

    /**
     * Scanline Direction: Top to bottom
     *
     * Frame Direction: Right to left
     */
    RightTopOrientation(6),

    /**
     * Scanline Direction: Bottom to top
     *
     * Frame Direction: Right to left
     */
    RightBottomOrientation(7),

    /**
     * Scanline Direction: Bottom to top
     *
     * Frame Direction: Left to right
     */
    LeftBottomOrientation(8)
}
