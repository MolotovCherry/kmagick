package com.cherryleafroad.kmagick

@Suppress("unused")
enum class RenderingIntent(val id: Int) {
    UndefinedIntent(0),
    SaturationIntent(1),
    PerceptualIntent(2),
    AbsoluteIntent(3),
    RelativeIntent(4)
}
