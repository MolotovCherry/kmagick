use jni::{JNIEnv, objects::{JObject, JString, JValue}, sys::{jboolean, jbyteArray, jdouble, jint, jlong, jobject}};
use jni_tools::{jclass, jname, Handle, Utils};
use std::convert::TryFrom;
use crate::{
    PixelWand,
    DrawingWand
};

wand_wrapper!(MagickWand);

magick_enum_int_conversion!(
    enum ResourceType {
        Undefined,
        Area,
        Disk,
        File,
        Height,
        Map,
        Memory,
        Thread,
        Throttle,
        Time,
        Width,
        ListLength,
    }
);

#[jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad.kmagick/MagickWandException")]
impl MagickWand {
    fn newImage(&self, env: JNIEnv, _: JObject, columns: jlong, rows: jlong, pixel_wand: JObject) -> crate::utils::Result<()> {
        let columns = usize::try_from(columns)?;
        let rows = usize::try_from(rows)?;

        let r_obj = env.get_handle::<PixelWand>(pixel_wand)?;

        Ok(self.new_image(columns, rows, &r_obj.instance)?)
    }

    #[jname(name="magickSetResourceLimit")]
    fn setResourceLimit(&self, _: JNIEnv, _: JObject, resource: jint, limit: jlong) -> crate::utils::Result<()> {
        let limit = u64::try_from(limit)?;

        let resource = magick_rust::ResourceType::try_from_int(resource)?;

        Ok(magick_rust::MagickWand::set_resource_limit(resource, limit)?)
    }

    fn setOption(&mut self, env: JNIEnv, _: JObject, key: JString, value: JString) -> crate::utils::Result<()> {
        let key = env.get_jstring(key)?;
        let value = env.get_jstring(value)?;
        Ok(self.set_option(&*key, &*value)?)
    }

    fn annotateImage(&mut self, env: JNIEnv, _: JObject, drawing_wand: JObject, x: jdouble, y: jdouble, angle: jdouble, text: JString) -> crate::utils::Result<()> {
        let r_obj = env.get_handle::<DrawingWand>(drawing_wand)?;
        let text = env.get_jstring(text)?;

        Ok(self.annotate_image(&r_obj.instance, x, y, angle, &*text)?)
    }

    fn addImage(&mut self, env: JNIEnv, _: JObject, other_wand: JObject) -> crate::utils::Result<()> {
        let r_obj = env.get_handle::<MagickWand>(other_wand)?;
        Ok(self.add_image(&r_obj.instance)?)
    }

    fn appendAll(&mut self, env: JNIEnv, _: JObject, stack: jboolean) -> crate::utils::Result<jobject> {
        let wand = self.append_all(stack != 0);

        Ok(new_from_wand!(env, wand, MagickWand).into_inner())
    }

    fn writeImages(&self, env: JNIEnv, _: JObject, path: JString, adjoin: jboolean) -> crate::utils::Result<()> {
        let path = env.get_jstring(path)?;
        Ok(self.write_images(&*path, adjoin != 0)?)
    }

    fn readImageBlob(&self, env: JNIEnv, _: JObject, data: jbyteArray) -> crate::utils::Result<()> {
        let bytes = env.convert_byte_array(data)?;
        Ok(self.read_image_blob(bytes)?)
    }

    fn pingImageBlob(&self, env: JNIEnv, _: JObject, data: jbyteArray) -> crate::utils::Result<()> {
        let bytes = env.convert_byte_array(data)?;
        Ok(self.ping_image_blob(bytes)?)
    }

    fn compareImages(&self, env: JNIEnv, _: JObject, reference: JObject, metric: jint) -> crate::utils::Result<jobject> {
        let reference = env.get_handle::<MagickWand>(reference)?;

        let (distortion, r_diffImage) = self.compare_images(&reference.instance, metric);

        let mut diffImage = None;
        if diffImage.is_some() {
            let wand = r_diffImage.unwrap();
            diffImage = Some(new_from_wand!(env, wand, MagickWand));
        }

        let cls = env.find_class("com/cherryleafroad/kmagick/Comparison")?;
        let j_distortion = JValue::Double(distortion);
        let j_diffImage = JValue::Object(
            if diffImage.is_some() {
                diffImage.unwrap()
            } else {
                JObject::null()
            }
        );
        let mid = env.get_method_id(cls, "<init>", "(DLcom/cherryleafroad/kmagick/MagickWand;)V")?;

        let comparison = env.new_object_unchecked(cls, mid, &[j_distortion, j_diffImage])?;

        Ok(comparison.into_inner())
    }

    fn composeImages(&self, env: JNIEnv, _: JObject, reference: JObject, composition_operator: jint, clip_to_self: jboolean, x: jlong, y: jlong) -> crate::utils::Result<()> {
        let reference = env.get_handle::<MagickWand>(reference)?;
        self.compose_images(&reference.instance, composition_operator, clip_to_self != 0, x as isize, y as isize)?;
        Ok(())
    }

