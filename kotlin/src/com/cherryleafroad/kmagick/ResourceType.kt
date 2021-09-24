package com.cherryleafroad.kmagick

@Suppress("unused")
enum class ResourceType(val id: Int) {
    UndefinedResource(0),
    AreaResource(1),
    DiskResource(2),
    FileResource(3),
    HeightResource(4),
    MapResource(5),
    MemoryResource(6),
    ThreadResource(7),
    ThrottleResource(8),
    TimeResource(9),
    WidthResource(10),
    ListLengthResource(11)
}
