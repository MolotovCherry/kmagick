package com.cherryleafroad.kmagick

/**
 * The DecorationType enumerations are used to specify line decorations of rendered text.
 */
enum class DecorationType(internal val id: Int) {
    /**
     * Unset
     */
    UndefinedDecoration(0),

    /**
     * No decoration
     */
    NoDecoration(1),

    /**
     * Underlined text
     */
    UnderlineDecoration(2),

    /**
     * Overlined text
     */
    OverlineDecoration(3),

    /**
     * Strike-through text
     */
    LineThroughDecoration(4);

    internal companion object {
        fun fromNative(id: Int): DecorationType {
            return (DecorationType::id::find)(id)!!
        }
    }
}
