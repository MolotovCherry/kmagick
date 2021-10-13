# KMagick

ImageMagick bindings for Kotlin; uses the ImageMagick wand API.

## Supported Platforms
Windows and Android*

\* Others may work too, but I have not tested Mac or Linux.

## Download
All downloads are in the [releases section](https://github.com/cherryleafroad/kmagick/releases).

## Setup

### Android
1. Grab the jar and sources jar and put it in your project.
2. Add this line to your dependencies: `implementation fileTree(dir: 'libs', include: ['*.jar'])`
3. Place the jars in the `app/libs` folder.
4. Place ImageMagick shared so files in your `app/src/main/jniLibs` folder along with the Android `kmagick.so` library in the download section.

\* I plan to add a Maven config sometime, but I've been too busy and tired.

### Windows
1. Grab the `kmagick.dll` file along with the jar and sources jar.
2. Install Windows ImageMagick (dll version) and make sure the program files folder is in your `PATH`.
3. Setup your project to use the jar as normal.
4. Make sure `kmagick.dll` is in your path as well.

## API and Examples
First of all, check out the official [ImageMagick](https://imagemagick.org/script/magick-wand.php) function reference. If you have any confusion/questions, it'll be answered there. Also, the sources jar contains comments for every function which should be good enough in most cases.

There's an example under the `example` directory as well.
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
}
```

For more examples and information on usage, please browse the API in your IDE or check ImageMagick's website.

## Missing an API Function?

They're actually not that hard to add! If you need one that's missing, go check out the [rust imagemagick bindings crate](https://github.com/nlfiedler/magick-rust) and consider making an issue or sending them a PR. It'll make its way downstream to me and I can add it here.

## Contributions
Contributions are welcome! If you have an improvement, please send a PR or make an issue about it and I'll see what we can do. 😉 If you know Rust, contributions are even more welcome, especially to the [ImageMagick Rust bindings crate](https://github.com/nlfiedler/magick-rust) (because it'll make the functions available downstream for me).
