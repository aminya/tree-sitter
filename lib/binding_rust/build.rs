use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-env-changed=TREE_SITTER_STATIC_ANALYSIS");
    if env::var("TREE_SITTER_STATIC_ANALYSIS").is_ok() {
        if let (Some(clang_path), Some(scan_build_path)) = (which("clang"), which("scan-build")) {
            let clang_path = clang_path.to_str().unwrap();
            let scan_build_path = scan_build_path.to_str().unwrap();
            env::set_var(
                "CC",
                &format!(
                    "{} -analyze-headers --use-analyzer={} cc",
                    scan_build_path, clang_path
                ),
            );
        }
    }

    let src_path = Path::new("src");
    for entry in fs::read_dir(&src_path).unwrap() {
        let entry = entry.unwrap();
        let path = src_path.join(entry.file_name());
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    let mut config = cc::Build::new();
    config
        .flag_if_supported("-std=c17")
        .flag_if_supported("-Wno-unused-parameter")
        .include(src_path)
        .include("include")
        .file(src_path.join("lib.c"))
        .compile("tree-sitter");

    if env::var("OPT_LEVEL").unwrap_or("3".to_string()) == "3" {
        let compiler = config.get_compiler();
        if compiler.is_like_msvc() {
            config.opt_level_str("/O2");
            config
                .flag("/Ob3") // aggressive inline
                .flag("/GF") // duplicate string elimination
                .flag("/GR-") // do not generate runtime type information
                .flag("/Gw") // optimize global data
                .flag("/GA") // optimize thread-local storage
                .flag("/DNDEBUG"); // turn off debug asserts

        // lld-link is invoked by Rust in the end, so these do not work
        // .flag("/LTCG") // link time code generation
        // .flag("/GL") // whole program optimization
        } else if compiler.is_like_clang() || compiler.is_like_gnu() {
            config.opt_level_str("fast");
        }
    }
}

fn which(exe_name: impl AsRef<Path>) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}
