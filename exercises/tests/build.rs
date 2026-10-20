//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
 // 正确格式：cargo:rustc-env=变量名=值
 println!("cargo:rustc-env=TEST_FOO={}", timestamp);

 // 对应 tests8：启用名为 "pass" 的 feature
 // 正确格式：cargo:rustc-cfg=feature="特性名"
 println!("cargo:rustc-cfg=feature=\"pass\"");
 
 // 让 Cargo 知道如果这个文件变了，就重新运行它
//  println!("cargo:rerun-if-changed=build.rs");
}
