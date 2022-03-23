package com.cherryleafroad.kmagick

enum class DitherMethod(internal val id: Int) {
    UndefinedDitherMethod(0),
    NoDitherMethod(1),
    RiemersmaDitherMethod(2),
    FloydSteinbergDitherMethod(3);

    internal companion object {
        fun fromNative(id: Int): ClipPathUnits {
            return (ClipPathUnits::id::find)(id)!!
        }
    }
}
