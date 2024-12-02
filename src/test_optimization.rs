use crate::bindings::*;
use std::ffi::{c_uchar, CString};
use std::ptr::{null, null_mut};
use std::thread::panicking;
use std::time::SystemTime;

fn get_now() -> unix_time {
    let u_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let time = unix_time {
        sec: u_time.as_secs(),
        nsec: u_time.subsec_nanos() as u64,
    };
    time
}

fn c_uchar_to_bool(value: c_uchar) -> bool {
    if value > 0 {
        true
    } else {
        false
    }
}

#[derive(Debug)]
pub struct Settings {
    #[allow(dead_code)]
    pub code_coverage: bool,
    #[allow(dead_code)]
    pub early_flake_detection: EfDSettings,
    #[allow(dead_code)]
    pub flaky_test_retries_enabled: bool,
    #[allow(dead_code)]
    pub itr_enabled: bool,
    #[allow(dead_code)]
    pub require_git: bool,
    #[allow(dead_code)]
    pub tests_skipping: bool,
}

#[derive(Debug)]
pub struct EfDSettings {
    #[allow(dead_code)]
    pub enabled: bool,
    #[allow(dead_code)]
    pub slow_test_retries: EfdSlowTestRetriesSettings,
    #[allow(dead_code)]
    pub faulty_session_threshold: i32,
}

#[derive(Debug)]
pub struct EfdSlowTestRetriesSettings {
    #[allow(dead_code)]
    pub ten_s: i32,
    #[allow(dead_code)]
    pub thirty_s: i32,
    #[allow(dead_code)]
    pub five_m: i32,
    #[allow(dead_code)]
    pub five_s: i32,
}

#[derive(Debug)]
pub struct FlakyTestRetriesSettings {
    #[allow(dead_code)]
    pub retry_count: i32,
    #[allow(dead_code)]
    pub total_retry_count: i32,
}

#[derive(Debug)]
pub struct KnownTest {
    #[allow(dead_code)]
    pub module_name: String,
    #[allow(dead_code)]
    pub suite_name: String,
    #[allow(dead_code)]
    pub test_name: String,
}

/********************************
    Test session
*********************************/

pub struct TestSession;
impl TestSession {

    #[allow(dead_code)]
    pub fn init() -> Self {
        let language_name = CString::new("rust").unwrap();
        let runtime_name = CString::new("rustc").unwrap();
        let runtime_version = CString::new(rustc_version_runtime::version().to_string()).unwrap();
        unsafe {
            civisibility_initialize(language_name.into_raw(), runtime_name.into_raw(), runtime_version.into_raw(), null_mut(), null_mut(), &mut get_now());
        }
        Self {}
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_session_set_string_tag(key_cstring.into_raw(), value_cstring.into_raw()))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_session_set_number_tag(key_cstring.into_raw(), value))
        }
    }

    #[allow(dead_code)]
    pub fn set_error_info(&self, error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_session_set_error(error_type_cstring.into_raw(), error_message_cstring.into_raw(), error_stacktrace_cstring.into_raw()))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self, exit_code: i32) {
        unsafe {
            if panicking() {
                civisibility_shutdown(1, &mut get_now());
            } else {
                civisibility_shutdown(exit_code, &mut get_now());
            }
        }
    }

    #[allow(dead_code)]
    pub fn create_module(&self, name: impl AsRef<str>, framework_name: impl AsRef<str>, framework_version: impl AsRef<str>) -> TestModule {
        let module_name_cstring = CString::new(name.as_ref()).unwrap();
        let framework_name_cstring = CString::new(framework_name.as_ref()).unwrap();
        let framework_version_cstring = CString::new(framework_version.as_ref()).unwrap();
        unsafe {
            let module_id = civisibility_create_module(
                module_name_cstring.into_raw(),
                framework_name_cstring.into_raw(),
                framework_version_cstring.into_raw(),
                &mut get_now());

            TestModule { module_id }
        }
    }

    #[allow(dead_code)]
    pub fn get_settings(&self) -> Settings {
        unsafe {
            let settings_response = civisibility_get_settings();
            Settings {
                code_coverage: c_uchar_to_bool(settings_response.code_coverage),
                early_flake_detection: EfDSettings {
                    enabled: c_uchar_to_bool(settings_response.early_flake_detection.enabled),
                    slow_test_retries: EfdSlowTestRetriesSettings {
                        ten_s: settings_response.early_flake_detection.slow_test_retries.ten_s,
                        thirty_s: settings_response.early_flake_detection.slow_test_retries.thirty_s,
                        five_m: settings_response.early_flake_detection.slow_test_retries.five_m,
                        five_s: settings_response.early_flake_detection.slow_test_retries.five_s,
                    },
                    faulty_session_threshold: settings_response.early_flake_detection.faulty_session_threshold,
                },
                flaky_test_retries_enabled: c_uchar_to_bool(settings_response.flaky_test_retries_enabled),
                itr_enabled: c_uchar_to_bool(settings_response.itr_enabled),
                require_git: c_uchar_to_bool(settings_response.require_git),
                tests_skipping: c_uchar_to_bool(settings_response.tests_skipping),
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_flaky_test_retries_settings(&self) -> FlakyTestRetriesSettings {
        unsafe {
            let response = civisibility_get_flaky_test_retries_settings();
            FlakyTestRetriesSettings{
                retry_count: response.retry_count,
                total_retry_count: response.total_retry_count,
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_known_tests(&self) -> Vec<KnownTest> {
        unsafe {
            let mut length : ::std::os::raw::c_int = 0;
            let response = civisibility_get_known_tests(&mut length);
            let mut tests : Vec<KnownTest> = Vec::with_capacity(length as usize);
            for i in 0..length {
                let element = **response.offset(i as isize);
                tests.push(KnownTest {
                    module_name: CString::from_raw(element.module_name).to_str().unwrap().to_owned().to_owned(),
                    suite_name: CString::from_raw(element.suite_name).to_str().unwrap().to_owned().to_owned(),
                    test_name: CString::from_raw(element.test_name).to_str().unwrap().to_owned().to_owned(),
                });
            }
            tests
        }
    }
}

/********************************
    Test module
*********************************/

pub struct TestModule {
    pub module_id: u64,
}
impl TestModule {

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_module_set_string_tag(self.module_id, key_cstring.into_raw(), value_cstring.into_raw()))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_module_set_number_tag(self.module_id, key_cstring.into_raw(), value))
        }
    }

    #[allow(dead_code)]
    pub fn set_error_info(&self, error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_module_set_error(self.module_id, error_type_cstring.into_raw(), error_message_cstring.into_raw(), error_stacktrace_cstring.into_raw()))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        unsafe {
            c_uchar_to_bool(civisibility_close_module(self.module_id, &mut get_now()))
        }
    }

    #[allow(dead_code)]
    pub fn create_test_suite(&self, name: impl AsRef<str>) -> TestSuite {
        let test_suite_name_cstring = CString::new(name.as_ref()).unwrap();
        unsafe {
            let suite_id = civisibility_create_test_suite(
                self.module_id,
                test_suite_name_cstring.into_raw(),
                &mut get_now());
            TestSuite { suite_id, module_id: self.module_id }
        }
    }
}

