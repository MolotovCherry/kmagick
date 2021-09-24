package com.cherryleafroad.kmagick

typealias Quantum = Double

class PixelWandException(message: String) : MagickException(message)

/**
 * PixelWand API. Used for specifying certain colors.
 */
@Suppress("unused", "PrivatePropertyName", "MemberVisibilityCanBePrivate")
class PixelWand {
    init {
        new()
    }

    /**
     * Holds the pointer to internal object in memory.
     */
    private var handle: ULong? = null

    /**
     * Call the internal function to create the new wand.
     */
    private external fun new()

    /**
     * Check to see if this is still the correct wand.
     */
    @Throws(PixelWandException::class)
    external fun isWand(): Boolean

    /**
     * Clone the wand into a new one.
     */
    @Throws(PixelWandException::class)
    external fun clone(): PixelWand

    /**
     * While this automatically gets called by the `finalize()` destructor,
     * `finalize()` is not guaranteed to be called at all, nor called on time.
     * It's recommended to manually destroy all wands when finished.
     */
    external fun destroy()

    /**
     * While this is here to automatically call the destructor, due to
     * the way Kotlin/Java works, it's not guaranteed to be called at all,
     * or called on time. It is not recommended relying on this to destroy
     * the wand consistently/timely.
     */
    protected fun finalize() {
        destroy()
    }

    /**
     * Check if the distance between two colors is less than the specified distance.
     *
     * @param other The other PixelWand to compare to
     * @param fuzz Any two colors that are less than or equal to this distance squared are considered similar.
     */
    @Throws(PixelWandException::class)
    external fun isPixelWandSimilar(other: PixelWand, fuzz: Double): Boolean

    /**
     * The normalized HSL color of the pixel wand.
     */
    var hsl: HSL
        get() = pixelGetHSL()
        set(value) = pixelSetHSL(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetHSL(): HSL
    @Throws(PixelWandException::class)
    private external fun pixelSetHSL(hsl: HSL)
    /**
     * Set the normalized HSL color of the pixel wand.
     */
    @Throws(PixelWandException::class)
    external fun setHSL(hue: Double, light: Double, saturation: Double)

    /**
     * The color of the pixel wand as a string.
     * (e.g. "blue", "#0000ff", "rgb(0,0,255)", "cmyk(100,100,100,10)", etc.).
     */
    var color: String
        get() = pixelGetColorAsString()
        set(value) = pixelSetColor(value)
    @Throws(PixelWandException::class)
    private external fun pixelSetColor(color: String)
    @Throws(PixelWandException::class)
    private external fun pixelGetColorAsString(): String

    /**
     * The normalized color of the pixel wand as a string.
     */
    val normalizedColor: String
        get() = pixelGetColorAsNormalizedString()
    @Throws(PixelWandException::class)
    private external fun pixelGetColorAsNormalizedString(): String

    /**
     * The color count associated with this color.
     */
    var colorCount: ULong
        get() = pixelGetColorCount()
        set(value) = pixelSetColorCount(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetColorCount(): ULong
    @Throws(PixelWandException::class)
    private external fun pixelSetColorCount(count: ULong)

    /**
     *The colormap index of the pixel wand.
     */
    var index: Quantum
        get() = pixelGetIndex()
        set(value) = pixelSetIndex(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetIndex(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetIndex(index: Quantum)

    /**
     * The fuzz value of the pixel wand.
     */
    var fuzz: Double
        get() = pixelGetFuzz()
        set(value) = pixelSetFuzz(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetFuzz(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetFuzz(fuzz: Double)

    /**
     * The normalized alpha value of the pixel wand.
     * The level of transparency: 1.0 is fully opaque and 0.0 is fully transparent.
     */
    var alpha: Double
        get() = pixelGetAlpha()
        set(value) = pixelSetAlpha(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetAlpha(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetAlpha(alpha: Double)

    /**
     * The alpha value of the pixel wand.
     */
    var alphaQuantum: Quantum
        get() = pixelGetAlphaQuantum()
        set(value) = pixelSetAlphaQuantum(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetAlphaQuantum(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetAlphaQuantum(alpha: Quantum)

    /**
     * The normalized black color of the pixel wand.
     */
    var black: Double
        get() = pixelGetBlack()
        set(value) = pixelSetBlack(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetBlack(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetBlack(black: Double)

    /**
     * The black color of the pixel wand.
     */
    var blackQuantum: Quantum
        get() = pixelGetBlackQuantum()
        set(value) = pixelSetBlackQuantum(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetBlackQuantum(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetBlackQuantum(black: Quantum)

    /**
     * The normalized blue color of the pixel wand.
     */
    var blue: Double
        get() = pixelGetBlue()
        set(value) = pixelSetBlue(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetBlue(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetBlue(blue: Double)

    /**
     * The blue color of the pixel wand.
     */
    var blueQuantum: Quantum
        get() = pixelGetBlueQuantum()
        set(value) = pixelSetBlueQuantum(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetBlueQuantum(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetBlueQuantum(blue: Quantum)

    /**
     * The normalized cyan color of the pixel wand.
     */
    var cyan: Double
        get() = pixelGetCyan()
        set(value) = pixelSetCyan(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetCyan(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetCyan(cyan: Double)

    /**
     * The cyan color of the pixel wand.
     */
    var cyanQuantum: Quantum
        get() = pixelGetCyanQuantum()
        set(value) = pixelSetCyanQuantum(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetCyanQuantum(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetCyanQuantum(cyan: Quantum)

    /**
     * The normalized green color of the pixel wand.
     */
    var green: Double
        get() = pixelGetGreen()
        set(value) = pixelSetGreen(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetGreen(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetGreen(green: Double)

    /**
     * The green color of the pixel wand.
     */
    var greenQuantum: Quantum
        get() = pixelGetGreenQuantum()
        set(value) = pixelSetGreenQuantum(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetGreenQuantum(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetGreenQuantum(green: Quantum)

    /**
     * The normalized magenta color of the pixel wand.
     */
    var magenta: Double
        get() = pixelGetMagenta()
        set(value) = pixelSetMagenta(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetMagenta(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetMagenta(magenta: Double)

    /**
     * The magenta color of the pixel wand.
     */
    var magentaQuantum: Quantum
        get() = pixelGetMagentaQuantum()
        set(value) = pixelSetMagentaQuantum(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetMagentaQuantum(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetMagentaQuantum(magenta: Quantum)

    /**
     * The normalized red color of the pixel wand.
     */
    var red: Double
        get() = pixelGetRed()
        set(value) = pixelSetRed(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetRed(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetRed(red: Double)

    /**
     * The red color of the pixel wand.
     */
    var redQuantum: Quantum
        get() = pixelGetRedQuantum()
        set(value) = pixelSetRedQuantum(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetRedQuantum(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetRedQuantum(red: Quantum)

    /**
     * The normalized yellow color of the pixel wand.
     */
    var yellow: Double
        get() = pixelGetYellow()
        set(value) = pixelSetYellow(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetYellow(): Double
    @Throws(PixelWandException::class)
    private external fun pixelSetYellow(yellow: Double)

    /**
     * The yellow color of the pixel wand.
     */
    var yellowQuantum: Quantum
        get() = pixelGetYellowQuantum()
        set(value) = pixelSetYellowQuantum(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetYellowQuantum(): Quantum
    @Throws(PixelWandException::class)
    private external fun pixelSetYellowQuantum(yellow: Quantum)
}
