// build.rs
use std::env;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=c_vendor/swisseph/");
    
    let target = env::var("TARGET").expect("TARGET not set");
    
    let mut builder = cc::Build::new();
    builder
        .include("c_vendor/swisseph")
        .warnings(false)
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable");

    // 为 Windows 平台解决 C 代码中未声明变量的问题
    // 通过预定义有问题的宏来修复未声明的变量 p
    if target.contains("windows") {
        // 在 Windows 上，定义一个宏来修复 sweephe4.c 中的未声明变量问题
        builder.define("MSDOS", "0");  // 确保 MSDOS 相关代码路径不会被启用
    }

    // 添加所有 .c 文件
    for entry in fs::read_dir("c_vendor/swisseph").unwrap() {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "c" {
                builder.file(&path);
            }
        }
    }

    // 添加安全桩
    builder.file("c_vendor/swisseph/safe_stubs.c");

    builder.compile("swisseph");
}