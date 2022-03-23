package com.cherryleafroad.kmagick

/**
 * CompositeOperator is used to select the image composition algorithm used to compose a composite image with an image.
 * By default, each of the composite image pixels are replaced by the corresponding image tile pixel. Specify
 * CompositeOperator to select a different algorithm.
 */
enum class CompositeOperator(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedCompositeOp(0),
    AlphaCompositeOp(1),

    /**
     * The result is the same shape as image, with composite image obscuring image there the image shapes overlap.
     * Note that this differs from OverCompositeOp because the portion of composite image outside of image's shape does
     * not appear in the result.
     */
    AtopCompositeOp(2),
    BlendCompositeOp(3),
    BlurCompositeOp(4),

    /**
     * The result image shaded by composite image.
     */
    BumpmapCompositeOp(5),
    ChangeMaskCompositeOp(6),
    ClearCompositeOp(7),
    ColorBurnCompositeOp(8),
    ColorDodgeCompositeOp(9),
    ColorizeCompositeOp(10),
    CopyBlackCompositeOp(11),

    /**
     * The resulting image is the blue layer in image replaced with the blue layer in composite image. The other layers are copied untouched.
     */
    CopyBlueCompositeOp(12),

    /**
     * The resulting image is image replaced with composite image. Here the matte information is ignored.
     */
    CopyCompositeOp(13),
    CopyCyanCompositeOp(14),

    /**
     * The resulting image is the green layer in image replaced with the green layer in composite image. The other layers are copied untouched.
     */
    CopyGreenCompositeOp(15),
    CopyMagentaCompositeOp(16),

    /**
     * The resulting image is the matte layer in image replaced with the matte layer in composite image. The other layers
     * are copied untouched.
     *
     * &nbsp;
     *
     * The image compositor requires a matte, or alpha channel in the image for some operations. This extra channel usually
     * defines a mask which represents a sort of a cookie-cutter for the image. This is the case when matte is 255 (full coverage)
     * for pixels inside the shape, zero outside, and between zero and 255 on the boundary. For certain operations, if image does
     * not have a matte channel, it is initialized with 0 for any pixel matching in color to pixel location (0,0), otherwise 255
     * (to work properly borderWidth must be 0).
     */
    CopyAlphaCompositeOp(17),

    /**
     * The resulting image is the red layer in image replaced with the red layer in composite image. The other layers are copied untouched.
     */
    CopyRedCompositeOp(18),
    CopyYellowCompositeOp(19),
    DarkenCompositeOp(20),
    DarkenIntensityCompositeOp(21),

    /**
     * The result of abs(composite image - image). This is useful for comparing two very similar images.
     */
    DifferenceCompositeOp(22),
    DisplaceCompositeOp(23),
    DissolveCompositeOp(24),
    DistortCompositeOp(25),
    DivideDstCompositeOp(26),
    DivideSrcCompositeOp(27),
    DstAtopCompositeOp(28),
    DstCompositeOp(29),
    DstInCompositeOp(30),
    DstOutCompositeOp(31),
    DstOverCompositeOp(32),
    ExclusionCompositeOp(33),
    HardLightCompositeOp(34),
    HardMixCompositeOp(35),
    HueCompositeOp(36),

    /**
     * The result is a simply composite image cut by the shape of image. None of the image data of image is included in the result.
     */
    InCompositeOp(37),
    IntensityCompositeOp(38),
    LightenCompositeOp(39),
    LightenIntensityCompositeOp(40),
    LinearBurnCompositeOp(41),
    LinearDodgeCompositeOp(42),
    LinearLightCompositeOp(43),
    LuminizeCompositeOp(44),
    MathematicsCompositeOp(45),
    MinusDstCompositeOp(46),
    MinusSrcCompositeOp(47),
    ModulateCompositeOp(48),
    ModulusAddCompositeOp(49),
    ModulusSubtractCompositeOp(50),
    MultiplyCompositeOp(51),
    NoCompositeOp(52),

    /**
     * The resulting image is composite image with the shape of image cut out.
     */
    OutCompositeOp(53),

    /**
     * The result is the union of the two image shapes with the composite image obscuring image in the region of overlap.
     */
    OverCompositeOp(54),
    OverlayCompositeOp(55),
    PegtopLightCompositeOp(56),
    PinLightCompositeOp(57),

    /**
     * The result is just the sum of the image data. Output values are cropped to 255 (no overflow). This operation is independent of the matte channels.
     */
    PlusCompositeOp(58),
    ReplaceCompositeOp(59),
    SaturateCompositeOp(60),
    ScreenCompositeOp(61),
    SoftLightCompositeOp(62),
    SrcAtopCompositeOp(63),
    SrcCompositeOp(64),
    SrcInCompositeOp(65),
    SrcOutCompositeOp(66),
    SrcOverCompositeOp(67),
    ThresholdCompositeOp(68),
    VividLightCompositeOp(69),

    /**
     * The result is the image data from both composite image and image that is outside the overlap region. The overlap region will be blank.
     */
    XorCompositeOp(70),
    StereoCompositeOp(71),
    FreezeCompositeOp(72),
    InterpolateCompositeOp(73),
    NegateCompositeOp(74),
    ReflectCompositeOp(75),
    SoftBurnCompositeOp(76),
    SoftDodgeCompositeOp(77),
    StampCompositeOp(78),
    RMSECompositeOp(79)
}
