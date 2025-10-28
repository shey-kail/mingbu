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

    // 添加所有 .c 文件
    for entry in fs::read_dir("c_vendor/swisseph").unwrap() {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "c" {
                // 为 Windows 特别处理，修复 sweephe4.c 中的未声明变量问题
                let path_str = path.to_string_lossy().to_string();
                if target.contains("windows") && path_str.ends_with("sweephe4.c") {
                    // 为 Windows 环境修复有问题的文件
                    let original_content = std::fs::read_to_string(&path).unwrap();
                    
                    // 修复未声明的变量 'p' 问题
                    let fixed_content = original_content.replace(
                        "while ((p = strchr(d, \'/\')) != NULL) *p = \'\\\\\';",
                        "{ char *p; while ((p = strchr(d, \'/\')) != NULL) *p = \'\\\\\'; }"
                    );
                    
                    // 创建临时修复后的文件
                    let temp_path = std::env::var("OUT_DIR").unwrap() + "/sweephe4_fixed.c";
                    std::fs::write(&temp_path, fixed_content).unwrap();
                    builder.file(&temp_path);
                } else {
                    builder.file(&path);
                }
            }
        }
    }

    // 添加安全桩
    builder.file("c_vendor/swisseph/safe_stubs.c");

    builder.compile("swisseph");
}
