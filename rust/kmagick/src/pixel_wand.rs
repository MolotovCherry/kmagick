use jni::{JNIEnv, objects::{JObject, JValue}, sys::{jboolean, jdouble, jobject}};
use jni_tools::jclass;

wand_wrapper!(PixelWand);

#[jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/PixelWandException")]
impl PixelWand {
    fn isSimilar(&self, env: JNIEnv, _: JObject, other: JObject, fuzz: jdouble) -> crate::utils::Result<jboolean> {
        use jni_tools::Handle;

        let r_obj = env.get_handle::<PixelWand>(other)?;

        let other = &r_obj.instance;

        // why is the api even like this? it really should use a bool.. whatever..
        let res = match self.is_similar(other, fuzz) {
            Ok(_) => true,
            Err(_) => false
        };

        Ok(res as jboolean)
    }

    fn pixelGetHSL(&self, env:jni::JNIEnv) -> crate::utils::Result<jobject> {
        let res = self.get_hsl();

        let cls = env.find_class("com/cherryleafroad/kmagick/HSL")?;
        let mid = env.get_method_id(cls, "<init>", "(DDD)V")?;

        let j_hue = JValue::Double(res.hue);
        let j_saturation = JValue::Double(res.saturation);
        let j_lightness = JValue::Double(res.lightness);

        let n_obj = env.new_object_unchecked(cls, mid, &[j_hue, j_saturation, j_lightness])?;

        Ok(n_obj.into_inner())
    }

    fn pixelSetHSL(&self, env: JNIEnv, _: JObject, hsl: JObject) -> crate::utils::Result<()> {
        let hue = env.get_field(hsl, "hue", "D")?.d()?;
        let saturation = env.get_field(hsl, "saturation", "D")?.d()?;
        let lightness = env.get_field(hsl, "lightness", "D")?.d()?;

        let hsl = magick_rust::HSL {
            hue, lightness, saturation
        };

        self.set_hsl(&hsl);
        Ok(())
    }
}

get_set_string!(
    PixelWand,
    pixelGetColorAsString, pixelSetColor, get_color_as_string, set_color
);

get_string!(
    PixelWand,
    pixelGetColorAsNormalizedString, get_color_as_normalized_string
);

get_set_sized!(
    PixelWand,
    pixelGetColorCount, pixelSetColorCount, get_color_count, set_color_count
);

get_set_double!(
    PixelWand,
    pixelGetFuzz,    pixelSetFuzz,    get_fuzz,    set_fuzz

    // color set types
    pixelGetAlpha,   pixelSetAlpha,   get_alpha,   set_alpha
    pixelGetBlack,   pixelSetBlack,   get_black,   set_black
    pixelGetBlue,    pixelSetBlue,    get_blue,    set_blue
    pixelGetCyan,    pixelSetCyan,    get_cyan,    set_cyan
    pixelGetGreen,   pixelSetGreen,   get_green,   set_green
    pixelGetMagenta, pixelSetMagenta, get_magenta, set_magenta
    pixelGetRed,     pixelSetRed,     get_red,     set_red
    pixelGetYellow,  pixelSetYellow,  get_yellow,  set_yellow
);

// the following are Quantum types == MagickFloatType == f32 == jfloat
get_set_float!(
    PixelWand,
    pixelGetIndex,          pixelSetIndex,          get_index,           set_index

    // quantum types , Quantum == MagickFloatType == f32 == jfloat
    pixelGetAlphaQuantum,   pixelSetAlphaQuantum,   get_alpha_quantum,   set_alpha_quantum
    pixelGetBlackQuantum,   pixelSetBlackQuantum,   get_black_quantum,   set_black_quantum
    pixelGetBlueQuantum,    pixelSetBlueQuantum,    get_blue_quantum,    set_blue_quantum
    pixelGetCyanQuantum,    pixelSetCyanQuantum,    get_cyan_quantum,    set_cyan_quantum
    pixelGetGreenQuantum,   pixelSetGreenQuantum,   get_green_quantum,   set_green_quantum
    pixelGetMagentaQuantum, pixelSetMagentaQuantum, get_magenta_quantum, set_magenta_quantum
    pixelGetRedQuantum,     pixelSetRedQuantum,     get_red_quantum,     set_red_quantum
    pixelGetYellowQuantum,  pixelSetYellowQuantum,  get_yellow_quantum,  set_yellow_quantum
);
