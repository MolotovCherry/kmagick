package com.cherryleafroad.kmagick

import org.objenesis.ObjenesisStd

internal val objenesis = ObjenesisStd()
internal val magickWandInstantiator = objenesis.getInstantiatorOf(MagickWand::class.java)
internal val pixelWandInstantiator = objenesis.getInstantiatorOf(PixelWand::class.java)
internal val drawingWandInstantiator = objenesis.getInstantiatorOf(DrawingWand::class.java)
