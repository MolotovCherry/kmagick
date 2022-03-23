package com.cherryleafroad.kmagick

/**
 * The EndianType enumerations are used to specify the endian option for formats which support it (e.g. TIFF).
 */
enum class EndianType(internal val id: Int) {
    /**
     * Not defined (default)
     */
    UndefinedEndian(0),

    /**
     * Little endian (like Intel X86 and DEC Alpha)
     */
    LSBEndian(1),

    /**
     * Big endian (like Motorola 68K, Mac PowerPC, & SPARC)
     */
    MSBEndian(2)
}
