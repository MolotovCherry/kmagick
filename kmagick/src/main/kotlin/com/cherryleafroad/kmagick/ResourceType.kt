package com.cherryleafroad.kmagick

/**
 * Useful for dynamically reducing system resources before attempting risky, or slow running, Image operations.
 */
enum class ResourceType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedResource(0),

    /**
     * Maximum width * height of a pixel cache before writing to disk.
     */
    AreaResource(1),

    /**
     * Maximum bytes used by pixel cache on disk before exception is thrown.
     */
    DiskResource(2),

    /**
     * Maximum cache files opened at any given time.
     */
    FileResource(3),

    /**
     * Maximum height of image before exception is thrown.
     */
    HeightResource(4),

    /**
     * Maximum memory map in bytes to allocated for pixel cache before using disk.
     */
    MapResource(5),

    /**
     * Maximum bytes to allocated for pixel cache before using disk.
     */
    MemoryResource(6),

    /**
     * Maximum parallel task sub-routines can spawn - if using OpenMP.
     */
    ThreadResource(7),

    /**
     * Total milliseconds to yield to CPU - if possible.
     */
    ThrottleResource(8),

    /**
     * Maximum seconds before exception is thrown.
     */
    TimeResource(9),

    /**
     * Maximum width of image before exception is thrown.
     */
    WidthResource(10),

    /**
     * Maximum images in sequence.
     */
    ListLengthResource(11)
}
