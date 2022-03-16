package com.cherryleafroad.kmagick

@Suppress("unused")
enum class LineCap(val id: Int) {
    UndefinedCap(0),
    ButtCap(1),
    RoundCap(2),
    SquareCap(3);

    @Suppress("unused")
    internal companion object {
        fun fromNative(id: Int): LineCap {
            return (LineCap::id::find)(id)!!
        }
    }
}