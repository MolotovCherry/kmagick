# KMagick

[![Build](https://github.com/MolotovCherry/kmagick/actions/workflows/build.yml/badge.svg?event=push)](https://github.com/MolotovCherry/kmagick/actions/workflows/build.yml) [![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/MolotovCherry/kmagick)](https://github.com/MolotovCherry/kmagick/releases) [![Docs](https://img.shields.io/badge/docs-v0.2.1-orange)](https://molotovcherry.github.io/kmagick/kmagick/com.cherryleafroad.kmagick/index.html)

Kotlin bindings for ImageMagick; uses the ImageMagick wand API.

## Supported Platforms
Windows and Android*

\* Others may work too, but I have not tested Mac or Linux.

## Download
All downloads are in the [releases section](https://github.com/MolotovCherry/kmagick/releases).

## Setup

### Android
A full example of the below setup can be found [here](https://github.com/MolotovCherry/kmagick/tree/main/example/android-setup)

1. Grab the jar and sources jar.
2. Add this line to your dependencies: `implementation fileTree(dir: 'libs', include: ['*.jar'])`
3. Place the jars in the `app/libs` folder.
4. Place the [Android ImageMagick shared library](https://github.com/MolotovCherry/Android-ImageMagick7/releases) `so` files in your `app/src/main/jniLibs` folder along with the Android `kmagick.so` library.
5. Either [download](http://objenesis.org/download.html) objenesis jar and place it in the libs folder, OR add this line to your dependencies:  
`implementation 'org.objenesis:objenesis:3.2'`

Debug messages can be found in Android logcat under the id `MAGICK`. Make sure you first set the appropriate `LogLevel` to see them.

\* I plan to add a Maven config sometime, but I've been too busy and tired.

(The Android ImageMagick library can be found [here](https://github.com/MolotovCherry/Android-ImageMagick7))

### Windows
1. Grab the `kmagick.dll` file along with the jar and sources jar.
2. Install Windows ImageMagick (dll version) and make sure the program files folder is in your `PATH`.
3. Setup your project to use the jar as normal.
4. Make sure `kmagick.dll` is in your path as well.
5. Either [download](http://objenesis.org/download.html) objenesis jar and place it along with your other jars, OR add this line (or similar depending on your build system) to your dependencies:  
`implementation 'org.objenesis:objenesis:3.2'`

## Documentation
You can browse the latest docs [here](https://molotovcherry.github.io/kmagick/kmagick/com.cherryleafroad.kmagick/index.html)

⚠️ Please remember that KMagick is merely a thin wrapper around the C library ImageMagick functions. So anything you can do through the C api will translate almost directly. Any behavior that results from using the bindings is most likely imagemagick itself and how the imagemagick api was used (but if not, please report an issue). If you need to do a specific task, please check the imagemagick documentation / ask them how to achieve it through the C api. Any help and issues reported here should be related to kmagick itself, not imagemagick's api.

## Behavior
As this is a low level library, crashes are not impossible. I've made every effort to make that impossible however.

If there is a low-level error/crash, this library will catch it and throw a java runtime exception instead of fatally crashing the JVM (which is what would normally happen if it was C!). Additionally, nearly all the functions in here, if they encounter a problem, will throw a related exception type (e.g. `PixelWandException`, `MagickWandException`, etc) along with a helpful human readable message. <ins>**You should always check for these just in case something makes your code fail**</ins>.

If there is a `panic` (a `crash` in Rust terms), please submit an issue along with reproducible steps so I can fix it. Unfortunately due to limitations the message might be a little cryptic, but provide as much info as you can (along with reproducible steps).

I have no control if the C code segfaults your project. Sorry. If there is one, that could be an ImageMagick bug and you should report it to them. However, make sure to report it here too just in case it's a bug in kmagick.

## API and Examples
First of all, check out the official [ImageMagick](https://imagemagick.org/script/magick-wand.php) function reference. If you have any confusion/questions, it'll be answered there. Also, the sources jar contains comments for every function which should be good enough in most cases.

<ins>Note</ins>: The majority of API functions throw exceptions if they fail. The ones used are `java/lang/RuntimeException` and a related `com/MolotovCherry/kmagick/*Exception` (check the function for which exception it may return). It is strongly recommended you handle all exceptions if you don't want your program/app to crash! I realize this is Kotlin and exceptions aren't fun to handle, but because this is a low level library, many things can go wrong! Please see the Exception section below to find out how to get Exception details so you can know why it's happening to you.

There's an [example](/example/src/main/kotlin/com/example/cli/Main.kt) under the `example` directory as well.  
The example shows all the different kmagick features to be aware of (which aren't all covered below).

You can use the included `run-example.bat` to build and run the example (note: default example is a tutorial and won't run without editing first).  
Remember that you need `kmagick.dll` in your `PATH` (or same cwd) for it to work
```kotlin
// Basic usage

// You MUST call Magick.initialize() before you can use the library.
Magick.initialize()

// You can also use this with a `use` block to automatically terminate at the end
Magick.initialize().use {
  // do your stuff
}

// when you're done, you should call terminate. The `use` block above does that automatically for you.
// Note: When you call this, ALL your wands will immediately be invalidated at the C level.
// DO NOT attempt to use them after or you'll get an exception.
Magick.terminate()

Magick.initialize().use {
    val a = PixelWand()
    a.color = "blue"
    
    val b = MagickWand()
    b.newImage(100, 200, a)
    
    // if you so desire, you can also destroy your wand in advance
    // just don't attempt to use it afterwards.
    // java might not guarantee the destructor will be called on finalize(),
    // so you might have to call this yourself to keep memory sane
    a.destroy()
    
    // wands also can use the `use` blocks if needed
    // just remember it'll be destroyed at the end of the block!
    b.use {
        it.readImage("/some/path/file.png")
    }
    
    // Any wand method may throw an exception, so make sure to handle them
    try {
        b.getImageHistogram()
    } catch (e: MagickWandException) {
        println("exception: ${e.message}")
    }
} // all wands auto destroyed here due to `Magick.terminate()`
```

### Exceptions

If you encounter an exception and need to know what happened, all `Wand` classes define 3 functions to help you out:
```
clearException()
getExceptionType()
getException()
```
You can cross reference the [`ExceptionType`](https://molotovcherry.github.io/kmagick/kmagick/com.cherryleafroad.kmagick/-exception-type/index.html) in the source code to see what exact error it was. `getException()` also returns [`NativeMagickException`](https://molotovcherry.github.io/kmagick/kmagick/com.cherryleafroad.kmagick/-native-magick-exception/index.html) which will give you both an `ExceptionType` and a message explaining what happened.

For more examples and information on usage, please browse the API in your IDE or check ImageMagick's website.

## Reporting issues
If you encountered a problem, please first check if you can find out what it is by running `getException()` and checking the details.  

If that gives you no solution, then please check whether it's merely your usage of the api. **_Some things may appear to be bugs, but rather are in fact not bugs; just merely it's how imagemagick works, and you need to follow the imagemagick api. Kmagick is only a direct binding to the imagemagick c api and does not alter your images in any way. So make sure to check and understand the imagemagick c api so you can do things the way imagemagick expects you to_**  

If this doesn't help and it's a real bug, then please check your `logcat` for your device and include the error log in the issue you make.

## Missing an API Function?

We're using `magick-rust` for our bindings. So, if some API that you need is missing, please make a feature request or send a PR to [magick-rust](https://github.com/nlfiedler/magick-rust) and it'll make its way downstream to me.

## Contributions
Contributions are welcome! If you have an improvement, please send a PR or make an issue about it and I'll see what we can do. 😉 If you know Rust, contributions are even more welcome, especially to [magick-rust](https://github.com/nlfiedler/magick-rust) (because it'll make the functions available downstream for me).

If you want to contribute Rust code to this project, check out the [rust folder](https://github.com/MolotovCherry/kmagick/tree/main/rust) which has directions on how to compile it locally.

## Rust devs - a note for you all
In the `Rust` directory, the two crates `jni-macros` and `jni-tools` offers some FULLY working macros which generate JNI bindings for Rust functions. Of course, it's only Kotlin compatible (no Java; although you _could_ edit the handle fn's to fix that). You can even use regular impl's which KEEP STATE between calls as if it was a real class instance! The `jni-tools` crate offers the visible public API for it. There's also docs on it to explain its usage, however, if you want to fully know how to use it, you should take a look at my Rust and Kotlin code as the prime example.

# Did this library help you?

[![Donate](https://raw.githubusercontent.com/MolotovCherry/Android-ImageMagick7/master/readme_files/donate.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=BKDN933UM444J)

If you found this library useful, please consider showing appreciation and help fund it by sending a donation my way.  
All donations help this project continue to be supported for longer and receive more frequent updates! Thanks for your support! <3

## 

<p>
  <div style="vertical-align: baseline;">This project is proudly supported by JetBrains OSS License</div>
  <a href="https://jb.gg/OpenSourceSupport"><img src="https://github.com/MolotovCherry/kmagick/blob/main/readme_files/jb_beam.png" height="150px" width="150px"/></a>
</p>
