package com.example.cli

import com.cherryleafroad.kmagick.*

fun main() {
    // close resources when finished
    Magick.initialize().use {
        val wand = MagickWand()

        val pw = PixelWand()
        pw.color = "#ff0000"
        wand.newImage(500, 500, pw)

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
        
       // ALWAYS make sure to handle your exceptions! Something may always go wrong
        try {
            // this will obviously fail
            wand.readImage("oops!")
        } catch (e: MagickWandException) {
            // handle it here
            val exc = wand.getException()
            println("Got a MagickWandException: ${e.message}")
            println("Extra exception details: ${exc.exceptionType}: ${exc.message}")
        }
        
        // But what if the library itself panics? (this is what we call a crash)
        // then a RuntimeException will be thrown instead
        // A RuntimeException can theoretically occur on ANY library function call
        // however the chances of that actually happening are very low
        try {
            // pretend that the following fn exists and causes a panic, so throws a RuntimeException
            // wand.panic()
        } catch (e: RuntimeException) {
            println("Something bad happened: ${e.message}")
        }

        wand.writeImage("D:/out.png")
    }
}
