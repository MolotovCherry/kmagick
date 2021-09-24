package com.cherryleafroad.kmagick

enum class PixelInterpolateMethod(val id: Int) {
    UndefinedInterpolatePixel(0),
    AverageInterpolatePixel(1),
    Average9InterpolatePixel(2),
    Average16InterpolatePixel(3),
    BackgroundInterpolatePixel(4),
    BilinearInterpolatePixel(5),
    BlendInterpolatePixel(6),
    CatromInterpolatePixel(7),
    IntegerInterpolatePixel(8),
    MeshInterpolatePixel(9),
    NearestInterpolatePixel(10),
    SplineInterpolatePixel(11)
}
