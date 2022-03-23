package com.cherryleafroad.kmagick

/**
 * Holds the result of a comparison between 2 images using `MagickWand.compareImages()`
 */
data class Comparison(
    /**
     * The computed distortion between the images.
     */
    val distortion: Double,

    /**
     * The difference between the two images. Null if there was no difference.
     */
    val diffImage: MagickWand?
)
