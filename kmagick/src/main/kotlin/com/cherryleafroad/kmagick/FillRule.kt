package com.cherryleafroad.kmagick

@Suppress("unused")
enum class FillRule(val id: Int) {
    UndefinedRule(0),
    EvenOddRule(1),
    NonZeroRule(2);

    @Suppress("unused")
    internal companion object {
        fun fromNative(id: Int): FillRule {
            return (FillRule::id::find)(id)!!
        }
    }
}
