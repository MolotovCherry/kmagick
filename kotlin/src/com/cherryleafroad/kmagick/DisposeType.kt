package com.cherryleafroad.kmagick

@Suppress("unused")
enum class DisposeType(val id: Int) {
    UnrecognizedDispose(0),
    UndefinedDispose(0),
    NoneDispose(1),
    BackgroundDispose(2),
    PreviousDispose(3)
}
