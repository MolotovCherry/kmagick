package com.cherryleafroad.kmagick

/**
 * The internal log level.
 *
 * @property Off No logs at all.
 * @property Error Corresponds to the `Error` log level.
 * @property Warn Corresponds to the `Warn` log level.
 * @property Info Corresponds to the `Info` log level.
 * @property Debug Corresponds to the `Debug` log level.
 * @property Trace Corresponds to the `Trace` log level.
 */
@Suppress("unused")
enum class LogLevel(val id: Int) {
    Off(0),
    Error(1),
    Warn(2),
    Info(3),
    Debug(4),
    Trace(5)
}
