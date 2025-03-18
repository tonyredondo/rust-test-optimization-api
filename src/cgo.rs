
#[cfg_attr(all(windows, target_env = "msvc"), link(name = "legacy_stdio_definitions", kind = "dylib"))]
extern "C" {
    #[cfg(target_os = "windows")]
    pub fn _rt0_amd64_windows_lib();
}