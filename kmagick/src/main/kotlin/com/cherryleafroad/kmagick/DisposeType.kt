package com.cherryleafroad.kmagick

/**
 * Define the GIF disposal image setting for images that are being created or read in.
 *
 * &nbsp;
 *
 * The layer disposal method defines the way each the displayed image is to be modified after the current 'frame' of an
 * animation has finished being displayed (after its 'delay' period), but before the next frame on an animation is to be
 * overlaid onto the display.
 */
enum class DisposeType(internal val id: Int) {
    /**
     * No disposal specified (equivalent to 'none').
     */
    UnrecognizedDispose(0),

    /**
     * No disposal specified (equivalent to 'none').
     */
    UndefinedDispose(0),

    /**
     * Do not dispose, just overlay next frame image.
     */
    NoneDispose(1),

    /**
     * Clear the frame area with the background color.
     */
    BackgroundDispose(2),

    /**
     * Clear to the image prior to this frames overlay.
     */
    PreviousDispose(3)
}
