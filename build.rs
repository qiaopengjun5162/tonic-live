// build.rs 是 Rust 的构建脚本，在项目编译前自动执行
// 主要用于生成代码、编译依赖或设置条件编译标志

/// This function is responsible for configuring and compiling protobuf files using the `tonic_build` crate.
///
/// # Parameters
///
/// - `out_dir`: A string representing the output directory where the generated Rust code will be placed.
/// - `proto_files`: A slice of strings containing the paths to the protobuf files to be compiled.
/// - `include_dirs`: A slice of strings containing the directories to search for imported protobuf files.
///
/// # Return
///
/// This function returns a `Result` indicating whether the compilation was successful.
/// If successful, it returns `Ok(())`. If an error occurs during compilation, it returns `Err(tonic_build::Error)`.
fn main() {
    // tonic_build 是 tonic 框架提供的工具，用于处理 Protocol Buffers 文件
    tonic_build::configure()
        .type_attribute(
            ".",
            "#[derive(Hash, Eq, serde::Serialize, serde::Deserialize)]",
        )
        // 设置生成的 Rust 代码的输出目录
        // 这里指定生成的代码将放在 src/pb 目录下
        .out_dir("src/pb")
        // 编译 Protocol Buffers 文件
        // 第一个参数 &["./abi.proto"] 指定要编译的 .proto 文件路径
        // 第二个参数 &["./"] 指定在哪些目录下查找 .proto 文件
        // compile_protos() 是新版本推荐使用的方法，替代了旧的 compile()
        .compile_protos(&["./abi.proto"], &["./"])
        // unwrap() 用于错误处理
        // 如果编译过程出现错误，这里会导致构建失败并显示错误信息
        .unwrap();

    println!("cargo:rerun-if-changed=abi.proto"); // 如果 abi.proto 文件发生变化，重新运行构建脚本
    println!("cargo:rerun-if-changed=build.rs"); // 如果 build.rs 文件发生变化，重新运行构建脚本
}
