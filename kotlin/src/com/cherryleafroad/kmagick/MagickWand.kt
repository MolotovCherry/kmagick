package com.cherryleafroad.kmagick

class MagickWandException(message: String) : MagickException(message)

class MagickWand {
    init {
        new()
    }

    /**
     * Holds the pointer to internal object in memory.
     */
    private var HANDLE: Long = 0

    /**
     * Call the internal function to create the new wand.
     */
    private external fun new()

    /**
     * Check to see if this is still the correct wand.
     */
    external fun isWand(): Boolean

    /**
     * Clone the wand into a new one.
     */
    external fun clone(): MagickWand

    /**
     * While this automatically gets called by the `finalize()` destructor,
     * `finalize()` is not guaranteed to be called at all, nor called on time.
     * It's recommended to manually destroy all wands when finished.
     */
    external fun destroy()

    /**
     * While this is here to automatically call the destructor, due to
     * the way Kotlin/Java works, it's not guaranteed to be called at all,
     * or called on time. It is not recommended to rely on this to destroy
     * the wand consistently/timely.
     */
    protected fun finalize() {
        destroy()
    }
}
