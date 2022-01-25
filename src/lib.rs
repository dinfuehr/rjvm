#[warn(non_camel_case_types)]
type jint = i32;

#[warn(non_snake_case)]
#[no_mangle]
pub extern "C" fn JNI_CreateJavaVM() -> jint {
    println!("in JNI_CreateJavaVM");
    panic!("YES!");
}

#[warn(non_snake_case)]
#[no_mangle]
pub extern "C" fn JNI_GetDefaultJavaVMInitArgs() -> jint {
    println!("in JNI_GetDefaultJavaVMInitArgs");
    panic!("YES!");
}

#[warn(non_snake_case)]
#[no_mangle]
pub extern "C" fn JNI_GetCreatedJavaVMs() -> jint {
    println!("in JNI_GetCreatedJavaVMs");
    panic!("YES!");
}
