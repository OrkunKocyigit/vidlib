fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-lib=dxva2");
        println!("cargo:rustc-link-lib=evr");
        println!("cargo:rustc-link-lib=mf");
        println!("cargo:rustc-link-lib=mfplat");
        println!("cargo:rustc-link-lib=mfplay");
        println!("cargo:rustc-link-lib=mfreadwrite");
        println!("cargo:rustc-link-lib=mfuuid");
        println!("cargo:rustc-link-lib=bcrypt");
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=Secur32");
        println!("cargo:rustc-link-lib=Strmiids");
    }
    tauri_build::build()
}
