package com.cherryleafroad.kmagick

// base exception class for all imagemagick exceptions
open class MagickException(message: String) : Exception(message)

class Magick {
    init {
        // load library and start genesis
        System.loadLibrary("kmagick")
    }

    var handle: Long? = null

    @Throws(MagickException::class)
    private external fun nativeInit()
    @Throws(MagickException::class)
    private external fun nativeTerminate()

    @Throws(MagickException::class)
    external fun magickQueryFonts(pattern: String): Array<String>?

    /**
     * Initialize the environment
     * REMEMBER to manually call `terminate()` when you're finished to clean ip
     */
    fun initialize() {
        // note: While we do genesis, there is NO destructor for terminus!
        // This MUST be called manually by the user
        nativeInit()
    }

    /**
     * You MUST call this manually when you're finished to destruct the environment
     * This WILL NOT be called automatically
     */
    fun terminate() {
        nativeTerminate()
    }
}
