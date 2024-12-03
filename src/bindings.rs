/* automatically generated by rust-bindgen 0.70.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct unix_time {
    pub sec: ::std::os::raw::c_ulonglong,
    pub nsec: ::std::os::raw::c_ulonglong,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct setting_early_flake_detection_slow_test_retries {
    pub ten_s: ::std::os::raw::c_int,
    pub thirty_s: ::std::os::raw::c_int,
    pub five_m: ::std::os::raw::c_int,
    pub five_s: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct setting_early_flake_detection {
    pub enabled: ::std::os::raw::c_uchar,
    pub slow_test_retries: setting_early_flake_detection_slow_test_retries,
    pub faulty_session_threshold: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct settings_response {
    pub code_coverage: ::std::os::raw::c_uchar,
    pub early_flake_detection: setting_early_flake_detection,
    pub flaky_test_retries_enabled: ::std::os::raw::c_uchar,
    pub itr_enabled: ::std::os::raw::c_uchar,
    pub require_git: ::std::os::raw::c_uchar,
    pub tests_skipping: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct flaky_test_retries_settings {
    pub retry_count: ::std::os::raw::c_int,
    pub total_retry_count: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct known_test {
    pub module_name: *mut ::std::os::raw::c_char,
    pub suite_name: *mut ::std::os::raw::c_char,
    pub test_name: *mut ::std::os::raw::c_char,
}
#[link(name="civisibility")]
extern "C" {
    pub fn civisibility_initialize(
        language: *mut ::std::os::raw::c_char,
        runtime_name: *mut ::std::os::raw::c_char,
        runtime_version: *mut ::std::os::raw::c_char,
        framework: *mut ::std::os::raw::c_char,
        framework_version: *mut ::std::os::raw::c_char,
        unix_start_time: *mut unix_time,
    );

    pub fn civisibility_session_set_string_tag(
        key: *mut ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_session_set_number_tag(
        key: *mut ::std::os::raw::c_char,
        value: f64,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_session_set_error(
        error_type: *mut ::std::os::raw::c_char,
        error_message: *mut ::std::os::raw::c_char,
        error_stacktrace: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_shutdown(
        exit_code: ::std::os::raw::c_int,
        unix_finish_time: *mut unix_time,
    );

    pub fn civisibility_create_module(
        name: *mut ::std::os::raw::c_char,
        framework: *mut ::std::os::raw::c_char,
        framework_version: *mut ::std::os::raw::c_char,
        unix_start_time: *mut unix_time,
    ) -> ::std::os::raw::c_ulonglong;

    pub fn civisibility_module_set_string_tag(
        module_id: ::std::os::raw::c_ulonglong,
        key: *mut ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_module_set_number_tag(
        module_id: ::std::os::raw::c_ulonglong,
        key: *mut ::std::os::raw::c_char,
        value: f64,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_module_set_error(
        module_id: ::std::os::raw::c_ulonglong,
        error_type: *mut ::std::os::raw::c_char,
        error_message: *mut ::std::os::raw::c_char,
        error_stacktrace: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_close_module(
        module_id: ::std::os::raw::c_ulonglong,
        unix_finish_time: *mut unix_time,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_create_test_suite(
        module_id: ::std::os::raw::c_ulonglong,
        name: *mut ::std::os::raw::c_char,
        unix_start_time: *mut unix_time,
    ) -> ::std::os::raw::c_ulonglong;

    pub fn civisibility_suite_set_string_tag(
        suite_id: ::std::os::raw::c_ulonglong,
        key: *mut ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_suite_set_number_tag(
        suite_id: ::std::os::raw::c_ulonglong,
        key: *mut ::std::os::raw::c_char,
        value: f64,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_suite_set_error(
        suite_id: ::std::os::raw::c_ulonglong,
        error_type: *mut ::std::os::raw::c_char,
        error_message: *mut ::std::os::raw::c_char,
        error_stacktrace: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_close_test_suite(
        suite_id: ::std::os::raw::c_ulonglong,
        unix_finish_time: *mut unix_time,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_create_test(
        suite_id: ::std::os::raw::c_ulonglong,
        name: *mut ::std::os::raw::c_char,
        unix_start_time: *mut unix_time,
    ) -> ::std::os::raw::c_ulonglong;

    pub fn civisibility_test_set_string_tag(
        test_id: ::std::os::raw::c_ulonglong,
        key: *mut ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_test_set_number_tag(
        test_id: ::std::os::raw::c_ulonglong,
        key: *mut ::std::os::raw::c_char,
        value: f64,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_test_set_error(
        test_id: ::std::os::raw::c_ulonglong,
        error_type: *mut ::std::os::raw::c_char,
        error_message: *mut ::std::os::raw::c_char,
        error_stacktrace: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_test_set_test_source(
        test_id: ::std::os::raw::c_ulonglong,
        test_source_file: *mut ::std::os::raw::c_char,
        test_source_start_line: *mut ::std::os::raw::c_int,
        test_source_end_line: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_close_test(
        test_id: ::std::os::raw::c_ulonglong,
        status: ::std::os::raw::c_uchar,
        skip_reason: *mut ::std::os::raw::c_char,
        unix_finish_time: *mut unix_time,
    ) -> ::std::os::raw::c_uchar;

    pub fn civisibility_get_settings() -> settings_response;

    pub fn civisibility_get_flaky_test_retries_settings() -> flaky_test_retries_settings;

    pub fn civisibility_get_known_tests(length: *mut ::std::os::raw::c_int)
                                        -> *mut *mut known_test;
}
