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

get_set_enum!(
    DrawingWand,
    drawGetGravity,         drawSetGravity,         get_gravity,          set_gravity,          GravityType
    drawGetClipRule,        drawSetClipRule,        get_clip_rule,        set_clip_rule,        FillRule
    drawGetClipUnits,       drawSetClipUnits,       get_clip_units,       set_clip_units,       ClipPathUnits
    drawGetFillRule,        drawSetFillRule,        get_fill_rule,        set_fill_rule,        FillRule
    drawGetFontStyle,       drawSetFontStyle,       get_font_style,       set_font_style,       StyleType
    drawGetFontStretch,     drawSetFontStretch,     get_font_stretch,     set_font_stretch,     StretchType
    drawGetStrokeLineCap,   drawSetStrokeLineCap,   get_stroke_line_cap,  set_stroke_line_cap,  LineCap
    drawGetStrokeLineJoin,  drawSetStrokeLineJoin,  get_stroke_line_join, set_stroke_line_join, LineJoin
    drawGetStrokeAntialias, drawSetStrokeAntialias, get_stroke_antialias, set_stroke_antialias, MagickBooleanType
    drawGetTextAlignment,   drawSetTextAlignment,   get_text_alignment,   set_text_alignment,   AlignType
    drawGetTextAntialias,   drawSetTextAntialias,   get_text_antialias,   set_text_antialias,   MagickBooleanType
    drawGetTextDecoration,  drawSetTextDecoration,  get_text_decoration,  set_text_decoration,  DecorationType
    drawGetTextDirection,   drawSetTextDirection,   get_text_direction,   set_text_direction,   DirectionType
);

get_set_type!(
    DrawingWand,
    drawGetOpacity,              drawSetOpacity,              get_opacity,                set_opacity,                f64
    drawGetFillOpacity,          drawSetFillOpacity,          get_fill_opacity,           set_fill_opacity,           f64
    drawGetFontSize,             drawSetFontSize,             get_font_size,              set_font_size,              f64
    drawGetFontWeight,           drawSetFontWeight,           get_font_weight,            set_font_weight,            size_t
    drawGetStrokeDashOffset,     drawSetStrokeDashOffset,     get_stroke_dash_offset,     set_stroke_dash_offset,     f64
    drawGetStrokeMiterLimit,     drawSetStrokeMiterLimit,     get_stroke_miter_limit,     set_stroke_miter_limit,     size_t
    drawGetStrokeOpacity,        drawSetStrokeOpacity,        get_stroke_opacity,         set_stroke_opacity,         f64
    drawGetStrokeWidth,          drawSetStrokeWidth,          get_stroke_width,           set_stroke_width,           f64
    drawGetTextKerning,          drawSetTextKerning,          get_text_kerning,           set_text_kerning,           f64
    drawGetTextInterlineSpacing, drawSetTextInterlineSpacing, get_text_interline_spacing, set_text_interline_spacing, f64
    drawGetTextInterwordSpacing, drawSetTextInterwordSpacing, get_text_interword_spacing, set_text_interword_spacing, f64
);
