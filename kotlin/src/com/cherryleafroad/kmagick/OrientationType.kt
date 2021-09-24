package com.cherryleafroad.kmagick

@Suppress("unused")
enum class OrientationType(val id: Int) {
    UndefinedOrientation(0),
    TopLeftOrientation(1),
    TopRightOrientation(2),
    BottomRightOrientation(3),
    BottomLeftOrientation(4),
    LeftTopOrientation(5),
    RightTopOrientation(6),
    RightBottomOrientation(7),
    LeftBottomOrientation(8)
}
