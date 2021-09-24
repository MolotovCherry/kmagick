package com.cherryleafroad.kmagick

enum class MetricType(val id: Int) {
    UndefinedErrorMetric(0),
    AbsoluteErrorMetric(1),
    FuzzErrorMetric(2),
    MeanAbsoluteErrorMetric(3),
    MeanErrorPerPixelErrorMetric(4),
    MeanSquaredErrorMetric(5),
    NormalizedCrossCorrelationErrorMetric(6),
    PeakAbsoluteErrorMetric(7),
    PeakSignalToNoiseRatioErrorMetric(8),
    PerceptualHashErrorMetric(9),
    RootMeanSquaredErrorMetric(10),
    StructuralSimilarityErrorMetric(11),
    StructuralDissimilarityErrorMetric(12)
}
