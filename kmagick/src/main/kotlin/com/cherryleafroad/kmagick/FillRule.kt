package com.cherryleafroad.kmagick

/**
 * FillRule specifies the algorithm which is to be used to determine what parts of the canvas are included inside the
 * shape. See the documentation on SVG's fill-rule property for usage details.
 */
enum class FillRule(internal val id: Int) {
    /**
     * Fill rule not specified
     */
    UndefinedRule(0),

    /**
     * See SVG fill-rule evenodd rule.
     */
    EvenOddRule(1),

    /**
     * See SVG fill-rule nonzero rule.
     */
    NonZeroRule(2);

    internal companion object {
        fun fromNative(id: Int): FillRule {
            return (FillRule::id::find)(id)!!
        }
    }
}
