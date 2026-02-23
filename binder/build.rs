fn main() {
    // libgit2-sys requires Windows system libraries that it doesn't always link properly
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=advapi32");
    }
}
