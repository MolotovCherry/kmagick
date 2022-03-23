package com.cherryleafroad.kmagick

enum class AlignType(internal val id: Int) {
    UndefinedAlign(0),
    LeftAlign(1),
    CenterAlign(2),
    RightAlign(3);

    internal companion object {
        fun fromNative(id: Int): AlignType {
            return (AlignType::id::find)(id)!!
        }
    }
}
