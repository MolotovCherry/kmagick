use proc_macro2::TokenStream;

pub(super) fn validate_types(ty: Vec<&TokenStream>, is_impl: bool, is_static: bool) -> syn::Result<()> {
    let allowed_types_second_param = &[
        "jobject", "jclass", "JObject", "JClass"
    ];
    let allowed_types = &[
        "jobject", "jclass", "jthrowable", "jstring", "jarray", "jbooleanArray",
        "jbyteArray", "jcharArray", "jshortArray", "jintArray", "jlongArray",
        "jfloatArray", "jdoubleArray", "jobjectArray", "jweak", "jint", "jlong",
        "jbyte", "jboolean", "jchar", "jshort", "jfloat", "jdouble", "jsize",
        "jfieldID", "jmethodID",
        "JByteBuffer", "JClass", "JFieldID", "JList", "JMap", "JMethodID",
        "JObject", "JStaticFieldID", "JStaticMethodID", "JString", "JThrowable",
        "JValue"
    ];

    for (i, ty) in ty.iter().enumerate() {
        let ty = ty.to_string();
        let ty_l = ty.to_lowercase();

        if i == 0 {
            if ty != "JNIEnv" {
                return Err(syn::Error::new_spanned(ty, "Param must be JNIEnv"));
            }
        } else if i == 1 {
            if !allowed_types_second_param.contains(&&*ty) {
                return Err(syn::Error::new_spanned(ty, "Param must be JObject or JClass (non-impl fn's only)"));
            }

            if is_impl && ty_l == "jclass" && is_static {
                return Err(syn::Error::new_spanned(ty, "JClass is not allowed on impl methods"));
            }
        } else {
            if !allowed_types.contains(&&*ty) {
                return Err(syn::Error::new_spanned(ty, "Param must be a j-type"));
            }
        }
    }

    Ok(())
}
