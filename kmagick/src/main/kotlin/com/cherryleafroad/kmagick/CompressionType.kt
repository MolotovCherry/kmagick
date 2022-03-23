package com.cherryleafroad.kmagick

/**
 * CompressionType is used to express the desired compression type when encoding an image. Be aware that most image
 * types only support a sub-set of the available compression types. If the compression type specified is incompatable
 * with the image, ImageMagick selects a compression type compatable with the image type.
 */
enum class CompressionType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedCompression(0),

    B44ACompression(1),
    B44Compression(2),

    /**
     * BZip (Burrows-Wheeler block-sorting text compression algorithm and Huffman coding) as used by bzip2 utilities
     */
    BZipCompression(3),
    DXT1Compression(4),
    DXT3Compression(5),
    DXT5Compression(6),

    /**
     * CCITT Group 3 FAX compression
     */
    FaxCompression(7),

    /**
     * CCITT Group 4 FAX compression (used only for TIFF)
     */
    Group4Compression(8),
    JBIG1Compression(9),
    JBIG2Compression(10),
    JPEG2000Compression(11),

    /**
     * JPEG compression
     */
    JPEGCompression(12),
    LosslessJPEGCompression(13),
    LZMACompression(14),

    /**
     * Lempel-Ziv-Welch (LZW) compression (caution, patented by Unisys)
     */
    LZWCompression(15),

    /**
     * No compression
     */
    NoCompression(16),
    PizCompression(17),
    Pxr24Compression(18),

    /**
     * Run-Length encoded (RLE) compression
     */
    RLECompression(19),

    /**
     * Lempel-Ziv compression (LZ77) as used in PKZIP and GNU gzip.
     */
    ZipCompression(20),
    ZipSCompression(21),

    /**
     * Zstd compression (https://facebook.github.io/zstd/).
     */
    ZstdCompression(22),

    /**
     * WebP compression.
     */
    WebPCompression(23),
    DWAACompression(24),
    DWABCompression(25),
    BC7Compression(26)
}
