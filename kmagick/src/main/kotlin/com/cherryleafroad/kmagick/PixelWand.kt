package com.cherryleafroad.kmagick

import java.io.Closeable

typealias Quantum = Float

class PixelWandException(message: String) : MagickException(message)

/**
 * PixelWand API. Used for specifying certain colors.
 */
@Suppress("unused", "MemberVisibilityCanBePrivate")
class PixelWand : Closeable {
    constructor() {
        new()
    }

    /**
     * Internal use ONLY. Copies another wand
     */
    internal constructor(wand: PixelWand) {
        clone(wand)
    }

    internal companion object {
        /**
         * Internal use ONLY. Creates instance without calling constructor
         */
        fun newInstance(): PixelWand {
            return pixelWandInstantiator.newInstance()
        }
    }

    /**
     * Holds the pointer to internal object in memory.
     */
    private var handle: Long? = null

    /**
     * Check to see if this is initialized with the underlying C obj.
     * If it's not, then calling any functions will result in a null exception.
     *
     * This object is _ALWAYS_ initialized, unless [destroy], [finalize], or [Magick.terminate] got called.
     */
    val isInitialized: Boolean
        get() = handle != null

    /**
     * Call the internal function to create the new wand.
     */
    @Throws(PixelWandException::class)
    private external fun new()

    /**
     * Check to see if this is still the correct wand.
     */
    external fun isWand(): Boolean

    /**
     * Clone the wand into a new one.
     */
    @Throws(PixelWandException::class)
    fun clone(): PixelWand {
        handle ?: throw PixelWandException("Wand is null")
        return PixelWand(this)
    }
    @Throws(PixelWandException::class)
    private external fun clone(wand: PixelWand)

    /**
     * Clear any internal exceptions
     */
    external fun clearException()

    /**
     * Get the type of internal exception
     */
    fun getExceptionType(): ExceptionType {
        val exceptionType = nativeGetExceptionType()
        return (ExceptionType::id::find)(exceptionType)!!
    }
    @Throws(PixelWandException::class)
    private external fun nativeGetExceptionType(): Int

    /**
     * Get the internal exception type and message
     */
    @Throws(PixelWandException::class)
    external fun getException(): NativeMagickException

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
     * This isn't meant to be called manually. You can call [destroy] instead. This does the
     * same thing as [destroy], but it's here to be used with a `use{}` block for
     * convenience. For example `wand.use { }`
     */
    override fun close() {
        destroy()
    }

    /**
     * Check if the distance between two colors is less than the specified distance.
     *
     * @param other The other PixelWand to compare to
     * @param fuzz Any two colors that are less than or equal to this distance squared are considered similar.
     */
    @Throws(PixelWandException::class)
    external fun isSimilar(other: PixelWand, fuzz: Double): Boolean

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
    var colorCount: Long
        get() = pixelGetColorCount()
        set(value) = pixelSetColorCount(value)
    @Throws(PixelWandException::class)
    private external fun pixelGetColorCount(): Long
    @Throws(PixelWandException::class)
    private external fun pixelSetColorCount(count: Long)

    /**
     *The colormap index of the pixel wand.
     */
    var index: Quantum
        get() = pixelGetIndex()
        set(value) = pixelSetIndex(value)
    private external fun pixelGetIndex(): Quantum
    private external fun pixelSetIndex(index: Quantum)

    /**
     * The fuzz value of the pixel wand.
     */
    var fuzz: Double
        get() = pixelGetFuzz()
        set(value) = pixelSetFuzz(value)
    private external fun pixelGetFuzz(): Double
    private external fun pixelSetFuzz(fuzz: Double)

    /**
     * The normalized alpha value of the pixel wand.
     * The level of transparency: 1.0 is fully opaque and 0.0 is fully transparent.
     */
    var alpha: Double
        get() = pixelGetAlpha()
        set(value) = pixelSetAlpha(value)
    private external fun pixelGetAlpha(): Double
    private external fun pixelSetAlpha(alpha: Double)

    /**
     * The alpha value of the pixel wand.
     */
    var alphaQuantum: Quantum
        get() = pixelGetAlphaQuantum()
        set(value) = pixelSetAlphaQuantum(value)
    private external fun pixelGetAlphaQuantum(): Quantum
    private external fun pixelSetAlphaQuantum(alpha: Quantum)

