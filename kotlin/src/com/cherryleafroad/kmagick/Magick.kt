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

    private var initialized: Boolean = false

    @Throws(MagickException::class)
    private external fun nativeInit()
    @Throws(MagickException::class)
    private external fun nativeTerminate()

    /**
     * Returns any font that matches the specified pattern (e.g. "*" for all).
     * Returns `null` and an exception if there was an error.
     */
    @Throws(MagickException::class)
    external fun magickQueryFonts(pattern: String): Array<String>?

    /**
     * Initialize the environment - This ___MUST___ be called before calling anything else.
     * This ___IS NOT___ called automatically for you.
     *
     * ___REMEMBER___ to manually call [terminate] when you're finished to clean up.
     *
     * If you prefer something more idiomatic, you can try a `use` with resources block.
     * E.g. `Magick.initialize().use { }`
     */
    fun initialize(): Magick {
        nativeInit()
        initialized = true
        return this
    }

    /**
     * You ___MUST___ call this manually when you're finished to destruct the environment.
     * This ___WILL NOT___ be called automatically.
     *
     * If you would like to automatically call this, try a `use` with resources block.
     * E.g. `Magick.initialize().use { }`
     */
    fun terminate() {
        nativeTerminate()
        initialized = false
    }

    /**
     * This isn't meant to be called manually. You can call [terminate] instead. This does the
     * same thing as [terminate], but it's here to be used with a `use{}` block for
     * convenience. For example `Magick.initialize().use { }`
     */
    override fun close() {
        terminate()
    }
}
