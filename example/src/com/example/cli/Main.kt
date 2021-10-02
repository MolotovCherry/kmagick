package com.example.cli

import com.cherryleafroad.kmagick.Magick
import com.cherryleafroad.kmagick.MagickWand

fun main() {
    Magick.initialize().use {
        println("new wand")
        var wand = MagickWand()
        //wand.new()
        println("destroyed wand")
        wand.destroy()
        println("made new wand -> handle")
        wand = MagickWand()
        println("cloning wand")
        val newWand = wand.clone()
        println("cloned handle")
    }
}
