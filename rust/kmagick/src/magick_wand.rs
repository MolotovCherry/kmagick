#![allow(non_snake_case)]

use std::convert::TryFrom;

use jni::{JNIEnv, objects::{JObject, JString, JValue}, sys::{jboolean, jbyteArray, jdouble, jdoubleArray, jint, jlong, jobject, jobjectArray, jstring}};

use jni_tools::{Handle, jclass, jname, Utils};

use crate::{
    DrawingWand,
    PixelWand
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
    fn newImage(&self, env: JNIEnv, _: JObject, columns: jlong, rows: jlong, pixel_wand: JObject) -> jni_tools::JNIResult<()> {
        let columns = usize::try_from(columns)?;
        let rows = usize::try_from(rows)?;

        let r_obj = env.get_handle::<PixelWand>(pixel_wand)?;

        Ok(self.new_image(columns, rows, &r_obj.instance)?)
    }

    fn setResourceLimit(&self, _: JNIEnv, _: JObject, resource: jint, limit: jlong) -> jni_tools::JNIResult<()> {
        let limit = u64::try_from(limit)?;

        let resource = magick_rust::ResourceType::try_from_int(resource)?;

        Ok(magick_rust::MagickWand::set_resource_limit(resource, limit)?)
    }

    fn setOption(&mut self, env: JNIEnv, _: JObject, key: JString, value: JString) -> jni_tools::JNIResult<()> {
        let key = env.get_jstring(key)?;
        let value = env.get_jstring(value)?;
        Ok(self.set_option(&*key, &*value)?)
    }

    fn annotateImage(&mut self, env: JNIEnv, _: JObject, drawing_wand: JObject, x: jdouble, y: jdouble, angle: jdouble, text: JString) -> jni_tools::JNIResult<()> {
        let r_obj = env.get_handle::<DrawingWand>(drawing_wand)?;
        let text = env.get_jstring(text)?;

        Ok(self.annotate_image(&r_obj.instance, x, y, angle, &*text)?)
    }

    fn addImage(&mut self, env: JNIEnv, _: JObject, other_wand: JObject) -> jni_tools::JNIResult<()> {
        let r_obj = env.get_handle::<MagickWand>(other_wand)?;
        Ok(self.add_image(&r_obj.instance)?)
    }

    fn appendAll(&mut self, env: JNIEnv, _: JObject, stack: jboolean) -> jni_tools::JNIResult<jobject> {
        let wand = self.append_all(stack != 0);

        Ok(new_from_wand!(env, wand, MagickWand).into_inner())
    }

    fn writeImages(&self, env: JNIEnv, _: JObject, path: JString, adjoin: jboolean) -> jni_tools::JNIResult<()> {
        let path = env.get_jstring(path)?;
        Ok(self.write_images(&*path, adjoin != 0)?)
    }

    fn readImageBlob(&self, env: JNIEnv, _: JObject, data: jbyteArray) -> jni_tools::JNIResult<()> {
        let bytes = env.convert_byte_array(data)?;
        Ok(self.read_image_blob(bytes)?)
    }

    fn pingImageBlob(&self, env: JNIEnv, _: JObject, data: jbyteArray) -> jni_tools::JNIResult<()> {
        let bytes = env.convert_byte_array(data)?;
        Ok(self.ping_image_blob(bytes)?)
    }

    fn compareImages(&self, env: JNIEnv, _: JObject, reference: JObject, metric: jint) -> jni_tools::JNIResult<jobject> {
        let reference = env.get_handle::<MagickWand>(reference)?;

        #[cfg(target_os="android")]
        let metric = u32::try_from(metric)?;

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

    fn compositeImage(&self, env: JNIEnv, _: JObject, reference: JObject, composition_operator: jint, clip_to_self: jboolean, x: jlong, y: jlong) -> jni_tools::JNIResult<()> {
        let reference = env.get_handle::<MagickWand>(reference)?;

        #[cfg(target_os="android")]
        let composition_operator = u32::try_from(composition_operator)?;

        self.compose_images(&reference.instance, composition_operator, clip_to_self != 0, x as isize, y as isize)?;
        Ok(())
    }

    fn clutImage(&self, env: JNIEnv, _: JObject, clut_wand: JObject, method: jint) -> jni_tools::JNIResult<()> {
        let clut_wand = env.get_handle::<MagickWand>(clut_wand)?;

        #[cfg(target_os="android")]
        let method = u32::try_from(method)?;

        self.clut_image(&clut_wand.instance, method)?;
        Ok(())
    }

    fn haldClutImage(&self, env: JNIEnv, _: JObject, clut_wand: JObject) -> jni_tools::JNIResult<()> {
        let clut_wand = env.get_handle::<MagickWand>(clut_wand)?;
        self.hald_clut_image(&clut_wand.instance)?;
        Ok(())
    }

    fn fx(&mut self, env: JNIEnv, _: JObject, expression: JString) -> jni_tools::JNIResult<jobject> {
        let expression = env.get_jstring(expression)?;
        let wand = self.instance.fx(&*expression);
        Ok(new_from_wand!(env, wand, MagickWand).into_inner())
    }

    fn setSize(&self, _: JNIEnv, _: JObject, columns: jlong, rows: jlong) -> jni_tools::JNIResult<()> {
        let columns = usize::try_from(columns)?;
        let rows = usize::try_from(rows)?;
        Ok(self.set_size(columns, rows)?)
    }

    fn levelImage(&self, _: JNIEnv, _: JObject, black_point: jdouble, gamma: jdouble, white_point: jdouble) -> jni_tools::JNIResult<()> {
        Ok(self.level_image(black_point, gamma, white_point)?)
    }

    fn extendImage(&self, _: JNIEnv, _: JObject, width: jlong, height: jlong, x: jlong, y: jlong) -> jni_tools::JNIResult<()> {
        let width = usize::try_from(width)?;
        let height = usize::try_from(height)?;
        let x = isize::try_from(x)?;
        let y = isize::try_from(y)?;

        Ok(self.extend_image(width, height, x, y)?)
    }

    fn profileImage(&self, env: JNIEnv, _: JObject, name: JString, profile: jbyteArray) -> jni_tools::JNIResult<()> {
        let name = env.get_jstring(name)?;
        let bytes: Vec<u8>;
        let profile = if !profile.is_null() {
            bytes = env.convert_byte_array(profile)?;
            Some(&*bytes)
        } else {
            None
        };

        Ok(self.profile_image(&*name, profile)?)
    }

    fn blurImage(&self, _: JNIEnv, _: JObject, radius: jdouble, sigma: jdouble) -> jni_tools::JNIResult<()> {
        Ok(self.blur_image(radius, sigma)?)
    }

    fn gaussianBlurImage(&self, _: JNIEnv, _: JObject, radius: jdouble, sigma: jdouble) -> jni_tools::JNIResult<()> {
        Ok(self.gaussian_blur_image(radius, sigma)?)
    }


    fn adaptiveResizeImage(&self, _: JNIEnv, _: JObject, width: jlong, height: jlong) -> jni_tools::JNIResult<()> {
        let width = usize::try_from(width)?;
        let height = usize::try_from(height)?;
        Ok(self.adaptive_resize_image(width, height)?)
    }

    fn rotateImage(&self, env: JNIEnv, _: JObject, background: JObject, degrees: jdouble) -> jni_tools::JNIResult<()> {
        let background = env.get_handle::<PixelWand>(background)?;
        Ok(self.rotate_image(&background.instance, degrees)?)
    }

    fn trimImage(&self, _: JNIEnv, _: JObject, fuzz: jdouble) -> jni_tools::JNIResult<()> {
        Ok(self.trim_image(fuzz)?)
    }

    fn resetImagePage(&self, env: JNIEnv, _: JObject, page_geometry: JString) -> jni_tools::JNIResult<()> {
        let page_geometry = env.get_jstring(page_geometry)?;
        Ok(self.reset_image_page(&*page_geometry)?)
    }

    fn getImageProperty(&self, env: JNIEnv, _: JObject, name: JString) -> jni_tools::JNIResult<jstring> {
        let name = env.get_jstring(name)?;
        let prop = self.get_image_property(&*name)?;
        Ok(env.new_string(&*prop)?.into_inner())
    }

    fn setImageProperty(&self, env: JNIEnv, _: JObject, name: JString, value: JString) -> jni_tools::JNIResult<()> {
        let name = env.get_jstring(name)?;
        let value = env.get_jstring(value)?;
        Ok(self.set_image_property(&*name, &*value)?)
    }

    fn getImagePixelColor(&self, env: JNIEnv, _: JObject, x: jlong, y: jlong) -> jni_tools::JNIResult<jobject> {
        let x = isize::try_from(x)?;
        let y = isize::try_from(y)?;
        let wand = self.get_image_pixel_color(x, y);

        if wand.is_some() {
            Ok(new_from_wand!(env, wand.unwrap(), PixelWand).into_inner())
        } else {
            Ok(std::ptr::null_mut())
        }
    }

    fn setSamplingFactors(&self, env: JNIEnv, _: JObject, sampling_factors: jdoubleArray) -> jni_tools::JNIResult<()> {
        let buf: &mut [f64] = &mut [];
        env.get_double_array_region(sampling_factors, 0, buf)?;
        Ok(self.set_sampling_factors(buf)?)
    }

    fn getImageHistogram(&self, env: JNIEnv) -> jni_tools::JNIResult<jobjectArray> {
        let wands = self.get_image_histogram();

        if wands.is_some() {
            let mut wands = wands.unwrap();
            let obj = env.new_object_array(
                i32::try_from(wands.len())?,
                "com/cherryleafroad/kmagick/PixelWand",
                JObject::null()
            )?;

            for num in 0..wands.len() {
                let wand = wands.remove(0);
                let wand = new_from_wand!(env, wand, PixelWand);
                env.set_object_array_element(obj, num as i32, wand)?;
            }

            Ok(obj)
        } else {
            Ok(std::ptr::null_mut())
        }
    }

    fn sharpenImage(&self, _: JNIEnv, _: JObject, radius: jdouble, sigma: jdouble) -> jni_tools::JNIResult<()> {
        Ok(self.sharpen_image(radius, sigma)?)
    }

    fn setBackgroundColor(&self, env: JNIEnv, _: JObject, pixel_wand: JObject) -> jni_tools::JNIResult<()> {
        let pixel_wand = env.get_handle::<PixelWand>(pixel_wand)?;
        Ok(self.set_background_color(&pixel_wand.instance)?)
    }

    fn setImageBackgroundColor(&self, env: JNIEnv, _: JObject, pixel_wand: JObject) -> jni_tools::JNIResult<()> {
        let pixel_wand = env.get_handle::<PixelWand>(pixel_wand)?;
        Ok(self.set_image_background_color(&pixel_wand.instance)?)
    }

    fn getImageResolution(&self, env: JNIEnv) -> jni_tools::JNIResult<jobject> {
        let (hor_res, vert_res) = self.get_image_resolution()?;
        let x = JValue::Double(hor_res);
        let y = JValue::Double(vert_res);

        let cls = env.find_class("com/cherryleafroad/kmagick/Resolution")?;
        let mid = env.get_method_id(cls, "<init>", "(DD)V")?;
        Ok(env.new_object_unchecked(cls, mid, &[x, y])?.into_inner())
    }

    fn setImageResolution(
        &self,
        _: JNIEnv,
        _: JObject,
        x_resolution: jdouble,
        y_resolution: jdouble,
    ) -> jni_tools::JNIResult<()> {
        Ok(self.set_image_resolution(x_resolution, y_resolution)?)
    }

    fn setResolution(&self, _: JNIEnv, _: JObject, x_resolution: jdouble, y_resolution: jdouble) -> jni_tools::JNIResult<()> {
        Ok(self.set_resolution(x_resolution, y_resolution)?)
    }

    fn sepiaToneImage(&self, _: JNIEnv, _: JObject, threshold: jdouble) -> jni_tools::JNIResult<()> {
        Ok(self.sepia_tone_image(threshold)?)
    }

    fn exportImagePixels(
        &self,
        env: JNIEnv,
        _: JObject,
        x: jlong,
        y: jlong,
        width: jlong,
        height: jlong,
        map: JString,
    ) -> jni_tools::JNIResult<jbyteArray> {
        let x = isize::try_from(x)?;
        let y = isize::try_from(y)?;
        let width = usize::try_from(width)?;
        let height = usize::try_from(height)?;
        let map = env.get_jstring(map)?;

        let export = self.export_image_pixels(x, y, width, height, &*map);
        if export.is_some() {
            // reinterpret [u8] as [i8] for java -> why is there no function for this in jni?
            let slice = &*export.unwrap();
            let export = bytemuck::cast_slice::<u8, i8>(slice);
            let size = i32::try_from(export.len())?;
            let bytes = env.new_byte_array(size)?;
            env.set_byte_array_region(bytes, 0, export)?;
            Ok(bytes)
        } else {
            Ok(std::ptr::null_mut())
        }
    }

    #[jname(name="magickResizeImage")]
    fn resizeImage(&self, _: JNIEnv, _: JObject, width: jlong, height: jlong, filter: jint) -> jni_tools::JNIResult<()> {
        let width = usize::try_from(width)?;
        let height = usize::try_from(height)?;

        #[cfg(target_os="android")]
        let filter = u32::try_from(filter)?;

        self.resize_image(width, height, filter);
        Ok(())
    }

    fn cropImage(
        &self,
        _: JNIEnv,
        _: JObject,
        width: jlong,
        height: jlong,
        x: jlong,
        y: jlong,
    ) -> jni_tools::JNIResult<()> {
        let width = usize::try_from(width)?;
        let height = usize::try_from(height)?;
        let x = isize::try_from(x)?;
        let y = isize::try_from(y)?;

        Ok(self.crop_image(width, height, x, y)?)
    }

    fn sampleImage(&self, _: JNIEnv, _: JObject, width: jlong, height: jlong) -> jni_tools::JNIResult<()> {
        let width = usize::try_from(width)?;
        let height = usize::try_from(height)?;
        Ok(self.sample_image(width, height)?)
    }

    #[cfg(not(target_os="android"))]
    fn resampleImage(
        &self,
        _: JNIEnv,
        _: JObject,
        x_resolution: jdouble,
        y_resolution: jdouble,
        filter: jint
    ) {
        self.resample_image(x_resolution, y_resolution, filter);
    }

    #[cfg(target_os="android")]
    fn resampleImage(
        &self,
        _: JNIEnv,
        _: JObject,
        x_resolution: jdouble,
        y_resolution: jdouble,
        filter: jint
    ) -> jni_tools::JNIResult<()> {
        let filter = u32::try_from(filter)?;

        self.resample_image(x_resolution, y_resolution, filter);
        Ok(())
    }

    fn liquidRescaleImage(&self, _: JNIEnv, _: JObject, width: jlong, height: jlong, delta_x: jdouble, rigidity: jdouble) -> jni_tools::JNIResult<()> {
        let width = usize::try_from(width)?;
        let height = usize::try_from(height)?;
        Ok(self.liquid_rescale_image(width, height, delta_x, rigidity)?)
    }

    fn implode(&self, _: JNIEnv, _: JObject, amount: jdouble, method: jint) -> jni_tools::JNIResult<()> {
        #[cfg(target_os="android")]
        let method = u32::try_from(method)?;

        Ok(self.instance.implode(amount, method)?)
    }

    fn fit(&self, _: JNIEnv, _: JObject, width: jlong, height: jlong) -> jni_tools::JNIResult<()> {
        let width = usize::try_from(width)?;
        let height = usize::try_from(height)?;
        self.instance.fit(width, height);
        Ok(())
    }

    fn requiresOrientation(&self) -> jboolean {
        self.requires_orientation() as jboolean
    }

    fn autoOrient(&self) -> jboolean {
        self.auto_orient() as jboolean
    }

    fn writeImageBlob(&self, env: JNIEnv, _: JObject, format: JString) -> jni_tools::JNIResult<jbyteArray> {
        let format = env.get_jstring(format)?;
        let bytes = self.write_image_blob(&*format)?;

        let length = i32::try_from(bytes.len())?;
        let j_byte_obj = env.new_byte_array(length)?;
        // there really should be a method on this in the jni ...
        let j_bytes = bytemuck::cast_slice::<u8, i8>(&*bytes);
        env.set_byte_array_region(j_byte_obj, 0, j_bytes)?;
        Ok(j_byte_obj)
    }

    fn writeImagesBlob(&self, env: JNIEnv, _: JObject, format: JString) -> jni_tools::JNIResult<jbyteArray> {
        let format = env.get_jstring(format)?;
        let bytes = self.write_images_blob(&*format)?;

        let length = i32::try_from(bytes.len())?;
        let j_byte_obj = env.new_byte_array(length)?;
        // there really should be a method on this in the jni ...
        let j_bytes = bytemuck::cast_slice::<u8, i8>(&*bytes);
        env.set_byte_array_region(j_byte_obj, 0, j_bytes)?;
        Ok(j_byte_obj)
    }

    fn getImageWidth(&self) -> jni_tools::JNIResult<jlong> {
        Ok(i64::try_from(self.get_image_width())?)
    }

    /// Retrieve the height of the image.
    fn getImageHeight(&self) -> jni_tools::JNIResult<jlong> {
        Ok(i64::try_from(self.get_image_height())?)
    }

    /// Retrieve the page geometry (width, height, x offset, y offset) of the image.
    fn getImagePage(&self, env: JNIEnv) -> jni_tools::JNIResult<jobject> {
        let (width, height, x, y) = self.get_image_page();
        let width = i64::try_from(width)?;
        let height = i64::try_from(height)?;
        let x = i64::try_from(x)?;
        let y = i64::try_from(y)?;

        let cls = env.find_class("com/cherryleafroad/kmagick/PageGeometry")?;
        let width = JValue::Long(width);
        let height = JValue::Long(height);
        let x = JValue::Long(x);
        let y = JValue::Long(y);

        let mid = env.get_method_id(cls, "<init>", "(JJJJ)V")?;

        Ok(env.new_object_unchecked(cls, mid, &[width, height, x, y])?.into_inner())
    }

    // mutations! section
    fn transformImageColorspace(&self, _: JNIEnv, _: JObject, colorspace: jint) -> jni_tools::JNIResult<()> {
        #[cfg(target_os="android")]
        let colorspace = u32::try_from(colorspace)?;

        Ok(self.transform_image_colorspace(colorspace)?)
    }

    fn setImageAlpha(&self, _: JNIEnv, _: JObject, alpha: jdouble) -> jni_tools::JNIResult<()> {
        Ok(self.set_image_alpha(alpha)?)
    }

    fn modulateImage(
        &self,
        _: JNIEnv,
        _: JObject,
        brightness: jdouble,
        saturation: jdouble,
        hue: jdouble
    ) -> jni_tools::JNIResult<()> {
        Ok(self.modulate_image(brightness, saturation, hue)?)
    }

    fn setImageAlphaChannel(&self, _: JNIEnv, _: JObject, alpha_channel: jint) -> jni_tools::JNIResult<()> {
        #[cfg(target_os="android")]
        let alpha_channel = u32::try_from(alpha_channel)?;

        Ok(self.set_image_alpha_channel(alpha_channel)?)
    }

    fn quantizeImage(
        &self,
        _: JNIEnv,
        _: JObject,
        number_of_colors: jlong,
        colorspace: jint,
        tree_depth: jlong,
        dither_method: jint,
        measure_error: jboolean
    ) -> jni_tools::JNIResult<()> {
        let number_of_colors = usize::try_from(number_of_colors)?;
        let tree_depth = usize::try_from(tree_depth)?;

        cfg_if::cfg_if! {
            if #[cfg(target_os="android")] {
                let colorspace = u32::try_from(colorspace)?;
                let dither_method = u32::try_from(dither_method)?;
                let measure_error = u32::try_from(measure_error)?;
            } else {
                let measure_error = i32::try_from(measure_error)?;
            }
        }

        Ok(self.quantize_image(number_of_colors, colorspace, tree_depth, dither_method, measure_error)?)
    }

    fn quantizeImages(
        &self,
        _: JNIEnv,
        _: JObject,
        number_of_colors: jlong,
        colorspace: jint,
        tree_depth: jlong,
        dither_method: jint,
        measure_error: jboolean
    ) -> jni_tools::JNIResult<()> {
        let number_of_colors = usize::try_from(number_of_colors)?;
        let tree_depth = usize::try_from(tree_depth)?;

        cfg_if::cfg_if! {
            if #[cfg(target_os="android")] {
                let colorspace = u32::try_from(colorspace)?;
                let dither_method = u32::try_from(dither_method)?;
                let measure_error = u32::try_from(measure_error)?;
            } else {
                let measure_error = i32::try_from(measure_error)?;
            }
        }

        Ok(self.quantize_images(number_of_colors, colorspace, tree_depth, dither_method, measure_error)?)
    }

    fn uniqueImageColors(&self) -> jni_tools::JNIResult<()> {
        Ok(self.unique_image_colors()?)
    }

    fn kmeans(
        &self,
        _: JNIEnv,
        _: JObject,
        number_colors: jlong,
        max_iterations: jlong,
        tolerance: jdouble
    ) -> jni_tools::JNIResult<()> {
        let number_colors = usize::try_from(number_colors)?;
        let max_iterations = usize::try_from(max_iterations)?;

        Ok(self.instance.kmeans(number_colors, max_iterations, tolerance)?)
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
    writeImage, write_image
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

simple_call!(
    MagickWand,
    flipImage,   flip_image
    negateImage, negate_image
    flopImage,   flop_image
);
