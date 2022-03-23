package com.cherryleafroad.kmagick

/**
 * Gives control of the alpha/matte channel of an image.
 *
 * &nbsp;
 *
 * Used to set a flag on an image indicating whether or not to use existing alpha channel data, to create an alpha channel,
 * or to perform other operations on the alpha channel.
 */
enum class AlphaChannelOption(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedAlphaChannel(0),

    /**
     * Enable the image's transparency channel. Note normally Set should be used instead of this, unless you specifically
     * need to preserve existing (but specifically turned Off) transparency channel.
     */
    ActivateAlphaChannel(1),

    /**
     * Associate the alpha channel with the image.
     */
    AssociateAlphaChannel(2),

    /**
     * Set any fully-transparent pixel to the background color, while leaving it fully-transparent. This can make some
     * image file formats, such as PNG, smaller as the RGB values of transparent pixels are more uniform, and thus can
     * compress better.
     */
    BackgroundAlphaChannel(3),

    /**
     * Turns [On][OnAlphaChannel] the alpha/matte channel, then copies the grayscale intensity of the image, into the alpha channel,
     * converting a grayscale mask into a transparent shaped mask ready to be colored appropriately. The color channels
     * are not modified.
     */
    CopyAlphaChannel(4),

    /**
     * Disables the image's transparency channel. Does not delete or change the existing data, just turns off the use of
     * that data.
     */
    DeactivateAlphaChannel(5),
    DiscreteAlphaChannel(6),

    /**
     * Disassociate the alpha channel from the image.
     */
    DisassociateAlphaChannel(7),

    /**
     * Copies the alpha channel values into all the color channels and turns [Off][OffAlphaChannel] the image's transparency,
     * so as to generate a grayscale mask of the image's shape. The alpha channel data is left intact just deactivated.
     * This is the inverse of [Copy][CopyAlphaChannel].
     */
    ExtractAlphaChannel(8),

    /**
     * Turns off the alpha channel.
     */
    OffAlphaChannel(9),

    /**
     * Turns on the alpha channel.
     */
    OnAlphaChannel(10),

    /**
     * Enables the alpha/matte channel and forces it to be fully opaque.
     */
    OpaqueAlphaChannel(11),

    /**
     * Composite the image over the background color.
     */
    RemoveAlphaChannel(12),

    /**
     * Activates the alpha/matte channel. If it was previously turned off then it also resets the channel to opaque.
     * If the image already had the alpha channel turned on, it will have no effect.
     */
    SetAlphaChannel(13),

    /**
     * As per [Copy][CopyAlphaChannel] but also colors the resulting shape mask with the current background color. That
     * is the RGB color channels is replaced, with appropriate alpha shape.
     */
    ShapeAlphaChannel(14),

    /**
     * Activates the alpha/matte channel and forces it to be fully transparent. This effectively creates a fully
     * transparent image the same size as the original and with all its original RGB data still intact, but fully transparent.
     */
    TransparentAlphaChannel(15);

    internal companion object {
        fun fromNative(id: Int): AlphaChannelOption {
            return (AlphaChannelOption::id::find)(id)!!
        }
    }
}
