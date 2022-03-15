package com.cherryleafroad.kmagick

@Suppress("unused")
enum class ClipPathUnits(val id: Int) {
    UndefinedPathUnits(0),
    UserSpace(1),
    UserSpaceOnUse(2),
    ObjectBoundingBox(3);

    @Suppress("unused")
    internal companion object {
        fun fromNative(id: Int): ClipPathUnits {
            return (ClipPathUnits::id::find)(id)!!
        }
    }
}
