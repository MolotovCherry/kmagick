package com.example.cli

import com.cherryleafroad.kmagick.*

//
// This isn't a runnable example, but is rather meant to be a tutorial of all relevant features
// If you do try to run it without editing it, it will throw an exception
//

@OptIn(ExperimentalUnsignedTypes::class)
fun main() {
    // close resources when finished
    Magick.initialize().use {
        // initialize new wand
        val wand = MagickWand()

        // initialize pixelwand and change the color, then use it to change the color of newImage
        val pw = PixelWand()
        pw.color = "#ff0000"
        wand.newImage(500, 500, pw)

        // get an image pixel color, and check what it is from the returned pixelwand
        wand.getImagePixelColor(100, 100)?.let {
            //srgb(255,0,0)
            println(it.color)
        }

        val dwand = DrawingWand()
        dwand.fontSize = 40.0
        dwand.font = "Times New Roman"

        val colorWand = PixelWand()
        colorWand.color = "black"
        dwand.fillColor = colorWand
        wand.annotateImage(dwand, 0.0,50.0, 0.0, "Some text")

        // get bytes of image, then read it back in
        val blob = wand.writeImageBlob("png")
        wand.readImageBlob(blob)

        // ALWAYS make sure to handle your exceptions! Most methods will throw an exception when there's an error
        // if you don't handle these cases, your app will crash due to an unhandled exception
        try {
            // this will obviously fail
            wand.readImage("oops!")
        // if you want to catch all of them, the base exception is MagickException
        } catch (e: MagickWandException) {
            // handle it here
            // this method will get the native exception details from ImageMagick and
            // give you more info than the java exception gives you
            val exc = wand.getException()
            println("Got a MagickWandException: ${e.message}")
            println("Extra exception details: ${exc.exceptionType}: ${exc.message}")
        }

        // But what if the library itself panics? (this is what we call a crash)
        // then a RuntimeException will be thrown instead.
        // while you CAN handle this, it's probably better that you don't.
        // because a crash is an internal error, and your app shouldn't continue to run when there's a problem like this.
        //
        // A RuntimeException can theoretically occur on ANY library function call
        // however the chances of that actually happening are very low, and if it does,
        // it's probably a BUG (please report it ASAP)
        try {
            // pretend that the following fn exists and causes a panic, so throws a RuntimeException
            // wand.panic()
        } catch (e: RuntimeException) {
            println("Something bad happened: ${e.message}")
        }

        // ALWAYS destroy your wands when done - If you don't, they will keep using up valuable memory
        // JVM can't GC these because they need to be kept alive. And unfortunately, Java's `finalize()` is very
        // unreliable, so we can't use that (it even makes your objects slower)
        // sadly, kotlin doesn't have a decent way to auto close multiple resources either
        //
        // it's unlikely you can rely on `Magick.initialize().use` for your entire app since you'll probably
        // need magick for awhile. so you can't rely on `Magick.terminate()` to destroy them all for you,
        // because it unitializes ImageMagick
        // There are other ways you can destroy the wands yourself

        // the simplest way
        wand.destroy()

        // destroy all wands in entire app. `Magick.terminate()` also does this (while uninitializing imagemagick)
        Magick.destroyWands()

        // Every wand class also includes specific methods to destroy wands
        MagickWand.destroyWands() // destroy all magickwands (but leaving other types alone)
        val id = wand.id // get internal id of wand
        MagickWand.destroyWandId(id) // destroy a specific wand by ID
        // you can also batch destroy wands with a ULongArray
        MagickWand.destroyWandIds(ulongArrayOf(0u, 1u))

        // This will raise an exception. Why?
        // One: the path doesn't exist
        // Two: the wand is uninitialized cause we destroyed it!
        // DO NOT use any destroyed wands, cause they are invalidated after
        wand.readImage("Foo")

        // write result out
        wand.writeImage("D:/out.png")
    }
}
