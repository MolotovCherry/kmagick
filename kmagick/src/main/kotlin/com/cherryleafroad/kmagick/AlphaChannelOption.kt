package com.cherryleafroad.kmagick

@Suppress("unused")
enum class AlphaChannelOption(val id: Int) {
    UndefinedAlphaChannel(0),
    ActivateAlphaChannel(1),
    AssociateAlphaChannel(2),
    BackgroundAlphaChannel(3),
    CopyAlphaChannel(4),
    DeactivateAlphaChannel(5),
    DiscreteAlphaChannel(6),
    DisassociateAlphaChannel(7),
    ExtractAlphaChannel(8),
    OffAlphaChannel(9),
    OnAlphaChannel(10),
    OpaqueAlphaChannel(11),
    RemoveAlphaChannel(12),
    SetAlphaChannel(13),
    ShapeAlphaChannel(14),
    TransparentAlphaChannel(15);

    internal companion object {
        fun fromNative(id: Int): AlphaChannelOption {
            return (AlphaChannelOption::id::find)(id)!!
        }
    }
}
