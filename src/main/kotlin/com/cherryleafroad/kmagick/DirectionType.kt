package com.cherryleafroad.kmagick

@Suppress("unused")
enum class DirectionType(val id: Int) {
    UndefinedDirection(0),
    RightToLeftDirection(1),
    LeftToRightDirection(2);

    @Suppress("unused")
    companion object {
        @JvmName("fromNative")
        internal fun fromNative(id: Int): DirectionType {
            return (DirectionType::id::find)(id)!!
        }
    }
}
