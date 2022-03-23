package com.cherryleafroad.kmagick

enum class DirectionType(internal val id: Int) {
    UndefinedDirection(0),
    RightToLeftDirection(1),
    LeftToRightDirection(2);

    internal companion object {
        fun fromNative(id: Int): DirectionType {
            return (DirectionType::id::find)(id)!!
        }
    }
}
