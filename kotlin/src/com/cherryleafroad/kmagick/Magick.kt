package com.cherryleafroad.kmagick

// base exception class for all imagemagick exceptions
open class MagickException(message: String) : Exception(message)

object Magick {
    init {
        // load library and start genesis
        System.loadLibrary("jmagick")
    }

    private external fun nativeInit();
    private external fun nativeTerminate();

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
