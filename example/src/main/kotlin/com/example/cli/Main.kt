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

        wand.writeImage("D:/out.png")
    }
}
