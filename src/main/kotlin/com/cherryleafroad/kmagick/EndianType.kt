package com.cherryleafroad.kmagick

@Suppress("unused")
enum class EndianType(val id: Int) {
    UndefinedEndian(0),
    LSBEndian(1),
    MSBEndian(2)
}
