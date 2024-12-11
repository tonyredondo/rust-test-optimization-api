// bindings.rs

// Use standard raw types for FFI
use std::os::raw::{c_char, c_int, c_double, c_uchar, c_ulonglong};

// All structs must have a stable memory layout
// Use #[repr(C)] to ensure that their layout matches the C layout.

// In C: struct unix_time { unsigned long long sec; unsigned long long nsec; }
#[repr(C)]
pub struct unix_time {
    pub sec: c_ulonglong,
    pub nsec: c_ulonglong,
}

// In C: struct setting_early_flake_detection_slow_test_retries
#[repr(C)]
pub struct setting_early_flake_detection_slow_test_retries {
    // int ten_s;
    pub ten_s: c_int,
    // int thirty_s;
    pub thirty_s: c_int,
    // int five_m;
    pub five_m: c_int,
    // int five_s;
    pub five_s: c_int,
}

// In C: struct setting_early_flake_detection
#[repr(C)]
pub struct setting_early_flake_detection {
    // unsigned char enabled;
    pub enabled: c_uchar,
    // struct setting_early_flake_detection_slow_test_retries slow_test_retries;
    pub slow_test_retries: setting_early_flake_detection_slow_test_retries,
    // int faulty_session_threshold;
    pub faulty_session_threshold: c_int,
}

// In C: struct settings_response
#[repr(C)]
pub struct settings_response {
    // unsigned char code_coverage;
    pub code_coverage: c_uchar,
    // struct setting_early_flake_detection early_flake_detection;
    pub early_flake_detection: setting_early_flake_detection,
    // unsigned char flaky_test_retries_enabled;
    pub flaky_test_retries_enabled: c_uchar,
    // unsigned char itr_enabled;
    pub itr_enabled: c_uchar,
    // unsigned char require_git;
    pub require_git: c_uchar,
    // unsigned char tests_skipping;
    pub tests_skipping: c_uchar,
}

// In C: struct flaky_test_retries_settings
#[repr(C)]
pub struct flaky_test_retries_settings {
    // int retry_count;
    pub retry_count: c_int,
    // int total_retry_count;
    pub total_retry_count: c_int,
}

// In C: struct known_test
#[repr(C)]
pub struct known_test {
    // char* module_name;
    pub module_name: *mut c_char,
    // char* suite_name;
    pub suite_name: *mut c_char,
    // char* test_name;
    pub test_name: *mut c_char,
}

// In C: struct skippable_test
#[repr(C)]
pub struct skippable_test {
    // char* suite_name;
    pub suite_name: *mut c_char,
    // char* test_name;
    pub test_name: *mut c_char,
    // char* parameters;
    pub parameters: *mut c_char,
    // char* custom_configurations_json;
    pub custom_configurations_json: *mut c_char,
}

// In C: struct test_coverage_file
#[repr(C)]
pub struct test_coverage_file {
    // char* filename;
    pub filename: *mut c_char,
}

// In C: struct test_coverage
#[repr(C)]
pub struct test_coverage {
    // unsigned long long test_suite_id;
    pub test_suite_id: c_ulonglong,
    // unsigned long long span_id;
    pub span_id: c_ulonglong,
    // struct test_coverage_file* files;
    pub files: *mut test_coverage_file,
    // unsigned long long files_len;
    pub files_len: c_ulonglong,
}

// External functions