/********************************
    Test suite
*********************************/

pub struct TestSuite {
    pub suite_id: u64,
    module_id: u64
}
impl TestSuite {
    #[allow(dead_code)]
    pub fn get_module(&self) -> TestModule {
        TestModule { module_id: self.module_id }
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_suite_set_string_tag(self.suite_id, key_cstring.into_raw(), value_cstring.into_raw()))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_suite_set_number_tag(self.suite_id, key_cstring.into_raw(), value))
        }
    }

    #[allow(dead_code)]
    pub fn set_error_info(&self, error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_suite_set_error(self.suite_id, error_type_cstring.into_raw(), error_message_cstring.into_raw(), error_stacktrace_cstring.into_raw()))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        unsafe {
            c_uchar_to_bool(civisibility_close_test_suite(self.suite_id, &mut get_now()))
        }
    }

    #[allow(dead_code)]
    pub fn create_test(&self, name: impl AsRef<str>) -> Test {
        let test_name_cstring = CString::new(name.as_ref()).unwrap();
        unsafe {
            let test_id = civisibility_create_test(
                self.suite_id,
                test_name_cstring.into_raw(),
                &mut get_now());
            Test { test_id, suite_id: self.suite_id, module_id: self.module_id }
        }
    }
}

/********************************
    Test
*********************************/

pub enum TestStatus {
    Pass = 0,
    Fail = 1,
    Skip = 2,
}

pub struct Test {
    pub test_id: u64,
    suite_id: u64,
    module_id: u64,
}
impl Test {
    #[allow(dead_code)]
    pub fn get_suite(&self) -> TestSuite {
        TestSuite { suite_id: self.suite_id, module_id: self.module_id }
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_test_set_string_tag(self.test_id, key_cstring.into_raw(), value_cstring.into_raw()))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_test_set_number_tag(self.test_id, key_cstring.into_raw(), value))
        }
    }

    #[allow(dead_code)]
    pub fn set_error_info(&self, error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
        unsafe {
            c_uchar_to_bool(civisibility_test_set_error(self.test_id, error_type_cstring.into_raw(), error_message_cstring.into_raw(), error_stacktrace_cstring.into_raw()))
        }
    }

    #[allow(dead_code)]
    pub fn set_test_source(
        &self,
        file: impl AsRef<str>,
        start_line: *const i32,
        end_line: *const i32,
    ) -> bool {
        let file_cstring = CString::new(file.as_ref()).unwrap();
        let mut c_start_line : *mut ::std::os::raw::c_int = null_mut();
        if start_line != null() {
            c_start_line = start_line.cast_mut();
        }
        let mut c_end_line : *mut ::std::os::raw::c_int = null_mut();
        if end_line != null() {
            c_end_line = end_line.cast_mut();
        }
        unsafe {
            c_uchar_to_bool(civisibility_test_set_test_source(
                self.test_id,
                file_cstring.into_raw(),
                c_start_line,
                c_end_line,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self, status: TestStatus, skip_reason: impl AsRef<str>) -> bool {
        let mut skip_reason_cstring:*mut ::std::os::raw::c_char = null_mut();
        let skip_reason_ref = skip_reason.as_ref();
        if skip_reason_ref != "" {
            skip_reason_cstring = CString::new(skip_reason_ref).unwrap().into_raw();
        }
        unsafe {
            c_uchar_to_bool(civisibility_close_test(self.test_id,
                                                    status as u8,
                                                    skip_reason_cstring,
                                                    &mut get_now()))
        }
    }
}
