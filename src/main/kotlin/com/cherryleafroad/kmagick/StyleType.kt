package com.cherryleafroad.kmagick

@Suppress("unused")
enum class StyleType(val id: Int) {
    UndefinedStyle(0),
    NormalStyle(1),
    ItalicStyle(2),
    ObliqueStyle(3),
    AnyStyle(4),
    BoldStyle(5);

    @Suppress("unused")
    companion object {
        @JvmName("fromNative")
        internal fun fromNative(id: Int): StyleType {
            return (StyleType::id::find)(id)!!
        }
    }
}
