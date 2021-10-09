package com.cherryleafroad.kmagick

@Suppress("unused")
enum class AlignType(val id: Int) {
    UndefinedAlign(0),
    LeftAlign(1),
    CenterAlign(2),
    RightAlign(3);

    @Suppress("unused")
    companion object {
        @JvmName("fromNative")
        internal fun fromNative(id: Int): AlignType {
            return (AlignType::id::find)(id)!!
        }
    }
}
