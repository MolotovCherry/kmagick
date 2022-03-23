package com.cherryleafroad.kmagick

/**
 * The LineJoin enumerations specify the shape to be used at the corners of paths or basic shapes when they are
 * stroked. See SVG's 'stroke-linejoin' for examples.
 */
enum class LineJoin(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedJoin(0),

    /**
     * Sharp-edged join
     */
    MiterJoin(1),

    /**
     * Rounded-edged join
     */
    RoundJoin(2),

    /**
     * Beveled-edged join
     */
    BevelJoin(3);

    internal companion object {
        fun fromNative(id: Int): LineJoin {
            return (LineJoin::id::find)(id)!!
        }
    }
}
