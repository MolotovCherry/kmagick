package com.cherryleafroad.kmagick

@Suppress("unused")
enum class LineJoin(val id: Int) {
    UndefinedJoin(0),
    MiterJoin(1),
    RoundJoin(2),
    BevelJoin(3)
}