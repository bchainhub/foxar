fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=advapi32");
    }
    vergen::EmitBuilder::builder().build_timestamp().git_sha(true).emit().unwrap();
}
