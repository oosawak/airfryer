fn main() {
  // Android 15+ devices may use 16 KB memory pages. NDK r27 needs
  // explicit linker flags to align native libraries for those devices.
  let target = std::env::var("TARGET").unwrap_or_default();
  if target.contains("android") {
    println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=16384");
    println!("cargo:rustc-link-arg=-Wl,-z,common-page-size=16384");
  }
  tauri_build::build()
}
