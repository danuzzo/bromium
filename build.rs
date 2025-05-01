extern crate cc;

fn main() {
    // Windows libraries needed by the C++ code
    println!("cargo:rustc-link-lib=dylib=ole32");
    println!("cargo:rustc-link-lib=dylib=oleaut32");
    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=gdi32");
    println!("cargo:rustc-link-lib=dylib=uiautomationcore");

    // Compile C++ code
    cc::Build::new()
        .cpp(true)
        .file("src/cpp/stdafx.cpp")
        .file("src/cpp/UiTreeWalk.cpp")
        .file("src/cpp/exports.cpp")  // We'll create this file to export functions
        .include("src/cpp")
        .flag("/EHsc")       // C++ exception handling
        .flag("/std:c++17")  // Modern C++
        .flag("/D_UNICODE")  // Unicode support
        .flag("/DUNICODE")   // Unicode support
        // Required for proper symbol export
        .flag("/DWIN32")
        .flag("/D_WINDOWS")
        // Force static library output
        .static_flag(true)
        .compile("uixpath"); // Output library name

    // Tell cargo to invalidate the built crate whenever the C++ sources change
    println!("cargo:rerun-if-changed=src/cpp/UiTreeWalk.cpp");
    println!("cargo:rerun-if-changed=src/cpp/UiTreeWalk.h");
    println!("cargo:rerun-if-changed=src/cpp/stdafx.cpp");
    println!("cargo:rerun-if-changed=src/cpp/stdafx.h");
    println!("cargo:rerun-if-changed=src/cpp/ControlTypeId.h");
    println!("cargo:rerun-if-changed=src/cpp/exports.cpp");
}