package com.cherryleafroad.kmagick

/**
 * `GravityType` specifies positioning of an object (e.g. text, image) within a bounding region (e.g. an image).
 * Gravity provides a convenient way to locate objects irrespective of the size of the bounding region, in other
 * words, you don't need to provide absolute coordinates in order to position an object. A common default for gravity
 * is [NorthWestGravity].
 */
enum class GravityType(internal val id: Int) {
    /**
     * Don't use gravity.
     */
    UndefinedGravity(0),

    /**
     * Don't use gravity.
     */
    ForgetGravity(0),

    /**
     * Position object at top-left of region.
     */
    NorthWestGravity(1),

    /**
     * Postiion object at top-center of region
     */
    NorthGravity(2),

    /**
     * Position object at top-right of region
     */
    NorthEastGravity(3),

    /**
     * Position object at left-center of region
     */
    WestGravity(4),

    /**
     * Position object at center of region
     */
    CenterGravity(5),

    /**
     * Position object at right-center of region
     */
    EastGravity(6),

    /**
     * Position object at left-bottom of region
     */
    SouthWestGravity(7),

    /**
     * Position object at bottom-center of region
     */
    SouthGravity(8),

    /**
     * Position object at bottom-right of region
     */
    SouthEastGravity(9);

    internal companion object {
        fun fromNative(id: Int): GravityType {
            return (GravityType::id::find)(id)!!
        }
    }
}
