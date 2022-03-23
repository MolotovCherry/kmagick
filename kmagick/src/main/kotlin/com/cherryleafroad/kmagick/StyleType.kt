package com.cherryleafroad.kmagick

/**
 * The [StyleType] enumerations are used to specify the style (e.g. Italic) of a font. If the style is not important,
 * the [AnyStyle] enumeration may be specified for a wildcard match.
 */
enum class StyleType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedStyle(0),

    /**
     * Normal font style
     */
    NormalStyle(1),

    /**
     *
    Italic font style
     */
    ItalicStyle(2),

    /**
     * Oblique font style
     */
    ObliqueStyle(3),

    /**
     * Wildcard match for font style
     */
    AnyStyle(4),

    /**
     * Bold font style
     */
    BoldStyle(5);

    internal companion object {
        fun fromNative(id: Int): StyleType {
            return (StyleType::id::find)(id)!!
        }
    }
}
