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
     * The result is the same shape as image image, with composite image obscuring image there the image shapes overlap.
     * Note that this differs from OverCompositeOp because the portion of composite image outside of image's shape does
     * not appear in the result.
     */
    AtopCompositeOp(2),

    /**
     * Average the images together ([plus][PlusCompositeOp]) according to the percentages given and each pixels transparency. If only a
     * single percentage value is given it sets the weight of the composite or 'source' image, while the background image
     * is weighted by the exact opposite amount.
     */
    BlendCompositeOp(3),

    /**
     * A Variable Blur Mapping Composition method, where each pixel in the overlaid region is replaced with an Elliptical
     * Weighted Average (EWA), with an ellipse (typically a circle) of the given sigma size, scaled according to overlay
     * (source image) grayscale mapping.
     *
     * &nbsp;
     *
     * As per [Displace][DisplaceCompositeOp] and [Distort][DistortCompositeOp], the red channel will modulate the width
     * of the ellipse, while the green channel will modulate the height of the ellipse. If a single Angle value is given
     * in the arguments, then the ellipse will then be rotated by the angle specified.
     *
     * &nbsp;
     *
     * Normally the blue channel of the mapping overlay image is ignored. However if a second ellipse angle is given,
     * then it is assumed that the blue channel defines a variable angle for the ellipse ranging from the first angle to
     * the second angle given. This allows to generate radial blurs, or a rough approximation for rotational blur. Or
     * any mix of the two.
     */
    BlurCompositeOp(4),

    /**
     * The result image shaded by composite image.
     */
    BumpmapCompositeOp(5),

    /**
     * Replace any destination pixel that is the similar to the source images pixel (as defined by the current -fuzz factor),
     * with transparency.
     */
    ChangeMaskCompositeOp(6),

    /**
     * Both the color and the alpha of the destination are cleared. Neither the source nor the destination are used
     * (except for destinations size and other meta-data which is always preserved).
     */
    ClearCompositeOp(7),

    /**
     * Darkens the destination color to reflect the source color. Painting with white produces no change.
     */
    ColorBurnCompositeOp(8),

    /**
     * Brightens the destination color to reflect the source color. Painting with black produces no change.
     */
    ColorDodgeCompositeOp(9),
    ColorizeCompositeOp(10),

    /**
     * Copy the Black channel in the source image to the same channel in the destination image. If the channel
     * specified does not exist in the source image, then it is assumed that the source image is a special
     * grayscale channel image of the values that is to be copied.
     */
    CopyBlackCompositeOp(11),

    /**
     * Copy the Blue channel in the source image to the same channel in the destination image.
     */
    CopyBlueCompositeOp(12),

    /**
     * The resulting image is image replaced with composite image. Here the matte information is ignored.
     * This is equivalent to the Duff-Porter composition method [Src][SrcCompositeOp] but without clearing the parts of the
     * destination image that is not overlaid.
     */
    CopyCompositeOp(13),

    /**
     * Copy the Cyan channel in the source image to the same channel in the destination image.
     */
    CopyCyanCompositeOp(14),

    /**
     * Copy the Green channel in the source image to the same channel in the destination image.
     */
    CopyGreenCompositeOp(15),

    /**
     * Copy the Magenta channel in the source image to the same channel in the destination image.
     */
    CopyMagentaCompositeOp(16),

    /**
     * The resulting image is the matte layer in image replaced with the matte layer in composite image. The other layers
     * are copied untouched. If the channel specified does not exist in the source image,then it is assumed that the
     * source image is a special grayscale channel image of the values that is to be copied.
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
     * Copy the Red channel in the source image to the same channel in the destination image.
     */
    CopyRedCompositeOp(18),

    /**
     * Copy the Yellow channel in the source image to the same channel in the destination image.
     */
    CopyYellowCompositeOp(19),

    /**
     * 	Selects the darker of the destination and source colors. The destination is replaced with the source when the
     * 	source is darker, otherwise it is left unchanged.
     */
    DarkenCompositeOp(20),
    DarkenIntensityCompositeOp(21),

    /**
     * The result of abs(composite image - image). This is useful for comparing two very similar images.
     * Subtracts the darker of the two constituent colors from the lighter. Painting with white inverts the destination
     * color. Painting with black produces no change.
     */
    DifferenceCompositeOp(22),

    /**
     * Not available in "composite" at this time.
     *
     * &nbsp;
     *
     * With this option, the 'overlay' image, and optionally the 'mask' image, is used as a relative displacement map,
     * which is used to displace the lookup of what part of the destination image is seen at each point of the overlaid
     * area. Much like the displacement map is a 'lens' that distorts the original 'background' image behind it.
     *
     * &nbsp;
     *
     * The X-scale is modulated by the 'red' channel of the overlay image while the Y-scale is modulated by the green channel,
     * (the mask image if given is rolled into green channel of the overlay image. This separation allows you to modulate
     * the X and Y lookup displacement separately allowing you to do 2-dimensional displacements, rather than
     * 1-dimensional vectored displacements (using grayscale image).
     *
     * &nbsp;
     *
     * If the overlay image contains transparency this is used as a mask of the resulting image to remove 'invalid' pixels.
     *
     * &nbsp;
     *
     * The '%' flag makes the displacement scale relative to the size of the overlay image (100% = half width/height of
     * image). Using '!' switches percentage arguments to refer to the destination image size instead.
     */
    DisplaceCompositeOp(23),

    /**
     * Dissolve the 'source' image by the percentage given before overlaying 'over' the 'destination' image. If
     * src_percent is greater than 100, it starts dissolving the main image so it will become transparent at a value of
     * 200. If both percentages are given, each image are dissolved to the percentages given.
     */
    DissolveCompositeOp(24),

    /**
     * Not available in "composite" at this time.
     *
     * &nbsp;
     *
     * Exactly as per 'Displace' (above), but using absolute coordinates, relative to the center of the overlay (or that given).
     * Basically allows you to generate absolute distortion maps where 'black' will look up the left/top edge, and 'white'
     * looks up the bottom/right edge of the destination image, according to the scale given.
     *
     * &nbsp;
     *
     * The '!' flag not only switches percentage scaling, to use the destination image, but also the image the center offset
     * of thelookup. This means the overlay can lookup a completely different region of the destination image.
     */
    DistortCompositeOp(25),
    DivideDstCompositeOp(26),
    DivideSrcCompositeOp(27),

    /**
     * The part of the destination lying inside of the source is composited over the source and replaces the destination.
     * Areas not overlaid are cleared.
     */
    DstAtopCompositeOp(28),

    /**
     * The destination is left untouched. The source image is completely ignored.
     */
    DstCompositeOp(29),

    /**
     * The part of the destination lying inside of the source replaces the destination. Areas not overlaid are cleared.
     */
    DstInCompositeOp(30),

    /**
     * The part of the destination lying outside of the source replaces the destination.
     */
    DstOutCompositeOp(31),

    /**
     * The destination is composited over the source and the result replaces the destination.
     */
    DstOverCompositeOp(32),

    /**
     * 	Produces an effect similar to that of [Difference][DifferenceCompositeOp], but appears as lower contrast.
     * 	Painting with white inverts the destination color. Painting with black produces no change.
     */
    ExclusionCompositeOp(33),

    /**
     * Multiplies or screens the colors, dependent on the source color value. If the source color is lighter than 0.5,
     * the destination is lightened as if it were screened. If the source color is darker than 0.5, the destination is
     * darkened, as if it were multiplied. The degree of lightening or darkening is proportional to the difference between
     * the source color and 0.5. If it is equal to 0.5 the destination is unchanged. Painting with pure black or white
     * produces black or white.
     */
    HardLightCompositeOp(34),
    HardMixCompositeOp(35),
    HueCompositeOp(36),

    /**
     * The result is a simply composite image cut by the shape of image. None of the image data of image is included in
     * the result.
     */
    InCompositeOp(37),
    IntensityCompositeOp(38),

    /**
     * Selects the lighter of the destination and source colors. The destination is replaced with the source when the
     * source is lighter, otherwise it is left unchanged.
     */
    LightenCompositeOp(39),
    LightenIntensityCompositeOp(40),

    /**
     * As [LinearDodge][LinearDodgeCompositeOp], but also subtract one from the result. Sort of a additive
     * [Screen][ScreenCompositeOp] of the images.
     */
    LinearBurnCompositeOp(41),

    /**
     * This is equivalent to [Plus][PlusCompositeOp] in that the color channels are simply added, however it does not
     * [Plus][PlusCompositeOp] the alpha channel, but uses the normal [Over][OverCompositeOp] alpha blending, which
     * transparencies are involved. Produces a sort of additive multiply-like result.
     */
    LinearDodgeCompositeOp(42),

    /**
     * Like [HardLight][HardLightCompositeOp] but using [LinearDoge][LinearDodgeCompositeOp] and [LinearBurn][LinearBurnCompositeOp]
     * instead. Increases contrast slightly with an impact on the foreground's tonal values.
     */
    LinearLightCompositeOp(43),
    LuminizeCompositeOp(44),

    /**
     * Merge the source and destination images according to the formula A*Sc*Dc + B*Sc + C*Dc + D .
     * Can be used to generate a custom composition method that would otherwise need to be implemented using the slow
     * -fx DIY image operator.
     */
    MathematicsCompositeOp(45),

    /**
     * Subtract the colors in the destination image from the source image. When transparency is involved, opaque areas
     * is subtracted from any source opaque areas.
     */
    MinusDstCompositeOp(46),

    /**
     * Subtract the colors in the source image from the destination image. When transparency is involved, opaque areas
     * is subtracted from any destination opaque areas.
     */
    MinusSrcCompositeOp(47),

    /**
     * Take a grayscale image (with alpha mask) and modify the destination image's brightness according to watermark image's
     * grayscale value and the brightness percentage. The destinations color saturation attribute is just direct modified
     * by the saturation percentage, which defaults to 100 percent (no color change).
     */
    ModulateCompositeOp(48),
    ModulusAddCompositeOp(49),
    ModulusSubtractCompositeOp(50),

    /**
     * The source is multiplied by the destination and replaces the destination. The resultant color is always at least
     * as dark as either of the two constituent colors. Multiplying any color with black produces black. Multiplying any
     * color with white leaves the original color unchanged.
     */
    MultiplyCompositeOp(51),

    /**
     * Don't use any composite op.
     */
    NoCompositeOp(52),

    /**
     * The resulting image is composite image with the shape of image cut out.
     */
    OutCompositeOp(53),

    /**
     * The result is the union of the two image shapes with the composite image obscuring image in the region of overlap.
     */
    OverCompositeOp(54),

    /**
     * Multiplies or screens the colors, dependent on the destination color. Source colors overlay the destination whilst
     * preserving its highlights and shadows. The destination color is not replaced, but is mixed with the source color to
     * reflect the lightness or darkness of the destination.
     */
    OverlayCompositeOp(55),

    /**
     * Almost equivalent to [SoftLight][SoftLightCompositeOp], but using a continuous mathematical formula rather than
     * two conditionally selected formulae.
     */
    PegtopLightCompositeOp(56),

    /**
     * Similar to [HardLight][HardLightCompositeOp], but using sharp linear shadings, to simulate the effects of a strong
     * 'pinhole' light source.
     */
    PinLightCompositeOp(57),

    /**
     * The result is just the sum of the image data. Output values are cropped to 255 (no overflow). This operation is
     * independent of the matte channels. The source is added to the destination and replaces the destination. This operator
     * is useful for averaging or a controlled merger of two images, rather than a direct overlay.
     */
    PlusCompositeOp(58),
    ReplaceCompositeOp(59),
    SaturateCompositeOp(60),

    /**
     * The source and destination are complemented and then multiplied and then replace the destination. The resultant
     * color is always at least as light as either of the two constituent colors. Screening any color with white produces
     * white. Screening any color with black leaves the original color unchanged.
     */
    ScreenCompositeOp(61),

    /**
     * Darkens or lightens the colors, dependent on the source color value. If the source color is lighter than 0.5, the
     * destination is lightened. If the source color is darker than 0.5, the destination is darkened, as if it were burned
     * in. The degree of darkening or lightening is proportional to the difference between the source color and 0.5. If
     * it is equal to 0.5, the destination is unchanged. Painting with pure black or white produces a distinctly darker
     * or lighter area, but does not result in pure black or white.
     */
    SoftLightCompositeOp(62),

    /**
     * The part of the source lying inside of the destination is composited onto the destination.
     */
    SrcAtopCompositeOp(63),

    /**
     * The source is copied to the destination. The destination is not used as input, though it is cleared.
     */
    SrcCompositeOp(64),

    /**
     * The part of the source lying inside of the destination replaces the destination.
     */
    SrcInCompositeOp(65),

    /**
     * The part of the source lying outside of the destination replaces the destination.
     */
    SrcOutCompositeOp(66),

    /**
     * The source is composited over the destination. this is the default alpha blending compose method, when neither
     * the compose setting is set, nor is set in the image meta-data.
     */
    SrcOverCompositeOp(67),
    ThresholdCompositeOp(68),

    /**
     * A modified [LinearLight][LinearLightCompositeOp] designed to preserve very stong primary and secondary colors in the image.
     */
    VividLightCompositeOp(69),

    /**
     * The result is the image data from both composite image and image that is outside the overlap region. The overlap
     * region will be blank. The part of the source that lies outside of the destination is combined with the part of the
     * destination that lies outside of the source. Source or Destination, but not both.
     */
    XorCompositeOp(70),

    /**
     * Ccreate a stereo anaglyph.
     */
    StereoCompositeOp(71),

    /**
     * Another variation of reflect mode (base and blend color inverted, the result inverted again).
     */
    FreezeCompositeOp(72),

    /**
     * This mode somehow combines multiply and screen mode (looks very similar for very dark or bright colors).
     */
    InterpolateCompositeOp(73),

    /**
     * The "opposite" of difference mode. Note that it is not difference mode inverted, because black and white return
     * the same result, but colors between become brighter instead of darker.
     */
    NegateCompositeOp(74),

    /**
     * This mode is useful when adding shining objects or light zones to images. The formula is similar to color dodge,
     * but the result is not that bright in most cases. The result looks a bit like soft light.
     */
    ReflectCompositeOp(75),

    /**
     * A combination of color burn and inverse color dodge mode, but a lot smoother than both of them. The base image is
     * lightened a bit, but very dark blend colors are "burned" in.
     */
    SoftBurnCompositeOp(76),

    /**
     * Combination of color dodge and inverse color burn mode, but a lot smoother than both of them. The base image is
     * darkened a bit, but very bright blend colors are "dodged" in.
     */
    SoftDodgeCompositeOp(77),

    /**
     * This mode somehow is similar to average mode. It is helpful when applying relief or bump textures to images.
     */
    StampCompositeOp(78),
    RMSECompositeOp(79)
}
