//! Windows only: embed the app icon and version info (VERSIONINFO) into `fisitexto.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `WORDCRAFT_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/fisilab/fisitexto.ico");
    println!("cargo:rerun-if-env-changed=WORDCRAFT_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/fisilab/fisitexto.ico")
        .set("ProductName", "FisiTexto")
        .set("FileDescription", "FisiTexto — Processador de texto (Suíte FisiLab)")
        .set("CompanyName", "FisiLab — LARFIC/UFRGS")
        .set(
            "LegalCopyright",
            "Baseado no WordCraft (© ArtCraft Team e contribuidores). Modificações © 2026 FisiLab — LARFIC/UFRGS. MIT OR Apache-2.0.",
        )
        .set("OriginalFilename", "fisitexto.exe")
        .set("InternalName", "fisitexto");
    if let Err(e) = res.compile() {
        if std::env::var_os("WORDCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=fisitexto.exe built without icon/version resources: {e}");
    }
}
