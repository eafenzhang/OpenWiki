fn main() {
    // Pre-compile the macOS OCR Swift helper so end users don't need swiftc
    // (i.e. don't need Xcode Command Line Tools) to use OCR at runtime.
    // The resulting binary is bundled as a Tauri resource — see tauri.conf.json.
    #[cfg(target_os = "macos")]
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let swift_src = std::path::Path::new(&manifest_dir).join("resources/openwiki_ocr.swift");
        let swift_bin = std::path::Path::new(&manifest_dir).join("resources/openwiki_ocr_bin");

        println!("cargo:rerun-if-changed={}", swift_src.display());

        if swift_src.exists() {
            let status = std::process::Command::new("/usr/bin/swiftc")
                .args([
                    "-O",
                    swift_src.to_str().unwrap(),
                    "-o",
                    swift_bin.to_str().unwrap(),
                ])
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("cargo:warning=Pre-compiled OCR Swift binary -> {}", swift_bin.display());
                }
                Ok(s) => {
                    panic!("swiftc exited with status {} while compiling OCR helper", s);
                }
                Err(e) => {
                    panic!("Failed to invoke swiftc for OCR helper: {}. Is Xcode Command Line Tools installed on the build machine?", e);
                }
            }
        } else {
            panic!("OCR Swift source not found at {}", swift_src.display());
        }
    }

    tauri_build::build()
}
