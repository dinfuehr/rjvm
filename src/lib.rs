#[warn(non_camel_case_types)]
type jint = i32;

#[warn(non_snake_case)]
pub extern "C" fn JNI_CreateJavaVM() -> jint {
    println!("reached my library!");
    panic!("YES!");
}
