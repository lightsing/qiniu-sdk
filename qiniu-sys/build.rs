use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = dst.join("include");
    println!("cargo:rerun-if-changed=qiniu-c-sdk");
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", include.display());
    println!("cargo:static=1");
    println!("cargo:rustc-flags=-l qiniu");
    fs::create_dir_all(include.join("qiniu")).unwrap();

    for header in [
        "base.h",
        "cdn.h",
        "conf.h",
        "fop.h",
        "http.h",
        "io.h",
        "macro.h",
        "qetag.h",
        "reader.h",
        "resumable_io.h",
        "rs.h",
        "tm.h",
    ]
    .iter()
    {
        fs::copy(
            format!("qiniu-c-sdk/qiniu/{}", header),
            include.join("qiniu").join(header),
        )
        .unwrap();
    }

    cc::Build::new()
        .flag("-fPIC")
        .include("qiniu-c-sdk/b64")
        .include("qiniu-c-sdk/cJSON")
        .include("qiniu-c-sdk/qiniu")
        .file("qiniu-c-sdk/b64/b64.c")
        .file("qiniu-c-sdk/b64/urlsafe_b64.c")
        .file("qiniu-c-sdk/cJSON/cJSON.c")
        .file("qiniu-c-sdk/qiniu/auth_mac.c")
        .file("qiniu-c-sdk/qiniu/base.c")
        .file("qiniu-c-sdk/qiniu/base_io.c")
        .file("qiniu-c-sdk/qiniu/cdn.c")
        .file("qiniu-c-sdk/qiniu/conf.c")
        .file("qiniu-c-sdk/qiniu/fop.c")
        .file("qiniu-c-sdk/qiniu/http.c")
        .file("qiniu-c-sdk/qiniu/io.c")
        //.file("qiniu-c-sdk/qiniu/macro.h")
        .file("qiniu-c-sdk/qiniu/qetag.c")
        .file("qiniu-c-sdk/qiniu/reader.c")
        .file("qiniu-c-sdk/qiniu/resumable_io.c")
        .file("qiniu-c-sdk/qiniu/rs.c")
        .file("qiniu-c-sdk/qiniu/tm.c")
        .flag("-lcurl")
        .flag("-lcrypto")
        .flag("-lssl")
        .flag("-lm")
        .compile("qiniu");

    let bindings = bindgen::Builder::default()
        .header("qiniu-c-sdk/qiniu/base.h")
        .header("qiniu-c-sdk/qiniu/cdn.h")
        .header("qiniu-c-sdk/qiniu/conf.h")
        .header("qiniu-c-sdk/qiniu/fop.h")
        .header("qiniu-c-sdk/qiniu/http.h")
        .header("qiniu-c-sdk/qiniu/io.h")
        .header("qiniu-c-sdk/qiniu/macro.h")
        .header("qiniu-c-sdk/qiniu/qetag.h")
        .header("qiniu-c-sdk/qiniu/reader.h")
        .header("qiniu-c-sdk/qiniu/resumable_io.h")
        .header("qiniu-c-sdk/qiniu/rs.h")
        .header("qiniu-c-sdk/qiniu/tm.h")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