    fn clutImage(&self, env: JNIEnv, _: JObject, clut_wand: JObject, method: jint) -> crate::utils::Result<()> {
        let clut_wand = env.get_handle::<MagickWand>(clut_wand)?;
        self.clut_image(&clut_wand.instance, method)?;
        Ok(())
    }

    fn haldClutImage(&self, env: JNIEnv, _: JObject, clut_wand: JObject) -> crate::utils::Result<()> {
        let clut_wand = env.get_handle::<MagickWand>(clut_wand)?;
        self.hald_clut_image(&clut_wand.instance)?;
        Ok(())
    }

    fn fxImage(&mut self, env: JNIEnv, _: JObject, expression: JString) -> crate::utils::Result<jobject> {
        let expression = env.get_jstring(expression)?;
        let wand = self.fx(&*expression);
        Ok(new_from_wand!(env, wand, MagickWand).into_inner())
    }
}

get_sized!(
    MagickWand,
    magickGetImageColors, get_image_colors, usize //size_t
);

set_string!(
    MagickWand,
    labelImage, label_image
    readImage, read_image
    pingImage, ping_image
);

get_set_string!(
    MagickWand,
    magickGetFilename,      magickSetFilename,      get_filename,       set_filename
    magickGetFont,          magickSetFont,          get_font,           set_font
    magickGetFormat,        magickSetFormat,        get_format,         set_format
    magickGetImageFilename, magickSetImageFilename, get_image_filename, set_image_filename
    magickGetImageFormat,   magickSetImageFormat,   get_image_format,   set_image_format
);

get_set_enum_result!(
    MagickWand,
    magickGetColorspace,             magickSetColorspace,             get_colorspace,               set_colorspace,               ColorspaceType
    magickGetCompression,            magickSetCompression,            get_compression,              set_compression,              CompressionType
    magickGetGravity,                magickSetGravity,                get_gravity,                  set_gravity,                  GravityType
    magickGetImageColorspace,        magickSetImageColorspace,        get_image_colorspace,         set_image_colorspace,         ColorspaceType
    magickGetImageCompose,           magickSetImageCompose,           get_image_compose,            set_image_compose,            CompositeOperator
    magickGetImageCompression,       magickSetImageCompression,       get_image_compression,        set_image_compression,        CompressionType
    magickGetImageDispose,           magickSetImageDispose,           get_image_dispose,            set_image_dispose,            DisposeType
    magickGetImageEndian,            magickSetImageEndian,            get_image_endian,             set_image_endian,             EndianType
    magickGetImageGravity,           magickSetImageGravity,           get_image_gravity,            set_image_gravity,            GravityType
    magickGetImageInterlaceScheme,   magickSetImageInterlaceScheme,   get_image_interlace_scheme,   set_image_interlace_scheme,   InterlaceType
    magickGetImageInterpolateMethod, magickSetImageInterpolateMethod, get_image_interpolate_method, set_image_interpolate_method, PixelInterpolateMethod
    magickGetImageOrientation,       magickSetImageOrientation,       get_image_orientation,        set_image_orientation,        OrientationType
    magickGetImageRenderingIntent,   magickSetImageRenderingIntent,   get_image_rendering_intent,   set_image_rendering_intent,   RenderingIntent
    magickGetImageType,              magickSetImageType,              get_image_type,               set_image_type,               ImageType
    magickGetImageUnits,             magickSetImageUnits,             get_image_units,              set_image_units,              ResolutionType
    magickGetInterlaceScheme,        magickSetInterlaceScheme,        get_interlace_scheme,         set_interlace_scheme,         InterlaceType
    magickGetInterpolateMethod,      magickSetInterpolateMethod,      get_interpolate_method,       set_interpolate_method,       PixelInterpolateMethod
    magickGetOrientation,            magickSetOrientation,            get_orientation,              set_orientation,              OrientationType
    magickGetType,                   magickSetType,                   get_type,                     set_type,                     ImageType
);

get_set_sized_result!(
    MagickWand,
    magickGetCompressionQuality,      magickSetCompressionQuality,      get_compression_quality,       set_compression_quality,       usize //size_t
    magickGetImageCompressionQuality, magickSetImageCompressionQuality, get_image_compression_quality, set_image_compression_quality, usize //size_t
    magickGetImageDelay,              magickSetImageDelay,              get_image_delay,               set_image_delay,               usize //size_t
    magickGetImageDepth,              magickSetImageDepth,              get_image_depth,               set_image_depth,               usize //size_t
    magickGetImageIterations,         magickSetImageIterations,         get_image_iterations,          set_image_iterations,          usize //size_t
    magickGetImageScene,              magickSetImageScene,              get_image_scene,               set_image_scene,               usize //size_t
    magickGetIteratorIndex,           magickSetIteratorIndex,           get_iterator_index,            set_iterator_index,            isize //ssize_t
);
