use rand::Rng;
use jni::JNIEnv;

#[cfg(target_os="android")]
pub extern "system" fn Java_cn_com_sony_rustonandroid_MainActivity_doSth<'local>(
    mut env: JNIEnv<'local>, _: JClass<'local>, input: JString<'local>) -> jstring {


    let num = rand::thread_rng().gen_range(1000..9999);


    output.into_raw()
}
/*
https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
vi ~/.cargo/config
[target.aarch64-linux-android]
linker = "/root/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android29-clang"

[target.armv7-linux-androideabi]
linker = "/root/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi29-clang
"

[target.i686-linux-android]
linker = "/root/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android29-clang"

cargo build --target aarch64-linux-android --release
cargo build --target add armv7-linux-androideabi --release
cargo build --target i686-linux-android --release
*/
