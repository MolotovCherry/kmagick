package com.cherryleafroad.kmagick

@Suppress("unused")
enum class CompressionType(val id: Int) {
    UndefinedCompression(0),
    B44ACompression(1),
    B44Compression(2),
    BZipCompression(3),
    DXT1Compression(4),
    DXT3Compression(5),
    DXT5Compression(6),
    FaxCompression(7),
    Group4Compression(8),
    JBIG1Compression(9),
    JBIG2Compression(10),
    JPEG2000Compression(11),
    JPEGCompression(12),
    LosslessJPEGCompression(13),
    LZMACompression(14),
    LZWCompression(15),
    NoCompression(16),
    PizCompression(17),
    Pxr24Compression(18),
    RLECompression(19),
    ZipCompression(20),
    ZipSCompression(21),
    ZstdCompression(22),
    WebPCompression(23),
    DWAACompression(24),
    DWABCompression(25),
    BC7Compression(26)
}
