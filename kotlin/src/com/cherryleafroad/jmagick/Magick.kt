package com.cherryleafroad.jmagick

// base exception class for all imagemagick exceptions
class MagickException(message: String) : Exception(message)

object Magick {
    init {
        // load library and start genesis
        System.loadLibrary("jmagick")
        nativeInit()
    }

    private external fun nativeInit();
    private external fun magickWandGenesis()
    private external fun magickWandTerminus()

    @Throws(MagickException::class)
    external fun magickQueryFonts(pattern: String): List<String>

    fun initialize() {
        // note: While we do genesis, there is NO destructor for terminus!
        // This MUST be called manually by the user
        magickWandGenesis()
    }

    fun terminate() {
        magickWandTerminus()
    }
}