    /**
     * The normalized black color of the pixel wand.
     */
    var black: Double
        get() = pixelGetBlack()
        set(value) = pixelSetBlack(value)
    private external fun pixelGetBlack(): Double
    private external fun pixelSetBlack(black: Double)

    /**
     * The black color of the pixel wand.
     */
    var blackQuantum: Quantum
        get() = pixelGetBlackQuantum()
        set(value) = pixelSetBlackQuantum(value)
    private external fun pixelGetBlackQuantum(): Quantum
    private external fun pixelSetBlackQuantum(black: Quantum)

    /**
     * The normalized blue color of the pixel wand.
     */
    var blue: Double
        get() = pixelGetBlue()
        set(value) = pixelSetBlue(value)
    private external fun pixelGetBlue(): Double
    private external fun pixelSetBlue(blue: Double)

    /**
     * The blue color of the pixel wand.
     */
    var blueQuantum: Quantum
        get() = pixelGetBlueQuantum()
        set(value) = pixelSetBlueQuantum(value)
    private external fun pixelGetBlueQuantum(): Quantum
    private external fun pixelSetBlueQuantum(blue: Quantum)

    /**
     * The normalized cyan color of the pixel wand.
     */
    var cyan: Double
        get() = pixelGetCyan()
        set(value) = pixelSetCyan(value)
    private external fun pixelGetCyan(): Double
    private external fun pixelSetCyan(cyan: Double)

    /**
     * The cyan color of the pixel wand.
     */
    var cyanQuantum: Quantum
        get() = pixelGetCyanQuantum()
        set(value) = pixelSetCyanQuantum(value)
    private external fun pixelGetCyanQuantum(): Quantum
    private external fun pixelSetCyanQuantum(cyan: Quantum)

    /**
     * The normalized green color of the pixel wand.
     */
    var green: Double
        get() = pixelGetGreen()
        set(value) = pixelSetGreen(value)
    private external fun pixelGetGreen(): Double
    private external fun pixelSetGreen(green: Double)

    /**
     * The green color of the pixel wand.
     */
    var greenQuantum: Quantum
        get() = pixelGetGreenQuantum()
        set(value) = pixelSetGreenQuantum(value)
    private external fun pixelGetGreenQuantum(): Quantum
    private external fun pixelSetGreenQuantum(green: Quantum)

    /**
     * The normalized magenta color of the pixel wand.
     */
    var magenta: Double
        get() = pixelGetMagenta()
        set(value) = pixelSetMagenta(value)
    private external fun pixelGetMagenta(): Double
    private external fun pixelSetMagenta(magenta: Double)

    /**
     * The magenta color of the pixel wand.
     */
    var magentaQuantum: Quantum
        get() = pixelGetMagentaQuantum()
        set(value) = pixelSetMagentaQuantum(value)
    private external fun pixelGetMagentaQuantum(): Quantum
    private external fun pixelSetMagentaQuantum(magenta: Quantum)

    /**
     * The normalized red color of the pixel wand.
     */
    var red: Double
        get() = pixelGetRed()
        set(value) = pixelSetRed(value)
    private external fun pixelGetRed(): Double
    private external fun pixelSetRed(red: Double)

    /**
     * The red color of the pixel wand.
     */
    var redQuantum: Quantum
        get() = pixelGetRedQuantum()
        set(value) = pixelSetRedQuantum(value)
    private external fun pixelGetRedQuantum(): Quantum
    private external fun pixelSetRedQuantum(red: Quantum)

    /**
     * The normalized yellow color of the pixel wand.
     */
    var yellow: Double
        get() = pixelGetYellow()
        set(value) = pixelSetYellow(value)
    private external fun pixelGetYellow(): Double
    private external fun pixelSetYellow(yellow: Double)

    /**
     * The yellow color of the pixel wand.
     */
    var yellowQuantum: Quantum
        get() = pixelGetYellowQuantum()
        set(value) = pixelSetYellowQuantum(value)
    private external fun pixelGetYellowQuantum(): Quantum
    private external fun pixelSetYellowQuantum(yellow: Quantum)
}
