package com.cherryleafroad.kmagick

@Suppress("unused")
enum class DecorationType(val id: Int) {
    UndefinedDecoration(0),
    NoDecoration(1),
    UnderlineDecoration(2),
    OverlineDecoration(3),
    LineThroughDecoration(4);

    @Suppress("unused")
    internal companion object {
        fun fromNative(id: Int): DecorationType {
            return (DecorationType::id::find)(id)!!
        }
    }
}
