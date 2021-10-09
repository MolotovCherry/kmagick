package com.cherryleafroad.kmagick

@Suppress("unused")
enum class StretchType(val id: Int) {
    UndefinedStretch(0),
    NormalStretch(1),
    UltraCondensedStretch(2),
    ExtraCondensedStretch(3),
    CondensedStretch(4),
    SemiCondensedStretch(5),
    SemiExpandedStretch(6),
    ExpandedStretch(7),
    ExtraExpandedStretch(8),
    UltraExpandedStretch(9),
    AnyStretch(10);

    @Suppress("unused")
    companion object {
        @JvmName("fromNative")
        internal fun fromNative(id: Int): StretchType {
            return (StretchType::id::find)(id)!!
        }
    }
}
