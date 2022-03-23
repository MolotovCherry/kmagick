package com.cherryleafroad.kmagick

/**
 * Rendering intent is a concept defined by ICC Spec ICC.1:1998-09, "File Format for Color Profiles". ImageMagick
 * uses [RenderingIntent] in order to support ICC Color Profiles.
 *
 * From the specification: "Rendering intent specifies the style of reproduction to be used during the evaluation of
 * this profile in a sequence of profiles. It applies specifically to that profile in the sequence and not to the entire
 * sequence. Typically, the user or application will set the rendering intent dynamically at runtime or embedding time."
 */
enum class RenderingIntent(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedIntent(0),

    /**
     * A rendering intent that specifies the saturation of the pixels in the image is preserved perhaps at the expense
     * of accuracy in hue and lightness.
     */
    SaturationIntent(1),

    /**
     * A rendering intent that specifies the full gamut of the image is compressed or expanded to fill the gamut of the
     * destination device. Gray balance is preserved but colorimetric accuracy might not be preserved.
     */
    PerceptualIntent(2),

    /**
     * Absolute colorimetric
     */
    AbsoluteIntent(3),

    /**
     * Relative colorimetric
     */
    RelativeIntent(4)
}
