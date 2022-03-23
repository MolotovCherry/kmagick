package com.cherryleafroad.kmagick

/**
 * FilterTypes are used to adjust the filter algorithm used when resizing images. Different filters experience varying
 * degrees of success with various images and can take sipngicantly different amounts of processing time. ImageMagick
 * uses the [LanczosFilter] by default since this filter has been shown to provide the best results for most images in a
 * reasonable amount of time. Other filter types (e.g. [TriangleFilter]) may execute much faster but may show artifacts
 * when the image is re-sized or around diagonal lines. The only way to be sure is to test the filter with sample images.
 */
enum class FilterType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedFilter(0),

    /**
     * Point Filter
     */
    PointFilter(1),

    /**
     * Box Filter
     */
    BoxFilter(2),

    /**
     * Triangle Filter
     */
    TriangleFilter(3),

    /**
     * Hermite Filter
     */
    HermiteFilter(4),

    /**
     * Hann Filter
     */
    HannFilter(5),

    /**
     * Hamming Filter
     */
    HammingFilter(6),

    /**
     * Blackman Filter
     */
    BlackmanFilter(7),

    /**
     * Gaussian Filter
     */
    GaussianFilter(8),

    /**
     * Quadratic Filter
     */
    QuadraticFilter(9),

    /**
     * Cubic Filter
     */
    CubicFilter(10),

    /**
     * Catrom Filter
     */
    CatromFilter(11),

    /**
     * Mitchell Filter
     */
    MitchellFilter(12),

    /**
     * Jinc Filter
     */
    JincFilter(13),

    /**
     * Sinc Filter
     */
    SincFilter(14),

    /**
     * SincFast Filter
     */
    SincFastFilter(15),

    /**
     * Kaiser Filter
     */
    KaiserFilter(16),

    /**
     * Welch Filter
     */
    WelchFilter(17),

    /**
     * Parzen Filter
     */
    ParzenFilter(18),

    /**
     * Bohman Filter
     */
    BohmanFilter(19),

    /**
     * Bartlett Filter
     */
    BartlettFilter(20),

    /**
     * Lagrange Filter
     */
    LagrangeFilter(21),

    /**
     * Lanczos Filter
     */
    LanczosFilter(22),

    /**
     * LanczosSharp Filter
     */
    LanczosSharpFilter(23),

    /**
     * Lanczos2 Filter
     */
    Lanczos2Filter(24),

    /**
     * Lanczos2Sharp Filter
     */
    Lanczos2SharpFilter(25),

    /**
     * Robidoux Filter
     */
    RobidouxFilter(26),

    /**
     * RobidouxSharp Filter
     */
    RobidouxSharpFilter(27),

    /**
     * Cosine Filter
     */
    CosineFilter(28),

    /**
     * Spline Filter
     */
    SplineFilter(29),

    /**
     * LanczosRadius Filter
     */
    LanczosRadiusFilter(30),

    /**
     * CubicSpline Filter
     */
    CubicSplineFilter(31),

    /**
     * Sentinel Filter
     */
    SentinelFilter(32)
}
