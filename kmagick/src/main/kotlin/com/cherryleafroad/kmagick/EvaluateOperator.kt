package com.cherryleafroad.kmagick

enum class EvaluateOperator(internal val id: Int) {
    Undefined(0),

    /**
     * Add value to pixels and return absolute value.
     */
    Abs(1),

    /**
     * Add value to pixels.
     */
    Add(2),

    /**
     * Add value to pixels modulo QuantumRange.
     */
    AddModulus(3),

    /**
     * Binary AND of pixels with value.
     */
    And(4),

    /**
     * 	Apply cosine to pixels with frequency value with 50% bias added.
     */
    Cosine(5),

    /**
     * Divide pixels by value.
     */
    Divide(6),

    /**
     * base-e exponential function
     */
    Exponential(7),
    GaussianNoise(8),
    ImpulseNoise(9),
    LaplacianNoise(10),

    /**
     * Shift the pixel values left by value bits (i.e., multiply pixels by 2^value).
     */
    LeftShift(11),

    /**
     * Apply scaled logarithm to normalized pixels.
     */
    Log(12),

    /**
     * Set pixels to maximum of value and current pixel value (i.e. set any pixels currently less than value to value).
     */
    Max(13),

    /**
     * Add the value and divide by 2.
     */
    Mean(14),

    /**
     * Choose the median value from an image sequence.
     */
    Median(15),

    /**
     * Set pixels to minimum of value and current pixel value (i.e. set any pixels currently greater than value to value).
     */
    Min(16),
    MultiplicativeNoise(17),

    /**
     * Multiply pixels by value.
     */
    Multiply(18),

    /**
     * Binary OR of pixels with value.
     */
    Or(19),
    PoissonNoise(20),

    /**
     * Raise normalized pixels to the power value.
     */
    Pow(21),

    /**
     * 	Shift the pixel values right by value bits (i.e., divide pixels by 2^value).
     */
    RightShift(22),

    /**
     * Square the pixel and add the value.
     */
    RootMeanSquare(23),

    /**
     * 	Set pixel equal to value.
     */
    Set(24),

    /**
     * Apply sine to pixels with frequency value with 50% bias added.
     */
    Sine(25),

    /**
     * Subtract value from pixels.
     */
    Subtract(26),
    Sum(27),

    /**
     * Threshold pixels to zero values equal to or below value.
     */
    ThresholdBlack(28),

    /**
     * 	Threshold pixels larger than value.
     */
    Threshold(29),

    /**
     * 	Threshold pixels to maximum values above value.
     */
    ThresholdWhite(30),
    UniformNoise(31),

    /**
     * Binary XOR of pixels with value.
     */
    Xor(32),

    /**
     * Apply inverse scaled logarithm to normalized pixels.
     */
    InverseLog(33);
}
