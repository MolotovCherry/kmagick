package com.cherryleafroad.kmagick

/**
 * [NativeMagickException] holds an exception produced internally by ImageMagick.
 */
data class NativeMagickException(
    /**
     * The type of exception.
     */
    val exceptionType: ExceptionType,

    /**
     * A message explaining the exception.
     */
    val message: String
) {
    internal companion object {
        fun fromNative(id: Int, msg: String): NativeMagickException {
            val exceptionType = (ExceptionType::id::find)(id)!!
            return NativeMagickException(exceptionType, msg)
        }
    }
}
