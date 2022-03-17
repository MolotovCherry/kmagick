package com.cherryleafroad.kmagick

import java.io.Closeable

// base exception class for all imagemagick exceptions
open class MagickException(message: String) : Exception(message)

@Suppress("unused", "MemberVisibilityCanBePrivate")
object Magick : Closeable {
    init {
        // load library and start genesis
        System.loadLibrary("kmagick")
    }

    @Throws(MagickException::class)
    private external fun nativeInit()

    /**
     * Returns any font that matches the specified pattern (e.g. "*" for all).
     * Returns `null` and an exception if there was an error.
     */
    @Throws(MagickException::class)
    external fun magickQueryFonts(pattern: String): Array<String>?

    /**
     * Set the internal log level used. By default, a debug build = DEBUG log level,
     * and a release build = INFO log level. But you can change it or even turn it off.
     */
    fun setLogLevel(level: LogLevel) {
        nativeSetLogLevel(level.id)
    }
    private external fun nativeSetLogLevel(level: Int)

    /**
     * Initialize the environment - This ___MUST___ be called before calling anything else.
     * This ___IS NOT___ called automatically for you.
     *
     * ___REMEMBER___ to manually call [terminate] when you're finished to clean up.
     *
     * If you prefer something more idiomatic, you can try a `use` with resources block.
     * E.g. `Magick.initialize().use { }`
     */
    @Throws(MagickException::class)
    fun initialize(): Magick {
        nativeInit()
        return this
    }

    /**
     * Call this manually when you're finished to destruct the environment.
     * This ___WILL NOT___ be called automatically.
     *
     * If you would like to automatically call this, try a `use` with resources block.
     * E.g. `Magick.initialize().use { }`
     */
    external fun terminate()

    /**
     * This isn't meant to be called manually. You can call [terminate] instead. This does the
     * same thing as [terminate], but it's here to be used with a `use{}` block for
     * convenience. For example `Magick.initialize().use { }`
     */
    override fun close() {
        terminate()
    }

    /**
     * Checks whether the magick system was initialized
     */
    external fun isInitialized(): Boolean

    /**
     * Destroys all wands (same thing that happens when you call `terminate()`)
     * WARNING: DO NOT use the destroyed wands after. They are invalidated after that.
     */
    external fun destroyWands()

    /**
     * Destroys all wands with certain types that match ID's
     * WARNING: DO NOT use the destroyed wands after. They are invalidated after that.
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    @JvmName("destroyWandIdsType")
    internal external fun destroyWandIdsType(ids: ULongArray, wandType: Int)

    /**
     * Destroys all wands of a certain type
     * WARNING: DO NOT use the destroyed wands after. They are invalidated after that.
     */
    @JvmName("destroyWandType")
    internal external fun destroyWandType(wandType: Int)

    /**
     * Destroys a wand with a specific ID
     * WARNING: DO NOT use the destroyed wand after. It is invalidated after that.
     */
    @JvmName("destroyWandIdType")
    internal external fun destroyWandIdType(id: ULong, wandType: Int)

    /**
     * Destroys any kind of wand with a specific ID
     * WARNING: DO NOT use the destroyed wands after. They are invalidated after that.
     */
    @JvmName("destroyWandId")
    external fun destroyWandId(id: ULong)

    /**
     * Destroys any kind of wand whose id is contained in the array
     * WARNING: DO NOT use the destroyed wands after. They are invalidated after that.
     */
    @JvmName("destroyWandIds")
    @OptIn(ExperimentalUnsignedTypes::class)
    external fun destroyWandIds(ids: ULongArray)
}
