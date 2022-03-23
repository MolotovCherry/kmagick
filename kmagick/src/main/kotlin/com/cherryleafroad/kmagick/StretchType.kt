package com.cherryleafroad.kmagick

/**
 * The [StretchType] enumerations are used to specify the relative width of a font to the regular width
 * for the font family. If the width is not important, the [AnyStretch] enumeration may be specified for a wildcard match.
 */
enum class StretchType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedStretch(0),

    /**
     * Normal width font
     */
    NormalStretch(1),

    /**
     * Ultra-condensed (narrowest) font
     */
    UltraCondensedStretch(2),

    /**
     * Extra-condensed font
     */
    ExtraCondensedStretch(3),

    /**
     * Condensed font
     */
    CondensedStretch(4),

    /**
     * Semi-Condensed font
     */
    SemiCondensedStretch(5),

    /**
     * Semi-Expanded font
     */
    SemiExpandedStretch(6),

    /**
     * Expanded font
     */
    ExpandedStretch(7),

    /**
     * Extra-Expanded font
     */
    ExtraExpandedStretch(8),

    /**
     * Ultra-expanded (widest) font
     */
    UltraExpandedStretch(9),

    /**
     * Wildcard match for font stretch
     */
    AnyStretch(10);

    internal companion object {
        fun fromNative(id: Int): StretchType {
            return (StretchType::id::find)(id)!!
        }
    }
}
