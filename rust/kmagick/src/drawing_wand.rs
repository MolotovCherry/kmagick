use jni::objects::{JObject, JString};
use jni::sys::jdouble;
use jni_tools::{jclass, Utils};
use jni::JNIEnv;
use super::utils::Result;

wand_wrapper!(DrawingWand);

#[jclass(pkg="com/cherryleafroad/kmagick", exc="com/cherryleafroad/kmagick/DrawingWandException")]
impl DrawingWand {
    fn drawAnnotation(&mut self, env: JNIEnv, _: JObject, x: jdouble, y: jdouble, text: JString) -> Result<()> {
        let text = env.get_jstring(text)?;
        self.draw_annotation(x, y, &text)?;
        Ok(())
    }
}
