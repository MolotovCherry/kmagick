package com.cherryleafroad.kmagick

/**
 * The LineCap enumerations specify shape to be used at the end of open subpaths when they are stroked.
 * See SVG's 'stroke-linecap' for examples.
 */
enum class LineCap(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedCap(0),

    /**
     * Square ending.
     */
    ButtCap(1),

    /**
     * Rounded ending (half-circle end with radius of 1/2 stroke width).
     */
    RoundCap(2),

    /**
     * Square ending, extended by 1/2 the stroke width at end.
     */
    SquareCap(3);

    internal companion object {
        fun fromNative(id: Int): LineCap {
            return (LineCap::id::find)(id)!!
        }
    }
}
