wand_wrapper!(PixelWand);

magick_bindings::magick_bindings!(
    PixelWand,
    isSimilar <<= is_similar(other: &PixelWand, fuzz: f64) -> Result<()>
);
