package com.cherryleafroad.kmagick

enum class ClipPathUnits(internal val id: Int) {
    UndefinedPathUnits(0),
    UserSpace(1),
    UserSpaceOnUse(2),
    ObjectBoundingBox(3);

    internal companion object {
        fun fromNative(id: Int): ClipPathUnits {
            return (ClipPathUnits::id::find)(id)!!
        }
    }
}
