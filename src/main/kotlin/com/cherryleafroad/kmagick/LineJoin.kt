package com.cherryleafroad.kmagick

@Suppress("unused")
enum class LineJoin(val id: Int) {
    UndefinedJoin(0),
    MiterJoin(1),
    RoundJoin(2),
    BevelJoin(3);

    @Suppress("unused")
    companion object {
        @JvmName("fromNative")
        internal fun fromNative(id: Int): LineJoin {
            return (LineJoin::id::find)(id)!!
        }
    }
}
