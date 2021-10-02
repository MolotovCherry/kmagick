package com.cherryleafroad.kmagick

/**
 * Holds the result of a comparison between 2 images using `MagickWand.compareImages()`
 *
 * @property distortion The computed distortion between the images.
 * @property diffImage The difference between the two images. Null if there was no difference.
 */
data class Comparison(
    val distortion: Double,
    val diffImage: MagickWand?
)
