package com.cherryleafroad.kmagick

/**
 * A measure of the differences between images according to the type given metric.
 */
enum class MetricType(internal val id: Int) {
    UndefinedErrorMetric(0),

    /**
     * absolute error count, number of different pixels (-fuzz affected)
     */
    AbsoluteErrorMetric(1),

    /**
     * mean color distance
     */
    FuzzErrorMetric(2),

    /**
     * mean absolute error (normalized), average channel error distance
     */
    MeanAbsoluteErrorMetric(3),

    /**
     * mean error per pixel (normalized mean error, normalized peak error)
     */
    MeanErrorPerPixelErrorMetric(4),

    /**
     * mean error squared, average of the channel error squared
     */
    MeanSquaredErrorMetric(5),

    /**
     * normalized cross correlation
     */
    NormalizedCrossCorrelationErrorMetric(6),

    /**
     * peak absolute (normalized peak absolute)
     */
    PeakAbsoluteErrorMetric(7),

    /**
     * peak signal to noise ratio
     */
    PeakSignalToNoiseRatioErrorMetric(8),

    /**
     * perceptual hash for the sRGB and HCLp colorspaces. Specify an alternative colorspace with -define
     */
    PerceptualHashErrorMetric(9),

    /**
     * root mean squared (normalized root mean squared)
     */
    RootMeanSquaredErrorMetric(10),

    /**
     * structural similarity index
     */
    StructuralSimilarityErrorMetric(11),

    /**
     * structural dissimilarity index
     */
    StructuralDissimilarityErrorMetric(12)
}
