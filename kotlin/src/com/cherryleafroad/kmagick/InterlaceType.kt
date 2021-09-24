package com.cherryleafroad.kmagick

@Suppress("unused")
enum class InterlaceType(val id: Int) {
    UndefinedInterlace(0),
    NoInterlace(1),
    LineInterlace(2),
    PlaneInterlace(3),
    PartitionInterlace(4),
    GIFInterlace(5),
    JPEGInterlace(6),
    PNGInterlace(7)
}