extern "C" {

    #[cfg(target_os = "windows")]
    pub fn _rt0_amd64_windows_lib();

    // civisibility_initialize
    // extern void civisibility_initialize(char* language, char* runtime_name, char* runtime_version, char* framework, char* framework_version, struct unix_time* unix_start_time);
    pub fn civisibility_initialize(
        language: *mut c_char,
        runtime_name: *mut c_char,
        runtime_version: *mut c_char,
        framework: *mut c_char,
        framework_version: *mut c_char,
        unix_start_time: *mut unix_time,
    );

    // civisibility_session_set_string_tag
    pub fn civisibility_session_set_string_tag(key: *mut c_char, value: *mut c_char) -> c_uchar;

    // civisibility_session_set_number_tag
    pub fn civisibility_session_set_number_tag(key: *mut c_char, value: c_double) -> c_uchar;

    // civisibility_session_set_error
    pub fn civisibility_session_set_error(error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> c_uchar;

    // civisibility_shutdown
    pub fn civisibility_shutdown(exit_code: c_int, unix_finish_time: *mut unix_time);

    // civisibility_create_module
    pub fn civisibility_create_module(
        name: *mut c_char,
        framework: *mut c_char,
        framework_version: *mut c_char,
        unix_start_time: *mut unix_time,
    ) -> c_ulonglong;

    // civisibility_module_set_string_tag
    pub fn civisibility_module_set_string_tag(module_id: c_ulonglong, key: *mut c_char, value: *mut c_char) -> c_uchar;

    // civisibility_module_set_number_tag
    pub fn civisibility_module_set_number_tag(module_id: c_ulonglong, key: *mut c_char, value: c_double) -> c_uchar;

    // civisibility_module_set_error
    pub fn civisibility_module_set_error(module_id: c_ulonglong, error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> c_uchar;

    // civisibility_close_module
    pub fn civisibility_close_module(module_id: c_ulonglong, unix_finish_time: *mut unix_time) -> c_uchar;

    // civisibility_create_test_suite
    pub fn civisibility_create_test_suite(module_id: c_ulonglong, name: *mut c_char, unix_start_time: *mut unix_time) -> c_ulonglong;

    // civisibility_suite_set_string_tag
    pub fn civisibility_suite_set_string_tag(suite_id: c_ulonglong, key: *mut c_char, value: *mut c_char) -> c_uchar;

    // civisibility_suite_set_number_tag
    pub fn civisibility_suite_set_number_tag(suite_id: c_ulonglong, key: *mut c_char, value: c_double) -> c_uchar;

    // civisibility_suite_set_error
    pub fn civisibility_suite_set_error(suite_id: c_ulonglong, error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> c_uchar;

    // civisibility_close_test_suite
    pub fn civisibility_close_test_suite(suite_id: c_ulonglong, unix_finish_time: *mut unix_time) -> c_uchar;

    // civisibility_create_test
    pub fn civisibility_create_test(suite_id: c_ulonglong, name: *mut c_char, unix_start_time: *mut unix_time) -> c_ulonglong;

    // civisibility_test_set_string_tag
    pub fn civisibility_test_set_string_tag(test_id: c_ulonglong, key: *mut c_char, value: *mut c_char) -> c_uchar;

    // civisibility_test_set_number_tag
    pub fn civisibility_test_set_number_tag(test_id: c_ulonglong, key: *mut c_char, value: c_double) -> c_uchar;

    // civisibility_test_set_error
    pub fn civisibility_test_set_error(test_id: c_ulonglong, error_type: *mut c_char, error_message: *mut c_char, error_stacktrace: *mut c_char) -> c_uchar;

    // civisibility_test_set_test_source
    pub fn civisibility_test_set_test_source(
        test_id: c_ulonglong,
        test_source_file: *mut c_char,
        test_source_start_line: *mut c_int,
        test_source_end_line: *mut c_int,
    ) -> c_uchar;

    // civisibility_close_test
    // status = 0: passed, 1: failed, 2: skipped
    pub fn civisibility_close_test(test_id: c_ulonglong, status: c_uchar, skip_reason: *mut c_char, unix_finish_time: *mut unix_time) -> c_uchar;

    // civisibility_get_settings
    pub fn civisibility_get_settings() -> settings_response;

    // civisibility_get_flaky_test_retries_settings
    pub fn civisibility_get_flaky_test_retries_settings() -> flaky_test_retries_settings;

    // civisibility_get_known_tests
    pub fn civisibility_get_known_tests(known_tests: *mut *mut known_test) -> c_int;

    // civisibility_get_skippable_tests
    pub fn civisibility_get_skippable_tests(skippable_tests: *mut *mut skippable_test) -> c_int;

    // civisibility_send_code_coverage_payload
    pub fn civisibility_send_code_coverage_payload(coverages: *mut test_coverage, coverages_length: c_int);
}
