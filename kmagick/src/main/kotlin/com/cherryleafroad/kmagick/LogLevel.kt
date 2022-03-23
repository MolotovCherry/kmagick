package com.cherryleafroad.kmagick

/**
 * The internal log level.
 */
enum class LogLevel(internal val id: Int) {
    /**
     * No logs at all.
     */
    Off(0),

    /**
     * Only log Error level.
     */
    Error(1),

    /**
     * Log Warn level and above.
     */
    Warn(2),

    /**
     * Log Info level and above.
     */
    Info(3),

    /**
     * Log Debug level and above.
     */
    Debug(4),

    /**
     * Log Trace level and above.
     */
    Trace(5)
}
