fn main() {
    println!("cargo:rustc-env=LD_LIBRARY_PATH=/opt/vc/lib");
    println!("cargo:rustc-link-lib=bcm_host");
    println!("cargo:rustc-link-lib=mmal_core");
    println!("cargo:rustc-link-lib=mmal_util");
    println!("cargo:rustc-link-lib=mmal_vc_client");
    println!("cargo:rustc-link-lib=vcos");
    println!("cargo:rustc-link-search=native=/opt/vc/lib");
}
