package com.cherryleafroad.kmagick

import org.objenesis.ObjenesisHelper

internal val magickWandInstantiator = ObjenesisHelper.getInstantiatorOf(MagickWand::class.java)
internal val pixelWandInstantiator = ObjenesisHelper.getInstantiatorOf(PixelWand::class.java)
internal val drawingWandInstantiator = ObjenesisHelper.getInstantiatorOf(DrawingWand::class.java)
