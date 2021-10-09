use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jdouble;
use jni_tools::jclass;
use jni_tools::Utils;

wand_wrapper!(DrawingWand);

#[jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/DrawingWandException")]
impl DrawingWand {
    fn drawAnnotation(&mut self, env: JNIEnv, _: JObject, x: jdouble, y: jdouble, text: JString) -> super::utils::Result<()> {
        let text = env.get_jstring(text)?;
        self.draw_annotation(x, y, &text)?;
        Ok(())
    }
}

string_get_set!(
    DrawingWand,
    drawGetFont,           drawSetFont,           get_font,            set_font
    drawGetFontFamily,     drawSetFontFamily,     get_font_family,     set_font_family
    drawGetVectorGraphics, drawSetVectorGraphics, get_vector_graphics, set_vector_graphics
    drawGetClipPath,       drawSetClipPath,       get_clip_path,       set_clip_path
    drawGetTextEncoding,   drawSetTextEncoding,   get_text_encoding,   set_text_encoding
);

magick_bindings::magick_bindings!(
    DrawingWand,
    nativeDrawGetGravity <<= get_gravity() -> GravityType, mut nativeDrawSetGravity <<= set_gravity(arg: GravityType)
);
