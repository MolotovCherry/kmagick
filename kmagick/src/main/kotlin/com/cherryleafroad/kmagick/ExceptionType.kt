package com.cherryleafroad.kmagick

/**
 * Represents an internal imagemagick exception type.
 */
enum class ExceptionType(internal val id: Int) {
    /**
     * Unset value.
     */
    UndefinedException(0),

    /**
     * A warning occurred.
     */
    WarningException(300),

    /**
     * A program resource is exhausted e.g. not enough memory.
     */
    ResourceLimitWarning(300),

    /**
     * A font is unavailable; a substitution may have occurred.
     */
    TypeWarning(305),

    /**
     * A command-line option was malformed.
     */
    OptionWarning(310),

    /**
     * An ImageMagick delegate failed to complete.
     */
    DelegateWarning(315),

    /**
     * The image type can not be read or written because the appropriate delegate is missing.
     */
    MissingDelegateWarning(320),

    /**
     * The image file may be corrupt.
     */
    CorruptImageWarning(325),

    /**
     * The image file could not be opened for reading or writing.
     */
    FileOpenWarning(330),

    /**
     * A binary large object could not be allocated, read, or written.
     */
    BlobWarning(335),

    /**
     * There was a problem reading or writing from a stream.
     */
    StreamWarning(340),

    /**
     * Pixels could not be read or written to the pixel cache.
     */
    CacheWarning(345),

    /**
     * There was a problem with an image coder.
     */
    CoderWarning(350),
    FilterWarning(352),

    /**
     * There was a problem with an image module.
     */
    ModuleWarning(355),

    /**
     * A drawing operation failed.
     */
    DrawWarning(360),

    /**
     * The operation could not complete due to an incompatible image.
     */
    ImageWarning(365),

    /**
     * There was a problem specific to the MagickWand API.
     */
    WandWarning(370),

    /**
     * There is a problem generating a true or pseudo-random number.
     */
    RandomWarning(375),

    /**
     * An X resource is unavailable.
     */
    XServerWarning(380),

    /**
     * There was a problem activating the progress monitor.
     */
    MonitorWarning(385),

    /**
     * There was a problem getting or setting the registry.
     */
    RegistryWarning(390),

    /**
     * There was a problem getting a configuration file.
     */
    ConfigureWarning(395),

    /**
     * A policy denies access to a delegate, coder, filter, path, or resource.
     */
    PolicyWarning(399),

    /**
     * An error occurred.
     */
    ErrorException(400),

    /**
     * A program resource is exhausted e.g. not enough memory.
     */
    ResourceLimitError(400),

    /**
     * A font is unavailable; a substitution may have occurred.
     */
    TypeError(405),

    /**
     * A command-line option was malformed.
     */
    OptionError(410),

    /**
     * An ImageMagick delegate failed to complete.
     */
    DelegateError(415),

    /**
     * The image type can not be read or written because the appropriate; delegate is missing.
     */
    MissingDelegateError(420),

    /**
     * The image file may be corrupt.
     */
    CorruptImageError(425),

    /**
     * The image file could not be opened for reading or writing.
     */
    FileOpenError(430),

    /**
     * A binary large object could not be allocated, read, or written.
     */
    BlobError(435),

    /**
     * There was a problem reading or writing from a stream.
     */
    StreamError(440),

    /**
     * Pixels could not be read or written to the pixel cache.
     */
    CacheError(445),

    /**
     * There was a problem with an image coder.
     */
    CoderError(450),
    FilterError(452),

    /**
     * There was a problem with an image module.
     */
    ModuleError(455),

    /**
     * A drawing operation failed.
     */
    DrawError(460),

    /**
     * The operation could not complete due to an incompatible image.
     */
    ImageError(465),

    /**
     * There was a problem specific to the MagickWand API.
     */
    WandError(470),

    /**
     * There is a problem generating a true or pseudo-random number.
     */
    RandomError(475),

    /**
     * An X resource is unavailable.
     */
    XServerError(480),

    /**
     * There was a problem activating the progress monitor.
     */
    MonitorError(485),

    /**
     * There was a problem getting or setting the registry.
     */
    RegistryError(490),

    /**
     * There was a problem getting a configuration file.
     */
    ConfigureError(495),

    /**
     * A policy was denied access to a delegate, coder, filter, path, or resource.
     */
    PolicyError(499),

    /**
     * A fatal error occurred.
     */
    FatalErrorException(700),

    /**
     * A program resource is exhausted e.g. not enough memory.
     */
    ResourceLimitFatalError(700),

    /**
     * A font is unavailable; a substitution may have occurred.
     */
    TypeFatalError(705),

    /**
     * A command-line option was malformed.
     */
    OptionFatalError(710),

    /**
     * An ImageMagick delegate failed to complete.
     */
    DelegateFatalError(715),

    /**
     * The image type can not be read or written because the appropriate; delegate is missing.
     */
    MissingDelegateFatalError(720),

    /**
     * The image file may be corrupt.
     */
    CorruptImageFatalError(725),

    /**
     * The image file could not be opened for reading or writing.
     */
    FileOpenFatalError(730),

    /**
     * A binary large object could not be allocated, read, or written.
     */
    BlobFatalError(735),

    /**
     * There was a problem reading or writing from a stream.
     */
    StreamFatalError(740),

    /**
     * Pixels could not be read or written to the pixel cache.
     */
    CacheFatalError(745),

    /**
     * There was a problem with an image coder.
     */
    CoderFatalError(750),
    FilterFatalError(752),

    /**
     * There was a problem with an image module.
     */
    ModuleFatalError(755),

    /**
     * A drawing operation failed.
     */
    DrawFatalError(760),

    /**
     * The operation could not complete due to an incompatible image.
     */
    ImageFatalError(765),

    /**
     * There was a problem specific to the MagickWand API.
     */
    WandFatalError(770),

    /**
     * There is a problem generating a true or pseudo-random number.
     */
    RandomFatalError(775),

    /**
     * An X resource is unavailable.
     */
    XServerFatalError(780),

    /**
     * There was a problem activating the progress monitor.
     */
    MonitorFatalError(785),

    /**
     * There was a problem getting or setting the registry.
     */
    RegistryFatalError(790),

    /**
     * There was a problem getting a configuration file.
     */
    ConfigureFatalError(795),

    /**
     * A policy denies access to a delegate, coder, filter, path, or resource.
     */
    PolicyFatalError(799)
}
