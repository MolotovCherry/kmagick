package com.cherryleafroad.kmagick

data class NativeMagickException(
    val exceptionType: ExceptionType,
    val message: String
) {
    @Suppress("unused")
    internal companion object {
        fun fromNative(id: Int, msg: String): NativeMagickException {
            val exceptionType = (ExceptionType::id::find)(id)!!
            return NativeMagickException(exceptionType, msg)
        }
    }
}
