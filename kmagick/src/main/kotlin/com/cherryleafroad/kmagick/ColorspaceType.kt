package com.cherryleafroad.kmagick

/**
 * The `ColorspaceType` enumeration is used to specify the colorspace that quantization (color reduction and mapping) is
 * done under or to specify the colorspace when encoding an output image. Colorspaces are ways of describing colors to
 * fit the requirements of a particular application (e.g. Television, offset printing, color monitors). Color reduction,
 * by default, takes place in the [RGBColorspace]. Empirical evidence suggests that distances in color spaces such as
 * [YUVColorspace] or [YIQColorspace] correspond to perceptual color differences more closely han do distances in RGB space.
 * These color spaces may give better results when color reducing an image. Refer to quantize for more details.
 *
 * &nbsp;
 *
 * When encoding an output image, the colorspaces [RGBColorspace], [CMYKColorspace], and [GRAYColorspace] may be specified.
 * The [CMYKColorspace] option is only applicable when writing TIFF, JPEG, and Adobe Photoshop bitmap (PSD) files.
 */
@Suppress("EnumEntryName")
enum class ColorspaceType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedColorspace(0),
    CMYColorspace(1),

    /**
     * Cyan-Magenta-Yellow-Black colorspace. CYMK is a subtractive color system used by printers and photographers for the
     * rendering of colors with ink or emulsion, normally on a white surface.
     */
    CMYKColorspace(2),

    /**
     * Grayscale colorspace
     */
    GRAYColorspace(3),
    HCLColorspace(4),
    HCLpColorspace(5),
    HSBColorspace(6),
    HSIColorspace(7),
    HSLColorspace(8),
    HSVColorspace(9),
    HWBColorspace(10),
    LabColorspace(11),
    LCHColorspace(12),
    LCHabColorspace(13),
    LCHuvColorspace(14),
    LogColorspace(15),
    LMSColorspace(16),
    LuvColorspace(17),
    OHTAColorspace(18),
    Rec601YCbCrColorspace(19),
    Rec709YCbCrColorspace(20),

    /**
     * Red-Green-Blue colorspace.
     */
    RGBColorspace(21),
    scRGBColorspace(22),
    sRGBColorspace(23),

    /**
     * The Transparent color space behaves uniquely in that it preserves the matte channel of the image if it exists.
     */
    TransparentColorspace(24),
    xyYColorspace(25),
    XYZColorspace(26),
    YCbCrColorspace(27),
    YCCColorspace(28),
    YDbDrColorspace(29),
    YIQColorspace(30),
    YPbPrColorspace(31),

    /**
     * Y-signal, U-signal, and V-signal colorspace. YUV is most widely used to encode color for use in television transmission.
     */
    YUVColorspace(32),
    LinearGRAYColorspace(33),
    JzazbzColorspace(34),
    DisplayP3Colorspace(35),
    Adobe98Colorspace(36),
    ProPhotoColorspace(37)
}
