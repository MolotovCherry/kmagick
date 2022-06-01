package com.cherryleafroad.kmagick

/**
 * ChannelType is used as an argument when doing color separations. Use ChannelType when extracting a layer from an
 * image. MatteChannel is useful for extracting the opacity values from an image. Note that an image may be represented
 * in RGB, RGBA, CMYK, or CMYKA, pixel formats and a channel may only be extracted if it is valid for the current pixel
 * format.
 */
enum class ChannelType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedChannel(0),

    /**
     * Extract red channel (RGB images only)
     */
    RedChannel(1),
    GrayChannel(1),

    /**
     * Extract cyan channel (CMYK images only)
     */
    CyanChannel(1),
    LChannel(1),

    /**
     * Extract green channel (RGB images only)
     */
    GreenChannel(2),

    /**
     * Extract magenta channel (CMYK images only)
     */
    MagentaChannel(2),
    aChannel(2),

    /**
     * Extract blue channel (RGB images only)
     */
    BlueChannel(4),
    bChannel(2),

    /**
     * Extract yellow channel (CMYK images only)
     */
    YellowChannel(4),

    /**
     * Extract black channel (CMYK images only)
     */
    BlackChannel(8),
    AlphaChannel(16),

    /**
     * Extract matte (opacity values) channel (CMYKA images only)
     */
    OpacityChannel(16),
    IndexChannel(32),
    ReadMaskChannel(64),
    WriteMaskChannel(128),
    MetaChannel(256),
    CompositeMaskChannel(512),
    CompositeChannels(31),
    AllChannels(134217727),
    TrueAlphaChannel(256),
    RGBChannels(512),
    GrayChannels(1024),
    SyncChannels(131072),
    DefaultChannels(134217727);

    internal companion object {
        fun fromNative(id: Int): ChannelType {
            return (ChannelType::id::find)(id)!!
        }
    }
}
