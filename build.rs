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
    if target.contains("windows") {
        // 使用编译器标志来修复问题
        // 在有问题的代码区域添加变量声明的预处理器修复
        // 让我们通过在编译前修改宏定义来绕过有问题的代码路径
        builder.define("_WIN64", None);
    }

    // 添加所有 .c 文件
    for entry in fs::read_dir("c_vendor/swisseph").unwrap() {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "c" {
                // 为 Windows 特别处理，先读取文件内容，修复问题后再编译
                let path_str = path.to_string_lossy().to_string();
                if target.contains("windows") && path_str.ends_with("sweephe4.c") {
                    // 为 Windows 环境复制并修复有问题的文件
                    let original_content = std::fs::read_to_string(&path).unwrap();
                    let fixed_content = original_content.replace(
                        "# if MSDOS\n  while ((p = strchr(d, '/')) != NULL) *p = '\\\\';",
                        "# if MSDOS\n  char *p;  /* Fixed: Declare variable p */\n  while ((p = strchr(d, '/')) != NULL) *p = '\\\\';"
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